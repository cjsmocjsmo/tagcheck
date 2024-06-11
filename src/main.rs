use std::env;
use std::fs;
use std::path::Path;
use id3::{Tag, TagLike};
// use std::path::PathBuf;
// use std::process::Command;
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
        match tag_info {
            // Ok((artist, album, title, cd, track, genre)) => {
            //     println!("\nArtist: {}\n", artist);
            //     println!("Album: {}\n", album);
            //     println!("Song: {}\n", title);
            //     println!("CD: {}\n", cd);
            //     println!("Track: {}\n", track);
            //     println!("Genre: {}\n", genre);
            // }
            Ok((_, _, _, _, _, _)) => (),
            Err(e) => {
                println!("Tag Info is missing\n\t{:?}", mediafile.clone());
                println!("Error: {}", e);
                badcount += 1;
            }
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

pub fn get_tag_info_mp3(apath: String) -> Result<(String, String, String, String, String, String), std::io::Error> {
    let tag = match Tag::read_from_path(apath.clone()) {
        Ok(tag) => tag,
        Err(_) => {
            println!("No ID3 tag found for: {:?}", apath.clone());
            let target_dir = Path::new("/home/charliepi/needs_work");
            if !target_dir.exists() {
                fs::create_dir_all(target_dir)?;
            }
            fs::rename(apath.clone(), target_dir.join(Path::new(&apath).file_name().unwrap()))?;
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "No ID3 tag found",
            ));
        }
    };

    let artist = tag.artist().expect(&apath.clone());
    let album = tag.album().expect(&apath);
    let song = tag.title().expect(&apath);
    let cd1 = tag.disc();
    let cd = match cd1 {
        Some(cd) => cd.to_string(),
        None => "01".to_string(),
    };
    let track = tag.track().expect(&apath);
    
    let rawgenre1 = tag.genre();
    // let default: &'static str = "(148)";
    let rawgenre = match rawgenre1 {
        Some(genre) => genre,
        // None => default,
        None => {
            println!("Genre tag is missing\n\t{:?}", apath.clone());
            "(148)"
        },
    };
    let rg1 = rawgenre.trim();
    let rg2 = rg1.replace("(", "").replace(")", "");
    let non_numeric_removed = rg2.chars().filter(|c| c.is_digit(10)).collect::<String>();
    println!("Raw genre: {:?}", &non_numeric_removed);
    let rg3 = match non_numeric_removed.parse::<u32>() {
        Ok(num) => num,
        Err(e) => match e.kind() {
            std::num::IntErrorKind::Empty => {
                // Handle the empty case, e.g., by using a default value or logging an error
                println!("Genre string was empty after processing\n\t{:?}", apath.clone());
                148 // Assuming 0 as a default value, adjust as necessary
            },
            _ => panic!("Failed to parse genre: {:?}", e),
        },
    };

    let genre = genre_code_to_name(rg3);

    // println!("Raw genre: {:?}", &rg3);
    // let genre1 = rawgenre.parse::<u32>().unwrap();
    // let genre = genre_code_to_name(genre1);

    Ok((
        artist.to_string(),
        album.to_string(),
        song.to_string(),
        cd.to_string(),
        track.to_string(),
        genre.to_string(),
    ))
}

fn genre_code_to_name(code: u32) -> &'static str {
    match code {
        0 => "Blues",
        1 => "Classic Rock",
        2 => "Country",
        3 => "Dance",
        4 => "Disco",
        5 => "Funk",
        6 => "Grunge",
        7 => "Hip-Hop",
        8 => "Jazz",
        9 => "Metal",
        10 => "New Age",
        11 => "Oldies",
        12 => "Other",
        13 => "Pop",
        14 => "R&B",
        15 => "Rap",
        16 => "Reggae",
        17 => "Rock",
        18 => "Techno",
        19 => "Industrial",
        20 => "Alternative",
        21 => "Ska",
        22 => "Death Metal",
        23 => "Pranks",
        24 => "Soundtrack",
        25 => "Euro-Techno",
        26 => "Ambient",
        27 => "Trip-Hop",
        28 => "Vocal",
        29 => "Jazz+Funk",
        30 => "Fusion",
        31 => "Trance",
        32 => "Classical",
        33 => "Instrumental",
        34 => "Acid",
        35 => "House",
        36 => "Game",
        37 => "Sound Clip",
        38 => "Gospel",
        39 => "Noise",
        40 => "Alternative Rock",
        41 => "Bass",
        42 => "Soul",
        43 => "Punk",
        44 => "Space",
        45 => "Meditative",
        46 => "Instrumental Pop",
        47 => "Instrumental Rock",
        48 => "Ethnic",
        49 => "Gothic",
        50 => "Darkwave",
        51 => "Techno-Industrial",
        52 => "Electronic",
        53 => "Pop-Folk",
        54 => "Eurodance",
        55 => "Dream",
        56 => "Southern Rock",
        57 => "Comedy",
        58 => "Cult",
        59 => "Gangsta",
        60 => "Top 40",
        61 => "Christian Rap",
        62 => "Pop/Funk",
        63 => "Jungle",
        64 => "Native US",
        65 => "Cabaret",
        66 => "New Wave",
        67 => "Psychadelic",
        68 => "Rave",
        69 => "Showtunes",
        70 => "Trailer",
        71 => "Lo-Fi",
        72 => "Tribal",
        73 => "Acid Punk",
        74 => "Acid Jazz",
        75 => "Polka",
        76 => "Retro",
        77 => "Musical",
        78 => "Rock & Roll",
        79 => "Hard Rock",
        80 => "Folk",
        81 => "Folk-Rock",
        82 => "National Folk",
        83 => "Swing",
        84 => "Fast Fusion",
        85 => "Bebob",
        86 => "Latin",
        87 => "Revival",
        88 => "Celtic",
        89 => "Bluegrass",
        90 => "Avantgarde",
        91 => "Gothic Rock",
        92 => "Progressive Rock",
        93 => "Psychedelic Rock",
        94 => "Symphonic Rock",
        95 => "Slow Rock",
        96 => "Big Band",
        97 => "Chorus",
        98 => "Easy Listening",
        99 => "Acoustic",
        100 => "Humour",
        101 => "Speech",
        102 => "Chanson",
        103 => "Opera",
        104 => "Chamber Music",
        105 => "Sonata",
        106 => "Symphony",
        107 => "Booty Bass",
        108 => "Primus",
        109 => "Porn Groove",
        110 => "Satire",
        111 => "Slow Jam",
        112 => "Club",
        113 => "Tango",
        114 => "Samba",
        115 => "Folklore",
        116 => "Ballad",
        117 => "Power Ballad",
        118 => "Rhythmic Soul",
        119 => "Freestyle",
        120 => "Duet",
        121 => "Punk Rock",
        122 => "Drum Solo",
        123 => "Acapella",
        124 => "Euro-House",
        125 => "Dance Hall",
        126 => "Goa",
        127 => "Drum & Bass",
        128 => "Club - House",
        129 => "Hardcore",
        130 => "Terror",
        131 => "Indie",
        132 => "BritPop",
        133 => "Negerpunk",
        134 => "Polsk Punk",
        135 => "Beat",
        136 => "Christian Gangsta Rap",
        137 => "Heavy Metal",
        138 => "Black Metal",
        139 => "Crossover",
        140 => "Contemporary Christian",
        141 => "Christian Rock",
        142 => "Merengue",
        143 => "Salsa",
        144 => "Thrash Metal",
        145 => "Anime",
        146 => "JPop",
        147 => "Synthpop",
        148 => "Unknown",
        _ => "Unknown",
    }
}