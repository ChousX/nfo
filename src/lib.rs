use std::{path::{Path, PathBuf}, fs::File, io::{BufReader, BufRead}};

use unicode_segmentation::UnicodeSegmentation;

pub struct Nfo{
    pub general: Vec<Information>,
    pub media: Vec<Information>,
    pub description: Option<String>,
}

pub struct GeneralInformation{
    pub title: Option<String>,
    pub author: Option<String>,
    pub read_by: Option<String>,
    pub copyright: Option<String>, //date should bring in chrono
    pub genre: Option<String>,
    pub publisher: Option<String>,
    pub duration: Option<String>, //duration
    pub chapters: Option<u32>
}

impl Nfo{
    pub fn new<T: Into<PathBuf>>(path: T) -> Option<Self>{
        let path = path.into();
        let input = match File::open(path){
            Ok(file) => file,
            Err(_) => return None,
        };
        let mut section = Section::None;
        let mut skip = false;
        for line in BufReader::new(input).lines(){
            if skip{
                skip = false;
                continue;
            }
            if let Ok(line) = line{
                {for word in line.unicode_words(){print!("{}|", word);}println!("");}
                let data: Vec<_> = line.unicode_words().collect();
                if data.len() < 2 {continue;}
                if  data[0] == "General" && data[1] == "Information"{
                    section = Section::GeneralInformation;
                    skip = true;
                    continue;
                } else
                if data[0] == "Media" && data[1] == "Information"{
                    section = Section::MediaInformation;
                    skip = true;
                    continue;
                } else 

                if data[0] == "Book" && data[1] == "Description"{
                    section = Section::BookDescription;
                    skip = true;
                    continue;
                }

                match section{
                    Section::GeneralInformation => {
                        match data[0]{
                            "Title" => {},
                            "Author" => {},
                            "Read" => {
                                if data[1] != "By"{
                                    continue;
                                }
                            },
                            "Copyright" => {},
                            "Audiobook" => {},
                            "Genre" => {},
                            "Publisher" => {},
                            "Duration" => {},
                            "Chapters" => {},
                            _ => {}
                        }
                    },
                    Section::MediaInformation => {},
                    Section::BookDescription => {},
                    Section::None => {

                    }
                }

            }
        }
        Some(Self{
            general: Vec::new(),
            media: Vec::new(),
            description: None
        })
    }
}



enum Section{
    GeneralInformation,
    MediaInformation,
    BookDescription,
    None,
}
pub struct Information{
    name: String,
    value: Value,
}

pub enum Value{
    String(String),
    Bool(bool),
    Unit(String, u32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works(){
        let nfo = Nfo::new(r"V:\Local-Books\Uprooted by Naomi Novik\Uprooted.nfo").unwrap();
        
    }
}
