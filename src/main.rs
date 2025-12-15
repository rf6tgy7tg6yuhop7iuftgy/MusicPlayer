use std::path::Path;
use directories::UserDirs;

use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window, group::Flex, enums::Align, text::{TextBuffer, TextEditor}};

fn main() {
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 1600, 900, "Music Player");
    wind.make_resizable(true);

    let mut flex = Flex::new(0, 0, 1600, 900, None);
    flex.set_type(fltk::group::FlexType::Column);
    flex.set_margin(5);

    let default_path = get_default_path();
    let music_folder = Box::leak(default_path.into_boxed_str());

    let mut text_buf = TextBuffer::default();
    let mut text_editor = TextEditor::default();
    let mut btn = Button::default().with_label("Search Music Directory");
    let mut frame = Frame::default().with_label(format!("{}{}", "Autodetected directory: ", music_folder).as_str());

    text_buf.set_text(music_folder);
    text_editor.set_buffer(text_buf.clone());

    let mut frame_cb = frame.clone();
    let text_buf_cb = text_buf.clone();
    btn.set_callback(move |_| {
        let txt = text_buf_cb.text();
        let files = get_files(Path::new(&txt));
        frame_cb.set_label(&files.join("\n"));
    });

    btn.set_align(Align::Left | Align::Inside);
    frame.set_align(Align::TopLeft | Align::Inside);
    flex.fixed(&mut text_editor, 24);
    flex.fixed(&mut btn, 30);

    flex.end();
    wind.end();
    wind.show();

    app.run().unwrap();
}
fn get_default_path() -> String {
    if let Some(user_dirs) = UserDirs::new() {
        if let Some(music_dir) = user_dirs.audio_dir() {
            return music_dir.to_string_lossy().into_owned();
        }
    }
    return String::from("Failed to get music directory automatically");
}
const ALLOWED_EXTENSIONS: [&str; 3] = ["mp3", "ogg", "wav"];
fn get_files(path: &Path) -> Vec<String> {
    if path.is_dir() {
        let entries = std::fs::read_dir(path).unwrap();
        let mut file_names = Vec::new();
        for entry in entries {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ALLOWED_EXTENSIONS.contains(&ext.to_string_lossy().to_lowercase().as_str()) {
                        file_names.push(path.file_name().unwrap().to_string_lossy().into_owned());
                    }
                }
            } else if path.is_dir() {
                let sub_files = get_files(&path);
                file_names.push(path.file_name().unwrap().to_string_lossy().into_owned());
                for sub_file in &sub_files {
                    file_names.push(format!(" | {}", sub_file));
                }
            }
        }
        return file_names;
    }
    return vec!["Failed to find music folder".to_string()];
}