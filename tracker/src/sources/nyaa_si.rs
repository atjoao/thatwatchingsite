// this needs to be refactored someday
use crate::structs::Anime;

use error_chain::error_chain;
use regex::Regex;
use xml::{reader::XmlEvent, EventReader};
use std::{io::BufReader, vec};
use reqwest;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

pub fn fetch(name: &str) -> Result<Vec<Anime>> {
    // RSS feed testing
    let resp = reqwest::blocking::get(&format!("https://nyaa.si/?page=rss&c=1_2&q={}", name))?;
    let body = BufReader::new(resp);

    // XML parsing
    let parser = EventReader::new(body);
    let mut current_element: Option<String> = None;

    let mut animes: Vec<Anime> = vec![];

    // Temporary storage to build an Anime struct
    let mut anime_temp = Anime {
        group: String::new(),
        name: String::new(),
        quality: String::new(),
        link: String::new(),
        
        batch: false,
        complete: false,

        episode: String::new(),
        season: String::new(),
    };


    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                current_element = None;
                if name.local_name == "title" || name.local_name == "link" {
                    current_element = Some(name.local_name.clone());
                }
            }

            Ok(XmlEvent::Characters(value)) => {
                if let Some(ref element_name) = current_element {
                    if element_name == "link" {
                        if anime_temp.group.is_empty() {
                            continue;
                        }

                        if !value.ends_with(".torrent") {
                            anime_temp.link.clear();
                        } else if !anime_temp.name.is_empty() {
                            anime_temp.link = value.clone();
                            animes.push(anime_temp.clone());
                            anime_temp.link.clear();
                        }
                    } else if element_name == "title" {
                        clean_title(&value, &mut anime_temp);                        
                    }
                }
            }

            // i probably don't need this
            Ok(XmlEvent::EndElement { .. }) => {
                current_element = None;
            }

            Err(e) => {
                eprintln!("Error: {e}");
                break;
            }

            _ => {}
        }
    }

    Ok(animes)
}

// there should be a better way to do this
fn clean_title(title: &str, anime_temp: &mut Anime) {
    // Regex pattern to extract group, name, quality, etc.

    // stupid thign
    anime_temp.batch = false;
    anime_temp.complete = false;
    anime_temp.season.clear();
    anime_temp.episode.clear();
    anime_temp.group.clear();

    let pattern = Regex::new(r"(?m)\[(.*?)\]\s*(.*?)\s*(\d{3,4}p)").unwrap();
    let know_ep_and_season = Regex::new(r"[Ss](\d{1,2})[Ee](\d{1,2})|-\s*(\d{1,2})").unwrap();
    let know_season = Regex::new(r"(?m)(?:[Ss](\d{1,2})|[Ss]eason\s*(\d{1,2}))").unwrap();

    let results: Vec<&str> = pattern
        .captures_iter(title)
        .flat_map(|cap| {
            cap.iter()
                .filter_map(|m| m.map(|m| m.as_str()))
                .filter(|s| !s.is_empty())
                .collect::<Vec<&str>>()
        })
        .collect();

    if results.len() >= 3 {
        anime_temp.group = results[1].to_string();
        anime_temp.quality = results[3].to_string();
        // TODO if episode detected remove from name and add to struct (how will i do this idk)
        anime_temp.name = results[2].to_string();

        if title.contains("batch"){
            // since its an batch i can just download it
            anime_temp.batch = true;

            if let Some(caps) = know_season.captures(&anime_temp.name) {
                // Check if either capture group 1 or 2 is present, and set `anime_temp.season`
                if let Some(season) = caps.get(1).or_else(|| caps.get(2)) {
                    anime_temp.season = season.as_str().to_string();
                }
            }

            return;
        }

        if anime_temp.name.len() <= 1 {
            anime_temp.group.clear();
            anime_temp.quality.clear();
            anime_temp.name.clear();
            return;
        } else {
            // get ep and season here
            let results: Vec<&str> = know_ep_and_season
                .captures_iter(&anime_temp.name)
                .flat_map(|cap| {
                    cap.iter()
                        .filter_map(|m| m.map(|m| m.as_str()))
                        .filter(|s| !s.is_empty())
                        .collect::<Vec<&str>>()
                })
                .collect();

            if results.len() == 0 {
                // since it contains all i can just download it
                anime_temp.complete = true;

                if let Some(caps) = know_season.captures(&anime_temp.name) {
                    // Check if either capture group 1 or 2 is present, and set `anime_temp.season`
                    if let Some(season) = caps.get(1).or_else(|| caps.get(2)) {
                        anime_temp.season = season.as_str().to_string();
                    }
                }
            }

            if results.len() == 2 {
                // append to the later on
                anime_temp.episode = results[1].to_string();
            } else if results.len() == 3 {
                anime_temp.episode = results[2].to_string();
                anime_temp.season = results[1].to_string();
            }
        }
    }
}