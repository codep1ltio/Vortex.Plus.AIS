use eframe::egui;
use std::time::{Instant};

mod launcher;
use launcher::launcher::launch_vortex;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([700.0, 400.0])
            .with_resizable(false),

        ..Default::default()
    };

    eframe::run_native(
        "Vortex+ Launcher",
        options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}

struct App {
    status: String,
    checking_updates: bool,
    activity_started_at: Option<Instant>,
    auto_update_check: bool,
    auto_launch: bool,
    show_log: bool,
    launching: bool,
    repairing: bool,
    logs: Vec<String>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            status: "Ready.".to_string(),
            checking_updates: false,
            activity_started_at: None,
            auto_update_check: true,
            auto_launch: false,
            show_log: true,
            launching: false,
            repairing: false,
            logs: vec![
                "Launcher started.".to_string(),
                "-----------------------------".to_string(),
                "If you haven't yet, please join our discord!".to_string(),
                "We need your help and your suggestions!".to_string(),
                "https://discord.gg/Q8wZHJ8PuF".to_string(),
                "-----------------------------".to_string(),
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
        self.checking_updates = true;
        self.activity_started_at = Some(Instant::now());
        self.status = "Checking for updates...".to_string();
        self.push_log("Update check started.");
        // TODO: check and actually update
    }

    fn start_repair_procedure(&mut self) {
        self.repairing = true;
        // TODO: repair
    }

    fn launch(&mut self) {
        self.status = "Launching Vortex+...".to_string();
        self.push_log("Launch requested.");

        match launch_vortex() {
            Ok(()) => {
                self.status = "Vortex+ launched.".to_string();
                self.push_log("Launch completed.");
                // TODO: launch
                std::process::exit(0);
            }
            Err(err) => {
                self.status = format!("Launch failed: {}", err);
                self.push_log(format!("Launch error: {}", err));
                self.push_log(
                    "If you're unsure of the cause, feel free to contact us in our discord.",
                );
                self.launching = false;
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

                ui.checkbox(&mut self.auto_update_check, "Check updates on start");
                ui.checkbox(&mut self.auto_launch, "Launch after update check");
                ui.checkbox(&mut self.show_log, "Show log");

                ui.add_space(12.0);
                ui.monospace("Current Build");
                ui.label("Vortex+ bootstrapper");
                ui.label("Launcher v0.1");

                ui.add_space(12.0);
                let btn_disable = self.launching || self.checking_updates || self.repairing;
                if ui
                    .add_enabled(
                        !btn_disable,
                        egui::Button::new("Check for updates"),
                    )
                    .clicked()
                {
                    self.start_update_check();
                }

                if ui
                    .add_enabled(
                        !btn_disable,
                        egui::Button::new("Repair Vortex"),
                    )
                    .clicked()
                {
                    self.start_repair_procedure();
                }

                ui.add_space(12.0);
                ui.monospace("Contribute | Community");
                ui.label("github.com/codep1ltio/Vortex.Plus.AIS");
                ui.label("discord.gg/Q8wZHJ8PuF");
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.add_space(10.0);
                ui.heading("Welcome");
                ui.label("Use this launcher to update and start Vortex+.");
                ui.add_space(10.0);
            });

            ui.group(|ui| {
                ui.vertical_centered(|ui| {
                    ui.label("Status");
                    ui.label(&self.status);

                    ui.add_space(1.5);

                    let btn_disable = self.launching || self.checking_updates || self.repairing;

                    let launch_button = ui.add_enabled(
                        !btn_disable,
                        egui::Button::new("Launch Vortex+")
                            .rounding(10.0)
                            .min_size(egui::vec2(220.0, 36.0)),
                    );

                    if launch_button.clicked() {
                        self.launching = true;
                        self.launch();
                    }

                    if self.auto_update_check && !self.checking_updates {
                        ui.small("Auto update check is enabled.");
                    }
                });
            });

            ui.add_space(12.0);

            if self.show_log {
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
            }
        });

        if self.auto_launch && !self.checking_updates {
            self.auto_launch = false;
            self.launch();
        }
    }
}
