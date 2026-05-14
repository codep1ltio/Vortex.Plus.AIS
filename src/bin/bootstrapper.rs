#![windows_subsystem = "windows"]

use eframe::egui;
use std::process::Child;
use std::sync::mpsc::{self, Receiver};

mod launcher;
use launcher::launcher::*;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([700.0, 400.0])
            .with_resizable(true),

        ..Default::default()
    };

    load_mods_folder().unwrap();

    eframe::run_native(
        "Vortex2+2 AIS Launcher",
        options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}

struct App {
    vortex_process: Option<Child>,
    status: String,
    check_updates_on_launch: bool,
    launched: bool,
    new_update_available: bool,
    close_launcher: bool,
    logs: Vec<String>,
    launch_rx: Option<Receiver<LaunchMsg>>,
}

enum LaunchMsg {
    Log(String),
    Done(Result<Child, String>),
}

impl Default for App {
    fn default() -> Self {
        Self {
            status: "".to_string(),
            check_updates_on_launch: true,
            launched: false,
            new_update_available: false,
            launch_rx: None,
            close_launcher: false,
            vortex_process: None,

            logs: vec![
                "Bootstrapper Loaded.".to_string(),
                "-------------------------------".to_string(),
                "If you haven't yet, please join our discord!".to_string(),
                "We need your help and your suggestions!".to_string(),
                "discord.gg/E9y6WfEdPW".to_string(),
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
        self.check_updates_on_launch = true;
        self.status = "Checking for updates...".to_string();
        self.push_log("Update check started.");
        if self.new_update_available {}
    }

    fn open_mods_folder(&mut self) {
        open_mods_folder()
            .map(|_| self.push_log("Opened mods folder"))
            .unwrap_or_else(|e: std::io::Error| {
                self.push_log(format!("Error while opening mods folder: {e}"));
            });
    }

    fn start_launch(&mut self) {
        if self.launched {
            return;
        }

        self.launched = true;
        self.status = "Vortex Active".to_string();

        let should_check_updates = self.check_updates_on_launch;
        let (tx, rx) = mpsc::channel();
        self.launch_rx = Some(rx);

        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().expect("tokio runtime");

            let result: Result<Child, String> = rt.block_on({
            let tx = tx.clone();

            async move {
                if should_check_updates {
                    let _ = tx.send(LaunchMsg::Log("Fetching for updates...".into()));

                    let release = match get_latest_release().await {
                        Ok(release) => release,
                        Err(err) => {
                            return Err(format!(
                                "Something went wrong during update fetching\nError => {err}\n----\nDisable update on launch if it's not important."
                            ));
                        }
                    };

                    let current_version = env!("CARGO_PKG_VERSION").trim_start_matches('v');
                    let latest_version = release.tag_name.trim_start_matches('v');

                    if latest_version != current_version {
                        let _ = tx.send(LaunchMsg::Log(
                            "New update available. Vortex will automatically boot after update.".into(),
                        ));
                        let _ = tx.send(LaunchMsg::Log("Downloading new update...".into()));

                        let zip_url = match release.assets.iter().find(|a| a.name == "vortex.zip") {
                            Some(asset) => asset.browser_download_url.clone(),
                            None => {
                                return Err("Missing vortex.zip in GitHub release".to_string());
                            }
                        };

                        if let Err(err) = install_update(&zip_url).await {
                            return Err(format!("Update install failed: {err}"));
                        }
                    } else {
                        let _ = tx.send(LaunchMsg::Log("No new updates.".into()));
                    }

                    launch_vortex().map_err(|e| e.to_string())
                } else {
                    launch_vortex().map_err(|e| e.to_string())
                }
            }
        });

            let _ = tx.send(LaunchMsg::Done(result));
        });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut process_log: Option<String> = None;
        let mut process_closed = false;

        if let Some(child) = self.vortex_process.as_mut() {
            match child.try_wait() {
                Ok(Some(status)) => {
                    process_log = Some(format!("Vortex closed: {status}"));
                    self.status = "Vortex AIS is ready for launch".to_string();
                    process_closed = true;
                }
                Ok(None) => {}
                Err(err) => {
                    process_log = Some(format!("Process check failed: {err}"));
                    process_closed = true;
                }
            }
        }

        if let Some(msg) = process_log {
            self.push_log(msg);
        }

        if process_closed {
            self.vortex_process = None;
            self.launched = false;
        }

        if let Some(rx) = self.launch_rx.as_ref() {
            let mut clear_rx = false;
            let mut pending_msgs = Vec::new();

            while let Ok(msg) = rx.try_recv() {
                pending_msgs.push(msg);
            }

            for msg in pending_msgs {
                match msg {
                    LaunchMsg::Log(line) => self.push_log(line),
                    LaunchMsg::Done(Ok(child)) => {
                        self.push_log("Vortex launched.");
                        self.vortex_process = Some(child);

                        if self.close_launcher {
                            std::process::exit(0);
                        }

                        clear_rx = true;
                    }
                    LaunchMsg::Done(Err(err)) => {
                        self.status = format!("Launch failed: {err}");
                        self.push_log(format!("Launch error: {err}"));
                        self.push_log("If you're unsure of the cause, feel free to contact us in our discord.");
                        self.launched = false;
                        clear_rx = true;
                    }
                }
            }

            if clear_rx {
                self.launch_rx = None;
            }
        }

        egui::SidePanel::left("side_panel")
            .resizable(false)
            .default_width(300.0)
            .show(ctx, |ui| {
                ui.add_space(8.0);
                ui.heading("Options");
                ui.add_space(8.0);

                ui.monospace("On launch options");
                ui.checkbox(&mut self.check_updates_on_launch, "Check for updates");
                ui.checkbox(&mut self.close_launcher, "Close bootstrapper");

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
                        egui::Button::new("Launch Vortex")
                            .rounding(15.0)
                            .min_size(egui::vec2(220.0, 40.0)),
                    );

                    if launch_button.clicked() {
                        self.start_launch();
                    }

                    ui.small(format!(
                        "{}",
                        (if self.check_updates_on_launch {
                            ""
                        } else {
                            "Update checking is disabled."
                        })
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
