use serde::Deserialize;
use serde_json;
use std::{
    env,
    fs::{self, File},
    path::Path,
};
use walkdir::WalkDir;
use zip::ZipArchive;

#[derive(Deserialize)]
struct Info {
    #[serde(rename = "_songName")]
    song_name: String,
    #[serde(rename = "_songSubName")]
    song_sub_name: String,
    #[serde(rename = "_songAuthorName")]
    song_author_name: String,
    #[serde(rename = "_levelAuthorName")]
    level_author_name: String,
}

fn main() {
    let args = env::args();
    let format = if args.len() > 1 {
        let mut f = args.skip(1).collect::<Vec<String>>().join(" ");
        if !f.ends_with(".zip") {
            f.push_str(".zip");
        }
        f
    } else {
        "%N - %a.zip".to_owned()
    };

    let walkdir = WalkDir::new(".")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            let path = e.path();
            if path.is_dir() {
                return false;
            }
            if let Some(ext) = path.extension() {
                ext == "zip"
            } else {
                false
            }
        });

    for e in walkdir {
        let path = e.path();
        let file = match File::open(&path) {
            Ok(f) => f,
            Err(_) => {
                eprintln!("  Can't open {}, skipping", path.display());
                continue;
            }
        };
        let mut zip = match ZipArchive::new(file) {
            Ok(z) => z,
            Err(_) => {
                eprintln!("  Can't read {}, skipping", path.display());
                continue;
            }
        };

        let info_file = match zip.by_name("info.dat") {
            Ok(f) => f,
            Err(_) => {
                eprintln!("  No info.dat for {}, skipping", path.display());
                continue;
            }
        };
        let info: Info = match serde_json::from_reader(info_file) {
            Ok(i) => i,
            Err(_) => {
                eprintln!("  Invalid info.dat for {}, skipping", path.display());
                continue;
            }
        };

        let filename = format
            .replace("%N", &info.song_name)
            .replace("%n", &info.song_sub_name)
            .replace("%A", &info.song_author_name)
            .replace("%a", &info.level_author_name);
        let basename = path.parent().unwrap_or(Path::new("."));
        let rename = basename.join(filename);
        match fs::rename(&path, &rename) {
            Ok(_) => println!("{} => {}", path.display(), rename.display()),
            Err(_) => eprintln!("  Can't rename {}, skipping", path.display()),
        };
    }
}
