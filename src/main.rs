use eframe::egui;
use egui::WidgetText;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};

fn main() -> Result<(), eframe::Error> {

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let (_stream2, stream_handle2) = OutputStream::try_default().unwrap();
    let (_stream3, stream_handle3) = OutputStream::try_default().unwrap();
    let (_stream4, stream_handle4) = OutputStream::try_default().unwrap();
    let (_stream5, stream_handle5) = OutputStream::try_default().unwrap();

    let mut testvec: Vec<Sink> = Vec::new();
    for i in 0..25{
        match i {
            1 | 2 | 3 | 4 | 5 => testvec.push(Sink::try_new(&stream_handle).unwrap()),
            6 | 7 | 8 | 9 | 10 => testvec.push(Sink::try_new(&stream_handle2).unwrap()),
            11 | 12 | 13 | 14 | 15 => testvec.push(Sink::try_new(&stream_handle3).unwrap()),
            16 | 17 | 18 | 19 | 20 => testvec.push(Sink::try_new(&stream_handle4).unwrap()),
            21 | 22 | 23 | 24 | 25 => testvec.push(Sink::try_new(&stream_handle5).unwrap()),
            _ => testvec.push(Sink::try_new(&stream_handle).unwrap())
        }
    }
    
    eframe::run_simple_native("Soundboard", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let paths = fs::read_dir("sounds").unwrap();
            for path in paths {
                let file_path = path.as_ref().unwrap().path();
                if ui.button(WidgetText::from(path.as_ref().unwrap().file_name().to_str().unwrap().strip_suffix(".mp3").unwrap())).clicked() {
                    let file = BufReader::new(File::open(file_path).unwrap());
                    let source = Decoder::new(file).unwrap();
                    for test in testvec.iter(){
                        if test.empty(){
                            test.append(source);
                            break;
                        }
                    }
                }
            }
            if ui.button("Open file…").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    let picked_path = path.display().to_string();
                    let file_name = path.file_name().unwrap().to_str().unwrap();
                    if picked_path.ends_with(".mp3"){
                        fs::copy(picked_path, format!("sounds/{}", file_name)).unwrap();
                    }
                }
            }
        });
    })
}