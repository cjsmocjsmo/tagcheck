use std::env;
use std::path::Path;
use id3::{Tag, TagLike};
use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a directory path as an argument.");
        return;
    }

    let dir_path = &args[1];
    if !Path::new(dir_path).is_dir() {
        println!("The provided path is not a directory.");
        return;
    }

    let mediafiles = find_media(&dir_path);
    let mut totalcount = 0;
    let mut badcount = 0;
    for mediafile in mediafiles {
        totalcount += 1;
        let tag_info = get_tag_info_mp3(mediafile.clone());
        if !tag_info {
            badcount += 1;
        
        }
    }
    println!("Total media files with missing tag info: {}", badcount);
    println!("Total media files scanned: {}", totalcount);
}

pub fn find_media(dir_path: &String) -> Vec<String> {
    println!("Dir path: {:?}", dir_path);
    let mut media_files = Vec::new();
    for entry in WalkDir::new(dir_path) {
        let entry = entry.unwrap();
        if entry.path().extension().map_or(false, |ext| {
            ext == "mp3"
                || ext == "MP3"
        }) {
            media_files.push(entry.path().to_string_lossy().into_owned());
        }
    }

    media_files
}

// pub fn get_tag_info_mp3(apath: String) -> Result<(String, String, String, String, String, String), std::io::Error> {
    pub fn get_tag_info_mp3(apath: String) -> bool {
    let tag = match Tag::read_from_path(apath.clone()) {
        Ok(tag) => tag,
        Err(_) => {
            println!("\n\nNo ID3 tag found for:\n {:?}", apath.clone());
            return false;
        }
    };

    let mut results = true;

    if tag.artist().unwrap_or("").is_empty() {
        println!("\n\nArtist tag is missing\n{:?}", apath.clone());
        results = false;
    }

    if tag.album().unwrap_or("").is_empty() {
        println!("\n\nAlbum tag is missing\n{:?}", apath.clone());
        results = false;
    }

    if tag.title().unwrap_or("").is_empty() {
        println!("\n\nSong tag is missing\n{:?}", apath.clone());
        results = false;
    }

    // if tag.disc().is_none() {
    //     println!("CD tag is missing\n\t{:?}", apath.clone());
    //     results = false;
    // }

    if tag.track().is_none() {
        println!("\n\nTrack tag is missing\n{:?}", apath.clone());
        results = false;
    }

    if tag.genre().is_none() {
        println!("\n\nGenre tag is missing\n{:?}", apath.clone());
        results = false;
    }

    results

}

