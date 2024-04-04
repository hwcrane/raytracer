use eframe::egui::load::SizedTexture;
use eframe::{egui, App};
use std::sync::{mpsc::Receiver, Arc, Mutex};

use crate::core::PixelData;

pub fn main(receiver: Receiver<PixelData>, image_width: u32, image_height: u32) {
    let options = eframe::NativeOptions::default();
    let app = MyApp::new(receiver, image_width, image_height);
    eframe::run_native("Ray Tracing", options, Box::new(|_cc| Box::new(app))).unwrap()
}

struct MyApp {
    reciver: Arc<Mutex<Receiver<PixelData>>>,
    pixels_recieved: u32,
    image_buffer: Vec<u8>,
    image_width: u32,
    image_height: u32,
}

impl MyApp {
    fn new(receiver: Receiver<PixelData>, image_width: u32, image_height: u32) -> Self {
        Self {
            reciver: Arc::new(Mutex::new(receiver)),
            image_buffer: vec![0; (image_width * image_height * 4) as usize],
            image_width,
            image_height,
            pixels_recieved: 0,
        }
    }

    fn update_image(&mut self) {
        let receiver = self.reciver.lock().unwrap();
        for data in receiver.try_iter() {
            self.pixels_recieved += 1;
            let PixelData { index, colour } = data;
            let pos = (index * 4) as usize;
            self.image_buffer[pos] = colour.0[0];
            self.image_buffer[pos + 1] = colour.0[1];
            self.image_buffer[pos + 2] = colour.0[2];
            self.image_buffer[pos + 3] = 255;
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.update_image();
        egui::CentralPanel::default().show(ctx, |ui| {
            let texture = egui::ColorImage::from_rgba_unmultiplied(
                [self.image_width as _, self.image_height as _],
                &self.image_buffer,
            );

            let texture_handle =
                ui.ctx()
                    .load_texture("rendered_image", texture, egui::TextureOptions::default());
            let t = SizedTexture::new(
                texture_handle.id(),
                egui::vec2(self.image_width as f32, self.image_height as f32),
            );
            ui.image(t)
        });

        // While the image is rendering, update every frame
        if self.pixels_recieved < self.image_width * self.image_height {
            ctx.request_repaint();
        }
    }
}
