//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] 
use eframe::egui;
//use inline_python::python;
use std::fs;
use std::process::Command;

fn get_name() ->(String,String){
    let paths = fs::read_dir("./").unwrap();
    let mut apath = String::new();
    let mut vpath = String::new();
    let mut flag = 0;
    for path in paths {
        //println!("Name: {}", path.unwrap().path().display());
        let filen = path.unwrap().path().into_os_string();
        let files = filen.into_string().expect("UTF8 ERR");
        
        if (&files[files.len()-3..files.len()]=="m4s")&(flag == 0){
            vpath = files[2..files.len()].to_string();
            //println!("NAME:{}",vpath);
            flag = flag+1;
        }else if (&files[files.len()-3..files.len()]=="m4s")&(flag == 1){
            apath = files[2..files.len()].to_string();
            //println!("NAME:{}",apath);
        }
    }
    (apath,vpath)
}

fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();
    
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(480.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Bili2Mp4",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    audio: String,
    video: String,
    outa: String,
    outv: String,
}

impl Default for MyApp {
    fn default() -> Self {
        let default_audio;
        let default_video;
        (default_audio,default_video)=get_name();
        Self {
            //(audio,video)=get_name();
            audio: default_audio,
            video: default_video,
            outa: "R.mp3".to_owned(),
            outv: "R.mp4".to_owned(),
        }
        
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Bili2Mp4 V0.1.0");
            
            ui.horizontal(|ui| {
                ui.label("Audio name: ");
                ui.text_edit_singleline(&mut self.audio);
            });
            ui.horizontal(|ui| {
                ui.label("Video name: ");
                ui.text_edit_singleline(&mut self.video);
                
            });
            
            //ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));

            if ui.button("Find").clicked() {
                (self.audio,self.video)=get_name();
            }

            if ui.button("Begin").clicked() {               
                println!("Begin......");
                let audio_data = fs::read(&self.audio).expect("resd ERR");
                let audio_w = &audio_data[9..audio_data.len()];
                fs::write("temp.mp3", audio_w).expect("write REE");       
                let video_data = fs::read(&self.video).expect("resd ERR");
                let video_w = &video_data[9..video_data.len()];
                fs::write("temp.mp4", video_w).expect("write REE");   
                //if cuda is avaliable
                //let mut p1 = Command::new("./ffmpeg.exe").arg("-i").arg("temp.mp4").arg("-i").arg("temp.mp3").arg("-codec").arg("cuda").arg("-codec").arg("copy").arg("Resoult.mp4").spawn().unwrap();
                let mut p1 = Command::new("./ffmpeg.exe").arg("-i").arg("temp.mp4").arg("-i").arg("temp.mp3").arg("-codec").arg("copy").arg(&self.outv).spawn().unwrap();
                let mut p2 = Command::new("./ffmpeg.exe").arg("-i").arg("temp.mp3").arg("-ab").arg("320k").arg(&self.outa).spawn().unwrap();
                p1.wait().unwrap();
                p2.wait().unwrap();
                //python!{print("End......")}
            }
            ui.label(format!("au:{},vi:{}",self.audio,self.video));
            ui.horizontal(|ui| {
                ui.label("Out Audio: ");
                ui.text_edit_singleline(&mut self.outa);
            });
            ui.horizontal(|ui| {
                ui.label("Out Video: ");
                ui.text_edit_singleline(&mut self.outv);
                
            });
        });
    }
}
