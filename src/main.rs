use wallpaper;
use rfd;
use serde_json::json;
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
        "mp4" => video_to_sequence(&path_buf, &path_buf.file_stem().and_then(OsStr::to_str).unwrap()),
        _ => println!("Unsupported file"),
    };
}

fn get_fps(video_path: &Path) -> f32 {
    let output = Command::new("ffprobe")
        .arg("-v").arg("error")
        .arg("-select_streams").arg("v:0")
        .arg("-show_entries").arg("stream=r_frame_rate")
        .arg("-of").arg("default=noprint_wrappers=1:nokey=1")
        .arg(video_path)
        .output()
        .expect("ffprobe failed");

    let parts: Vec<f32> = String::from_utf8_lossy(&output.stdout)
        .trim()
        .split('/')
        .map(|x| x.parse::<f32>().unwrap())
        .collect();

    parts[0] / parts[1]
}

fn video_to_sequence(video_path: &Path, name: &str) {
    let mut output_path = dirs::cache_dir().unwrap();
    output_path.push("Lumadesk");
    output_path.push("wallpapers");
    output_path.push(name);
    output_path.push("frames");

    std::fs::create_dir_all(&output_path).unwrap();
    
    let mut info_json_path = output_path.clone();
    info_json_path.pop();
    info_json_path.push("info.json");

    let mut output_pattern = output_path.clone();
    output_pattern.push("%d.png");

    Command::new("ffmpeg")
        .arg("-i")
        .arg(video_path)
        .arg(&output_pattern)
        .status()
        .expect("ffmpeg failed");

    let fps = get_fps(video_path);
    create_info_json(&info_json_path, fps);
}

fn create_info_json(path: &Path, fps: f32) {
    let data = json!({
        "fps": fps
    });

    fs::write(path, data.to_string())
        .expect("failed to write info.json");
}