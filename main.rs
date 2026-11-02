#![windows_subsystem = "windows"]
use eframe::egui::{self, ColorImage, TextureHandle};
use screenshots::Screen;
use std::time::{Duration, Instant};
use std::fs;
use std::thread;
use std::sync::mpsc;
use std::process::Command;
use image::{RgbaImage, DynamicImage};
use rusty_tesseract::{Args, Image as TesseractImage};
use std::os::windows::process::CommandExt;

fn main() -> Result<(), eframe::Error> {
    let current_path = std::env::var("PATH").unwrap_or_default();
    unsafe {
        std::env::set_var("PATH", format!("C:\\Program Files\\Tesseract-OCR;{}", current_path));
    }

    let options = eframe::NativeOptions::default();
    let mut is_running = false;
    let mut last_capture_time = Instant::now();
    let mut texture: Option<TextureHandle> = None;
    let mut detected_text = String::new();
    let mut selected_screen_index = 0;

    let mut x1 = String::from("620");
    let mut y1 = String::from("620");
    let mut x2 = String::from("1300");
    let mut y2 = String::from("1000");
    
    let mut target_text = String::new();
    let mut link_text = String::new();

    if let Ok(content) = fs::read_to_string("data.txt") {
        let lines: Vec<&str> = content.lines().collect();
        if lines.len() >= 6 {
            x1 = lines[0].to_string();
            y1 = lines[1].to_string();
            x2 = lines[2].to_string();
            y2 = lines[3].to_string();
            target_text = lines[4].to_string();
            link_text = lines[5].to_string();
        } else if lines.len() >= 4 {
            x1 = lines[0].to_string();
            y1 = lines[1].to_string();
            x2 = lines[2].to_string();
            y2 = lines[3].to_string();
        }
    }

    let (tx, rx) = mpsc::channel();

    eframe::run_ui_native("Game Disconnect Detector", options, move |ui, _frame| {
        ui.ctx().set_visuals(egui::Visuals::light());
        ui.painter().rect_filled(ui.max_rect(), 0.0, ui.visuals().panel_fill);
        let screens = Screen::all().unwrap_or_default();
        
        if let Ok(new_text) = rx.try_recv() {
            detected_text = new_text;
        }

        ui.horizontal(|ui| {
            ui.label("Monitor:");
            egui::ComboBox::from_id_salt("screen_selector")
                .selected_text(format!("Screen {}", selected_screen_index + 1))
                .show_ui(ui, |ui| {
                    for (index, screen) in screens.iter().enumerate() {
                        let label = format!("Screen {} ({}x{})", index + 1, screen.display_info.width, screen.display_info.height);
                        ui.selectable_value(&mut selected_screen_index, index, label);
                    }
                });
        });

        ui.add_space(10.0);

        ui.label("Working Area Coordinates:");
        ui.horizontal(|ui| {
            ui.label("X1:");
            ui.add(egui::TextEdit::singleline(&mut x1).desired_width(60.0));
            ui.label("Y1:");
            ui.add(egui::TextEdit::singleline(&mut y1).desired_width(60.0));
            
            ui.label("  |  X2:");
            ui.add(egui::TextEdit::singleline(&mut x2).desired_width(60.0));
            ui.label("Y2:");
            ui.add(egui::TextEdit::singleline(&mut y2).desired_width(60.0));
        });

        ui.add_space(10.0);

        ui.label("Trigger:");
        ui.horizontal(|ui| {
            ui.label("Target*:");
            ui.add(egui::TextEdit::singleline(&mut target_text).desired_width(120.0));
            ui.label("Link:");
            ui.add(egui::TextEdit::singleline(&mut link_text).desired_width(200.0));
        });

        ui.add_space(10.0);

        ui.horizontal(|ui| {
            if ui.button("▶ Start").clicked() {
                is_running = true;
                last_capture_time = Instant::now() - Duration::from_secs(5);
                let save_data = format!("{}\n{}\n{}\n{}\n{}\n{}", x1, y1, x2, y2, target_text, link_text);
                let _ = fs::write("data.txt", save_data);
                detected_text = "Waiting...".to_string();
            }
            if ui.button("⏹ Stop").clicked() {
                is_running = false;
            }
        });

        ui.add_space(10.0);

        if is_running {
            ui.label("Status: 🟢 Working (Checking every 5 seconds...)");
            
            if last_capture_time.elapsed() >= Duration::from_secs(5) {
                if let Some(screen) = screens.get(selected_screen_index) {
                    let px1 = x1.parse::<i32>().unwrap_or(0);
                    let py1 = y1.parse::<i32>().unwrap_or(0);
                    let px2 = x2.parse::<i32>().unwrap_or(0);
                    let py2 = y2.parse::<i32>().unwrap_or(0);
                    let width = (px2 - px1).max(1) as u32;
                    let height = (py2 - py1).max(1) as u32;

                    if let Ok(capture) = screen.capture_area(px1, py1, width, height) {
                        let size = [capture.width() as usize, capture.height() as usize];
                        let raw_pixels = capture.into_raw();
                        let color_image = ColorImage::from_rgba_unmultiplied(size, &raw_pixels);
                        
                        if let Some(tex) = &mut texture {
                            tex.set(color_image, egui::TextureOptions::LINEAR);
                        } else {
                            texture = Some(ui.ctx().load_texture(
                                "screenshot",
                                color_image,
                                egui::TextureOptions::LINEAR
                            ));
                        }

                        let tx_clone = tx.clone();
                        let target_clone = target_text.clone();
                        let link_clone = link_text.clone();

                        thread::spawn(move || {
                            if let Some(rgba_img) = RgbaImage::from_raw(width, height, raw_pixels) {
                                let mut dyn_img = DynamicImage::ImageRgba8(rgba_img).grayscale();
                                dyn_img = dyn_img.adjust_contrast(10.0);
                                
                                if let Ok(tes_img) = TesseractImage::from_dynamic_image(&dyn_img) {
                                    let mut args = Args::default();
                                    args.lang = "eng+tur".to_string(); 
                                    args.psm = Some(11);
                                    let mut config = std::collections::HashMap::new();
                                    config.insert("tessedit_char_blacklist".to_string(), "|[]{}()<>*&^%$#@!~_\\/".to_string());
                                    args.config_variables = config;
                                    
                                    match rusty_tesseract::image_to_string(&tes_img, &args) {
                                        Ok(text) => {
                                            let cleaned = text.replace('\n', " ").replace('\r', "").trim().to_string();
                                            let final_text = if cleaned.is_empty() {
                                                "Text not found.".to_string()
                                            } else {
                                                cleaned
                                            };

                                            if !target_clone.is_empty() && final_text.to_lowercase().contains(&target_clone.to_lowercase()) {
                                                if !link_clone.is_empty() {
                                                    let _ = reqwest::blocking::get(&link_clone);
                                                }
                                                let _ = Command::new("powershell").args(["-c", "[console]::beep(440,500)"]).creation_flags(0x08000000).output();
                                            }
                                            
                                            let _ = tx_clone.send(final_text);
                                        }
                                        Err(_) => {
                                            let _ = tx_clone.send("Error: Tesseract not found!".to_string());
                                        }
                                    }
                                }
                            }
                        });
                    }
                }
                last_capture_time = Instant::now();
            }
            ui.ctx().request_repaint_after(Duration::from_millis(100));
        } else {
            ui.label("Status: 🔴 Stopped");
        }

        ui.add_space(10.0);

        if !detected_text.is_empty() {
            ui.label(format!("Recognized Text: {}", detected_text));
        }

        if let Some(tex) = &texture {
            ui.add_space(10.0);
            ui.add(egui::Image::new(tex));
        }
    })
}
