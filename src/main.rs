use wallpaper;
use rfd;
// use serde_json;
use dirs;
use std::fs;

fn main() {
    create_folder();
    set_wallpaper();
}

fn create_folder() {
    let mut path = dirs::cache_dir().unwrap();
    path.push("Lumadesk");
    path.push("wallpapers");

    fs::create_dir_all(&path).unwrap();
}


fn set_wallpaper() {
    let path_buf = rfd::FileDialog::new()
        .add_filter("Image", &["png", "jpeg"])
        .pick_file()
        .unwrap();

    let path_str = path_buf.to_string_lossy();

    wallpaper::set_from_path(&path_str).unwrap();
}
