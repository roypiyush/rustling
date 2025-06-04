use eframe::egui;
use std::time::{Duration, Instant};

pub struct TimerApp {
    start_time: Option<Instant>,
    elapsed: Duration,
    running: bool,
}

impl Default for TimerApp {
    fn default() -> Self {
        Self {
            start_time: None,
            elapsed: Duration::ZERO,
            running: false,
        }
    }
}

impl eframe::App for TimerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.running {
            if let Some(start) = self.start_time {
                self.elapsed = Instant::now().duration_since(start);
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let minutes = self.elapsed.as_secs() / 60;
            let seconds = self.elapsed.as_secs() % 60;

            ui.heading(format!("{:02}:{:02}", minutes, seconds));

            if ui.button(if self.running { "Pause" } else { "Start" }).clicked() {
                if self.running {
                    self.running = false;
                    self.elapsed = Instant::now().duration_since(self.start_time.unwrap());
                    self.start_time = None;
                } else {
                    self.running = true;
                    self.start_time = Some(Instant::now() - self.elapsed);
                }
            }

            if ui.button("Reset").clicked() {
                self.running = false;
                self.elapsed = Duration::ZERO;
                self.start_time = None;
            }
        });

        ctx.request_repaint_after(std::time::Duration::from_millis(100));
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Timer App",
        options,
        Box::new(|_cc| Box::new(TimerApp::default())),
    )
}