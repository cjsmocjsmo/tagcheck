use regex::Regex;
use std::env;
use std::path::Path;


mod genrecodes;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut sp_char_check = false;
    let mut sp_char_fix = false;
    let mut write_tag = false;

    if args.len() < 2 {
        println!("Please provide a directory path as an argument.");
        return;
    }

    for arg in &args {
        if arg == "-f" || arg == "--special-char-fix" {
            sp_char_fix = true;
        }
        if arg == "-s" || arg == "--special-char-check" {
            sp_char_check = true;
        }
        if arg == "-w" || arg == "--write-tag" {
            write_tag = true;
        }
        if arg == "-h" || arg == "--help" {
            println!("Usage: tagcheck [options] <directory path>");
            println!("\nOptions:");
            println!("\t-f, --special-char-fix\t\tFix special characters in tags");
            println!("\t-s, --special-char-check\tCheck for special characters in tags");
            println!("\t-w, --write-tag\t\t\tWrite tag info to file");
            println!("\t-h, --help\t\t\tDisplay this help message");
            return;
        }
    }

    let dir_path = &args[1];
    if !Path::new(dir_path).is_dir() {
        println!("The provided path is not a directory.");
        return;
    }

    let mediafiles = utils::find_media(&dir_path);
    // let mut totalcount = 0;
    let mut badcount = 0;
    let mut badfiles = Vec::new();
    for mediafile in mediafiles.clone() {
        // totalcount += 1;
        let tag_info = utils::get_tag_info_mp3(mediafile.clone());
        match tag_info {
            Ok((artist, album, title, cd, track, _genre)) => {
                let re = Regex::new(r"[^a-zA-Z0-9 \-']").unwrap();
                let re1 = Regex::new(r"^\d").unwrap();
                let re2 = Regex::new(r"^\d{1,2}").unwrap();
                if sp_char_check {
                    if re.is_match(&artist) {
                        badfiles.push(mediafile.clone());
                        println!("\nArtist has special characters:\n {}", artist);
                    }
                    if re.is_match(&album) {
                        badfiles.push(mediafile.clone());
                        println!("\nAlbum has special characters:\n {}\n", album);
                    }
                    if re.is_match(&title) {
                        badfiles.push(mediafile.clone());
                        println!("\nSong has special characters:\n {}\n", title);
                    }
                    if !re1.is_match(&cd) {
                        badfiles.push(mediafile.clone());
                        println!("\nCD is not formated correctly:\n {}\n", cd);
                    }
                    if !re2.is_match(&track) {
                        badfiles.push(mediafile.clone());
                        println!("\nTrack is not formated correctly:\n {}\n", track);
                    }
                    // println!("Genre: {}\n", genre);
                }
            }
            Err(e) => {
                println!("Tag Info is missing\n\t{:?}", mediafile.clone());
                println!("Error: {}", e);
                badcount += 1;
            }
        }
    }
    // let mut newtaginfo = Vec::new();
    if sp_char_fix {
        for badfile in badfiles.clone() {
            let taginfo2 = utils::get_tag_info_mp3(badfile.clone());
            match taginfo2 {
                Ok((artist, album, title, cd, track, genre)) => {
                    let art = utils::rm_special_chars(artist);
                    let alb = utils::rm_special_chars(album);
                    let song = utils::rm_special_chars(title);
                    println!(
                        "Artist: {}\nAlbum: {}\nSong: {}\nCD: {}\nTrack: {}\nGenre: {}\n",
                        art, alb, song, cd, track, genre
                    );
                    if write_tag {
                        let _ = utils::write_tag_mp3(
                            badfile.clone(),
                            art,
                            alb,
                            song,
                            cd,
                            track,
                            genre,
                        );
                    }
                }
                Err(e) => {
                    println!("Tag Info is missing\n\t{:?}", badfile.clone());
                    println!("Error: {}", e);
                    badcount += 1;
                }
            }
        }
    }

    // println!("{:#?}", newtaginfo);

    // println!("Bad files {:#?}", badfiles.clone().len());
    println!("Total media files with missing tag info: {}", badcount);
    println!("Total media files scanned: {}", mediafiles.clone().len());
}
