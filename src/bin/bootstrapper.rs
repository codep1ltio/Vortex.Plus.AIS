use eframe::egui;

mod launcher;
use launcher::launcher::*;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([700.0, 400.0])
            .with_resizable(false),

        ..Default::default()
    };

    eframe::run_native(
        "Vortex2+2 AIS Launcher",
        options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}

struct App {
    status: String,
    check_for_updates: bool,
    launched: bool,
    logs: Vec<String>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            status: "".to_string(),
            check_for_updates: true,
            launched: false,
            logs: vec![
                "Bootstrapper Loaded.".to_string(),
                "-------------------------------".to_string(),
                "If you haven't yet, please join our discord!".to_string(),
                "We need your help and your suggestions!".to_string(),
                "https://discord.gg/E9y6WfEdPW".to_string(),
                "-------------------------------".to_string(),
            ],
        }
    }
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        Self::default()
    }

    fn push_log(&mut self, message: impl Into<String>) {
        self.logs.push(message.into());
        if self.logs.len() > 64 {
            self.logs.remove(0);
        }
    }

    fn start_update_check(&mut self) {
        self.check_for_updates = true;
        self.status = "Checking for updates...".to_string();
        self.push_log("Update check started.");
        // TODO: check and actually update
    }

    fn open_mods_folder(&mut self) {
        open_mods_folder()
            .map(|_| self.push_log("Opened mods folder"))
            .unwrap_or_else(|e: std::io::Error| {
                self.push_log(&format!("Error while opening mods folder: {}", e));
            });
    }

    fn launch(&mut self) {
        self.status = "Launching Vortex AIS...".to_string();
        self.push_log("Launch requested.");

        match launch_vortex() {
            Ok(()) => {
                std::process::exit(0);
            }
            Err(err) => {
                self.status = format!("Launch failed: {}", err);
                self.push_log(format!("Launch error: {}", err));
                self.push_log(
                    "If you're unsure of the cause, feel free to contact us in our discord.",
                );
                self.launched = false;
            }
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("side_panel")
            .resizable(false)
            .default_width(300.0)
            .show(ctx, |ui| {
                ui.add_space(8.0);
                ui.heading("Options");
                ui.add_space(8.0);

                ui.checkbox(&mut self.check_for_updates, "Check for updates on launch");

                ui.add_space(12.0);
                ui.monospace("Current Build");
                ui.label("Vortex2+2 AIS bootstrapper");
                ui.label("Launcher v0.2");

                ui.add_space(12.0);
                let btn_disable = self.launched;
                if ui
                    .add_enabled(!btn_disable, egui::Button::new("Check for updates"))
                    .clicked()
                {
                    self.start_update_check();
                }

                if ui
                    .add_enabled(!btn_disable, egui::Button::new("Open Mods Folder"))
                    .clicked()
                {
                    self.open_mods_folder();
                }

                ui.add_space(12.0);
                ui.monospace("Contribute | Community");
                ui.label("github.com/codep1ltio/Vortex.AIS");
                ui.label("discord.gg/E9y6WfEdPW");
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.add_space(10.0);
                ui.heading("Welcome");
                ui.label("Vortex2+2 AIS is here to make your life easier.");
                ui.add_space(10.0);
            });

            ui.vertical(|ui| {
                ui.vertical_centered(|ui| {
                    ui.label(&self.status);
                    if !self.status.is_empty() {
                        ui.add_space(1.5);
                    }

                    let btn_disable = self.launched;
                    let launch_button = ui.add_enabled(
                        !btn_disable,
                        egui::Button::new("Launch and Update Vortex AIS")
                            .rounding(15.0)
                            .min_size(egui::vec2(220.0, 40.0)),
                    );

                    if launch_button.clicked() {
                        self.launched = true;
                        self.launch();
                    }

                    ui.small(format!("{}", 
                        (if self.check_for_updates { "" } else { "Update checking is disabled." })
                    ));
                    ui.add_space(1.5);
                });
            });

            ui.add_space(12.0);

            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.monospace("Console");
                    if ui.small_button("Clear").clicked() {
                        self.logs.clear();
                    }
                });

                ui.separator();

                egui::ScrollArea::vertical()
                    .max_height(180.0)
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        for line in &self.logs {
                            ui.monospace(line);
                        }
                    });
            });
        });
    }
}
