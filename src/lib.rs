use chrono::{Duration, NaiveDateTime};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    string,
};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Default)]
pub struct Nfo {
    pub general: GeneralInformation,
    pub media: MediaInformation,
    pub description: Option<String>,
}

#[derive(Debug, Default)]
pub struct GeneralInformation {
    pub title: Option<String>,
    pub author: Option<String>,
    pub read_by: Option<String>,
    pub copyright: Option<String>,
    pub genre: Option<String>,
    pub publisher: Option<String>,
    pub duration: Option<Duration>,
    pub chapters: Option<u32>,
    pub unabridged: Option<bool>,
}

#[derive(Debug, Default)]
pub struct MediaInformation{
    pub source: Source,
    pub encode: Encoded,
    pub chapter_adjust: Option<String>,
    pub chapter_rename: Option<String>,
    pub ripper: Option<String>,
    pub id_tagging: Option<String>
}

#[derive(Debug, Default)]
pub struct Source{
    pub format: Option<String>,
    pub sample_rate: Option<String>,
    pub channels: Option<u16>,
    pub bitrate: Option<String>,
}

#[derive(Debug, Default)]
pub struct Encoded{
    pub lossless_encode: Option<bool>,
    pub codec: Option<String>,
    pub sample_rate: Option<String>,
    pub channels: Option<u16>,
    pub bitrate: Option<String>,
}



