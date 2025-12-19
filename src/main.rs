use wallpaper;
use rfd;
// use serde_json;
use dirs;
use std::fs;
use std::path::Path;
use std::ffi::OsStr;
use std::process::Command;

fn main() {
    create_root_folder();
    set_wallpaper();
}

fn create_root_folder() {
    let mut root_folder = dirs::cache_dir().unwrap();
    root_folder.push("Lumadesk");
    root_folder.push("wallpapers");

    fs::create_dir_all(&root_folder).unwrap();
}


fn set_wallpaper() {
    let path_buf = rfd::FileDialog::new()
        .add_filter("Image", &["png", "jpeg"])
        .add_filter("Video", &["mp4"])
        .pick_file()
        .unwrap();

    let path_str = path_buf.to_string_lossy();

    let ext = path_buf.extension().and_then(OsStr::to_str).unwrap();

    match ext {
        "png" | "jpeg" => wallpaper::set_from_path(&path_str).unwrap(),
        "mp4" => println!("{}", get_fps(&path_str)),
        _ => println!("Unsupported file"),
    };
}


fn get_fps(video_path: &str) -> String {
    let output = Command::new("ffprobe")
        .arg("-v")
        .arg("error")
        .arg("-select_streams")
        .arg("v:0")
        .arg("-show_entries")
        .arg("stream=r_frame_rate")
        .arg("-of")
        .arg("default=noprint_wrappers=1:nokey=1")
        .arg(video_path)
        .output()
        .expect("ffprobe failed");

    String::from_utf8_lossy(&output.stdout)
    .trim()
    .split('/')
    .collect::<Vec<_>>()
    .into_iter()
    .rev()
    .collect::<Vec<_>>()
    .join("/")
}

/*
fn video_to_sequence(video_path: Path, name: &str) {
    
}
*/