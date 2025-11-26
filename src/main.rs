
use eframe::egui; // brings everything we need into scope

fn main() -> Result<(), eframe::Error> {
    // Launch the window
    let options = eframe::NativeOptions {
        // Dark mode by default (looks sleek)
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 200.0]),
        ..Default::default()          // use all other defaults
    };

    eframe::run_native(
        "Blaze Player",                // window title
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))), // create our app
    )
}

// ──────────────────────────────────────────────────────────────
// This is our application state (the "Model" in MVC)
// For now it's tiny — just a counter so we can see clicks
// ──────────────────────────────────────────────────────────────
#[derive(Default)]
struct MyApp {
    clicks: u32,
}

impl eframe::App for MyApp {
    // This function is called every frame (~60 times per second)
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top panel (optional title bar)
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.heading("Blaze Player – Minimal Demo");
        });

        // Main central area
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(30.0);

                // Big friendly button
                if ui.button("Click me!").clicked() {
                    self.clicks += 1;                         // update our state
                    println!("Hello World! (clicked {} times)", self.clicks);
                }

                ui.add_space(20.0);
                ui.label(format!("You have clicked {} times", self.clicks));
            });
        });
    }
}