impl Nfo {
    pub fn new<T: Into<PathBuf>>(path: T) -> Option<Self> {
        let path = path.into();
        let input = match File::open(path) {
            Ok(file) => file,
            Err(_) => return None,
        };
        let mut general = GeneralInformation::default();
        let mut media = MediaInformation::default();

        let mut section = Section::None;
        let mut skip = false;
        let mut description = String::new();
        for line in BufReader::new(input).lines() {
            if skip {
                skip = false;
                continue;
            }
            if let Ok(line) = line {
                //{for word in line.unicode_words(){print!("{}|", word);}println!("");}

                let data: Vec<_> = line.unicode_words().collect();
                if data.len() < 2 {
                    continue;
                }
                if data[0] == "General" && data[1] == "Information" {
                    section = Section::GeneralInformation;
                    skip = true;
                    continue;
                } else if data[0] == "Media" && data[1] == "Information" {
                    section = Section::MediaInformation;
                    skip = true;
                    continue;
                } else if data[0] == "Book" && data[1] == "Description" {
                    section = Section::BookDescription;
                    skip = true;
                    continue;
                }

                // this is were the parsing really starts
                // we match the section and the first word of each line
                match section {
                    Section::GeneralInformation => match data[0] {
                        "Title" => {
                            if general.title.is_none() {
                                general.title = Some(aux_string(&data[1..]))
                            }
                        }
                        "Author" => {
                            if general.author.is_none() {
                                general.author = Some(aux_string(&data[1..]))
                            }
                        }
                        "Read" => {
                            if data[1] != "By" {
                                continue;
                            }
                            if general.read_by.is_none() {
                                general.read_by = Some(aux_string(&data[2..]))
                            }
                        }
                        "Copyright" => {
                            if general.copyright.is_none() {
                                general.copyright = Some(aux_string(&data[1..]))
                            }
                        }
                        "Genre" => {
                            if general.genre.is_none() {
                                general.genre = Some(aux_string(&data[1..]))
                            }
                        }
                        "Publisher" => {
                            if general.publisher.is_none() {
                                general.publisher = Some(aux_string(&data[1..]))
                            }
                        }

                        "Duration" => {
                            if general.duration.is_none() {
                                let mut duration = Duration::zero();
                                if data.len() > 1{
                                    match data[1].parse::<i64>(){
                                        Ok(hours) => duration = Duration::hours(hours),
                                        Err(_) => continue,
                                    }
                                        
                                        
                                }

                                if data.len() > 3{
                                    match data[3].parse::<i64>(){
                                        Ok(min) => duration = duration + Duration::minutes(min),
                                        Err(_) => continue,
                                    }
                                }

                                if data.len() > 5{
                                    match data[5].parse::<i64>(){
                                        Ok(sec) => duration = duration + Duration::seconds(sec),
                                        Err(_) => continue,
                                    }
                                        
                                }
                                if !duration.is_zero(){
                                    general.duration = Some(duration)
                                }
                            }
                        }
                        "Chapters" => {
                            if general.chapters.is_none() {
                                if let Ok(chap) = aux_string(&data[1..]).parse::<u32>() {
                                    general.chapters = Some(chap)
                                } else {
                                    println!("failed to parse chapters")
                                }
                            }
                        }
                        "Unabridged" => {
                            if general.unabridged.is_none() {
                                let b  = data[1] == "Yes";
                                if b {
                                    general.unabridged = Some(true)
                                } else if data[1] == "No" {
                                    general.unabridged = Some(false)
                                }
                                
                            }
                        }
                        _ => {}
                    },
                    Section::MediaInformation => {
                        match data[0]{
                            "Source" => {
                                match data[1]{
                                    "Format" => {
                                        if media.source.format.is_none(){
                                            media.source.format = Some(aux_string(&data[2..]))
                                        }
                                    },
                                    "Sample" => {
                                        if data[2] != "Rate" {
                                            continue;
                                        }
                                        if media.source.sample_rate.is_none(){
                                            media.source.sample_rate = Some(aux_string(&data[3..]))
                                        }
                                    },
                                    "Channels" => {
                                        if media.source.channels.is_none(){
                                            if let Ok(chan) = aux_string(&data[2..]).parse::<u16>() {
                                                media.source.channels = Some(chan)
                                            } else {
                                                println!("failed to parse chapters")
                                            }
                                        }
                                    },
                                    "Bitrate" => {
                                        if media.source.bitrate.is_none(){
                                            media.source.bitrate = Some(aux_string(&data[2..]))
                                        }
                                    },
                                    _ => {}
                                }
                            },
                            "Lossless" => {
                                let b  = data[1] == "Yes";
                                if b {
                                     media.encode.lossless_encode= Some(true)
                                } else if data[1] == "No" {
                                    general.unabridged = Some(false)
                                }
                            },
                            "Encoded" => {
                                match data[1]{
                                    "Codec" => {
                                        if media.encode.codec.is_none(){
                                            media.encode.codec = Some(aux_string(&data[2..]))
                                        }
                                    },
                                    "Sample" => {
                                        if data[2] != "Rate" {
                                            continue;
                                        }
                                        media.encode.sample_rate = Some(aux_string(&data[3..]))
                                    },
                                    "Channels" => {
                                        if media.encode.channels.is_none(){
                                            if let Ok(chan) = aux_string(&data[2..]).parse::<u16>() {
                                                media.encode.channels = Some(chan)
                                            } else {
                                                println!("failed to parse chanales")
                                            }
                                        }
                                    },
                                    "Bitrate" => {
                                        if media.encode.bitrate.is_none(){
                                            media.encode.bitrate = Some(aux_string(&data[2..]))
                                        }
                                    },
                                    _ => {},
                                }
                            },
                            "Chapter" => {
                                if data[1] == "Adjust"{
                                    if media.chapter_adjust.is_none(){
                                        media.chapter_adjust = Some(aux_string(&data[2..]))
                                    }
                                } else if data[1] == "Rename" {
                                    if media.chapter_rename.is_none(){
                                        media.chapter_rename = Some(aux_string(&data[2..]))
                                    }
                                }
                            }
                            _ => {},
                        }
                    }
                    Section::BookDescription => {
                        description.push_str(&line);
                        description.push_str("/n");
                    }
                    Section::None => {}
                }
            }
        }
        
        let description = if description.is_empty(){None} else {Some(description)};
        Some(Self {
            general,
            media,
            description
        })
    }
}

enum Section {
    GeneralInformation,
    MediaInformation,
    BookDescription,
    None,
}

fn aux_string(input: &[&str]) -> String {
    let mut acume = String::new();
    for word in input {
        acume.push_str(word);
        acume.push_str(" ");
    }
    acume.pop();
    acume
}

pub struct Information {
    name: String,
    value: Value,
}

pub enum Value {
    String(String),
    Bool(bool),
    Unit(String, u32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nfo = Nfo::new(r"V:\Local-Books\War of the Posers - Bad Guys Series, Book 4 by Eric Ugland\War of the Posers Bad Guys Series, Book 4.nfo").unwrap();
        println!("{:?}", nfo);
    }
}
