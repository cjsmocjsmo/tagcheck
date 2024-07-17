use crate::genrecodes;
use id3::{Tag, TagLike};
use walkdir::WalkDir;
// use encoding_rs::{WINDOWS_1252, UTF_8};
// use std::string::FromUtf8Error;

pub fn find_media(dir_path: &String) -> Vec<String> {
    println!("Dir path: {:?}", dir_path);
    let mut media_files = Vec::new();
    for entry in WalkDir::new(dir_path) {
        let entry = entry.unwrap();
        if entry
            .path()
            .extension()
            .map_or(false, |ext| ext == "mp3" || ext == "MP3")
        {
            media_files.push(entry.path().to_string_lossy().into_owned());
        }
    }

    media_files
}

pub fn get_tag_info_mp3(
    apath: String,
) -> Result<(String, String, String, String, String, String), std::io::Error> {
    let tag = match Tag::read_from_path(apath.clone()) {
        Ok(tag) => tag,
        Err(_) => {
            println!("No ID3 tag found for: {:?}", apath.clone());
            // let target_dir = Path::new("/home/charliepi/needs_work");
            // if !target_dir.exists() {
            //     fs::create_dir_all(target_dir)?;
            // }
            // fs::rename(apath.clone(), target_dir.join(Path::new(&apath).file_name().unwrap()))?;
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "No ID3 tag found",
            ));
        }
    };

    let artist = match tag.artist() {
        Some(a) => a,

        None => {
            println!("No artist found for: {:?}", apath.clone());
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "No artist found",
            ));
        }
    };

    let album = match tag.album() {
        Some(a) => a,
        None => {
            println!("No album found for: {:?}", apath.clone());
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "No album found",
            ));
        }
    };

    let song = match tag.title() {
        Some(a) => a,
        None => {
            println!("No song found for: {:?}", apath.clone());
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "No song found",
            ));
        }
    };

    let cd = match tag.disc() {
        Some(a) => a,
        None => {
            println!("No CD found for: {:?}", apath.clone());
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "No CD found",
            ));
        }
    };
    let track = match tag.track() {
        Some(a) => a,
        None => {
            println!("No track found for: {:?}", apath.clone());
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "No track found",
            ));
        }
    };

    let rawgenre = match tag.genre() {
        Some(a) => a,
        None => {
            println!("No genre found for: {:?}", apath.clone());
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "No genre found",
            ));
        }
    };

    let rg1 = rawgenre.trim();
    let rg2 = rg1.replace("(", "").replace(")", "");
    let non_numeric_removed = rg2.chars().filter(|c| c.is_digit(10)).collect::<String>();
    let rg3 = match non_numeric_removed.parse::<u32>() {
        Ok(num) => num,
        Err(e) => match e.kind() {
            std::num::IntErrorKind::Empty => {
                println!(
                    "Genre string was empty after processing\n\t{:?}",
                    apath.clone()
                );
                148
            }
            _ => panic!("Failed to parse genre: {:?}", e),
        },
    };

    let genre = genrecodes::genre_code_to_name(rg3);

    Ok((
        artist.to_string(),
        album.to_string(),
        song.to_string(),
        cd.to_string(),
        track.to_string(),
        genre.to_string(),
    ))
}

pub fn repl_sp1(astring: String) -> String {
    if astring.contains("&") {
        return astring.replace("&", "And");
    } else {
        return astring.to_string();
    }
}

pub fn repl_sp2(astring: String) -> String {
    if astring.contains("+") {
        return astring.replace("+", "And");
    } else {
        return astring.to_string();
    }
}

pub fn repl_sp3(astring: String) -> String {
    if astring.contains("’") {
        return astring.replace("’", "");
    } else {
        return astring.to_string();
    };
}

pub fn repl_sp4(astring: String) -> String {
    if astring.contains(",") {
        return astring.replace(",", "");
    } else {
        return astring.to_string();
    };
}

pub fn repl_sp5(astring: String) -> String {
    if astring.contains(" - ") {
        return astring.replace(" - ", " ");
    } else {
        return astring.to_string();
    };
}

