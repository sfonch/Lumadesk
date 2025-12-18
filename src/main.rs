// use wallpaper;
use rfd;

fn main() {
    set_wallpaper();
}

fn set_wallpaper() {
    let path_buf = rfd::FileDialog::new()
        .add_filter("image", &["png", "jpeg"])
        .pick_file()
        .unwrap();

    let path_str = path_buf.to_string_lossy();

    wallpaper::set_from_path(&path_str).unwrap();
}
