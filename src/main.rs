use walkdir::WalkDir;
use id3::Tag;
// use std::fs;

pub fn walk_additional_dir(apath: String) {
    let mut index = 0;
    let mut notagcount = 0;
    for e in WalkDir::new(apath)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if e.metadata().unwrap().is_file() {
            let fname = e.path().to_string_lossy().to_string();

            if fname.contains("Music") && fname.ends_with(".mp3") {
                let tag = Tag::read_from_path(&fname);
                if let Ok(_tag) = tag {
                    println!("{}", index.to_string());
                    // Tag information found
                    // Process the tag as needed
                } else {
                    // No tag information found
                    notagcount = notagcount + 1;
                    let dest_path = "/home/pi/needs_work";
                    let dest_file = format!("{}/{}", dest_path, e.file_name().to_string_lossy());
                    println!("{}", dest_file);
                    // fs::rename(&fname, &dest_file).unwrap_or_else(|err| {
                    //     eprintln!("Failed to move file: {}", err);
                    // });
                }
                index = index + 1;
            }
        }
    }
    println!("{}", notagcount)  
}

fn main() {
    walk_additional_dir("/home/pi/Music".to_string());
}