pub fn repl_sp6(astring: String) -> String {
    if astring.contains(" (") {
        let mysplit = astring.split(" (");
        let myvec = mysplit.collect::<Vec<&str>>();
        return myvec[0].to_string();
    } else {
        return astring.to_string();
    };
}

pub fn repl_sp7(astring: String) -> String {
    if astring.contains(".") {
        return astring.replace(".", "");
    } else {
        return astring.to_string();
    }
}

pub fn repl_sp8(astring: String) -> String {
    if astring.contains(":") {
        return astring.replace(":", "");
    } else {
        return astring.to_string();
    }
}

pub fn repl_sp9(astring: String) -> String {
    if astring.contains("?") {
        return astring.replace("?", "");
    } else {
        return astring.to_string();
    }
}

pub fn repl_sp10(astring: String) -> String {
    if astring.contains("!") {
        return astring.replace("!", "");
    } else {
        return astring.to_string();
    }
}

pub fn repl_sp11(astring: String) -> String {
    if astring.contains("  ") {
        return astring.replace("  ", " ");
    } else {
        return astring.to_string();
    }
}

pub fn repl_spl12(astring: String) -> String {
    if astring.contains("\"") {
        return astring.replace("\"", "");
    } else {
        return astring.to_string();
    }
}

pub fn capitalize_words(text: String) -> String {
    text.to_lowercase()
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            chars.next().map(|c| c.to_uppercase()).unwrap().to_string() + &chars.collect::<String>()
        })
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn convert_to_utf8(input: &str) -> Result<String, std::string::FromUtf8Error> {
    let bytes = input.as_bytes();
    String::from_utf8(bytes.to_vec())
}

pub fn rm_special_chars(astring: String) -> String {
    let a0 = repl_sp1(astring);
    let a1 = repl_sp2(a0);
    let a2 = repl_sp3(a1);
    let a3 = repl_sp4(a2);
    let a4 = repl_sp5(a3);
    let a5 = repl_sp6(a4);
    let a6 = repl_sp7(a5);
    let a7 = repl_sp8(a6);
    let a8 = repl_sp9(a7);
    let a9 = repl_sp10(a8);
    let a10 = repl_sp11(a9);
    let a11 = repl_spl12(a10);
    let a12 = match convert_to_utf8(&a11.as_str()) {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to convert to UTF-8: {:?}", e);
            return a11;
        }
    };
    let a13 = capitalize_words(a12);
    a13
}

pub fn write_tag_mp3(
    apath: String,
    artist: String,
    album: String,
    title: String,
    cd: String,
    track: String,
    genre: String,
) -> Result<(), std::io::Error> {
    let mut tag = match Tag::read_from_path(apath.clone()) {
        Ok(tag) => tag,
        Err(_) => {
            println!("No ID3 tag found for: {:?}", apath.clone());
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "No ID3 tag found",
            ));
        }
    };
    let cd1: Result<u32, _> = cd.parse();
    let cd2 = match cd1 {
        Ok(num) => num,
        Err(_) => {
            println!("CD is not a number: {:?}", cd);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "CD is not a number",
            ));
        }
    };
    let track1: Result<u32, _> = track.parse();
    let track2 = match track1 {
        Ok(num) => num,
        Err(_) => {
            println!("Track is not a number: {:?}", track);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Track is not a number",
            ));
        }
    };

    tag.set_artist(&artist);
    tag.set_album(&album);
    tag.set_title(&title);
    tag.set_disc(cd2);
    tag.set_track(track2);
    tag.set_genre(&genre);

    let write_tag_result = tag.write_to_path(apath.clone(), id3::Version::Id3v24);
    match write_tag_result {
        Ok(_) => {
            println!("Tag written to: {:?}", apath.clone());
        }
        Err(e) => {
            println!("Failed to write tag to: {:?}", apath.clone());
            println!("Error: {}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to write tag",
            ));
        }
    }


    Ok(())
}