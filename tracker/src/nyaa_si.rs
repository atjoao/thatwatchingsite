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

#[warn(dead_code)]
pub fn fetch() -> Result<Vec<Anime>> {
    // RSS feed testing
    let resp = reqwest::blocking::get("https://nyaa.si/?page=rss&c=1_2")?;
    let body = BufReader::new(resp);

    // XML parsing
    let parser = EventReader::new(body);
    let mut current_element: Option<String> = None;

    let mut animes: Vec<Anime> = vec![];

    // Temporary storage for building an Anime struct
    let mut anime_temp = Anime {
        group: String::new(),
        name: String::new(),
        quality: String::new(),
        link: String::new(),
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
                        anime_temp.link = value.clone();
                        animes.push(anime_temp.clone());
                        anime_temp.link.clear();
                    } else if element_name == "title" {
                        clean_title(&value, &mut anime_temp);
                    }
                }
            }

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

fn clean_title(title: &str, anime_temp: &mut Anime) {
    // Regex pattern to extract group, name, quality, etc.
    let pattern = Regex::new(r"(?m)\[(.*?)\]\s*(.*?)(?:-|s(\d{2})e(\d{2}))?\s*(\d{3,4}p)").unwrap();

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
// TODO if episode detected remove from name and add to struct 
        anime_temp.name = results[2].to_string();
    }
}