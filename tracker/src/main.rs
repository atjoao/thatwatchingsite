use error_chain::error_chain;
use regex::Regex;
use xml::{reader::XmlEvent, EventReader};
use std::io::BufReader;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}
fn main() -> Result<()>{
    // rss feed testing
    let resp = reqwest::blocking::get("https://nyaa.si/?page=rss&c=1_2")?;
    let body = BufReader::new(resp);
    //xml parsing
    let parser = EventReader::new(body);
    let mut current_element: Option<String> = None;


    for e in parser {
        match e {

            Ok(XmlEvent::StartElement { name, .. }) => {
                current_element = None;

                if name.local_name == "title" {
                    current_element = Some(name.local_name.clone());
                }

            }

            Ok(XmlEvent::Characters(value)) => {
                if let Some(ref element_name) = current_element {
                    println!("value of {}, {}", element_name, value);
                    clean_title(value)
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

    Ok(())
}

fn clean_title(title: String) {
    // note find a better way to do this later 
    let pattern = Regex::new(r"(?m)\[(.*?)\]\s*(.*?)(?:-|s(\d{2})e(\d{2}))?\s*(\d{3,4}p)").unwrap();
    
    let results: Vec<&str> = pattern
        .captures_iter(&title)
        .flat_map(|cap| {
            cap.iter()
                .filter_map(|m| m.map(|m| m.as_str()))
                .filter(|s| !s.is_empty())
                .collect::<Vec<&str>>()
        })
        .collect();

    for result in results {
        println!("{}", result);
    }
}