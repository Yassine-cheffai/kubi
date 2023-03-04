#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
mod resources;

struct Pod {
    is_selected: bool,
    name: String,
    status: String,
    ip: String,
    nominated_node: String,
    start_time: String,
}

fn main() -> Result<(), eframe::Error> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(640.0, 480.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Kubi - the kubernets desktop application",
        options,
        Box::new(|_cc| Box::new(Cluster::default())),
    )
}

struct Cluster {
    selected_resource: SelectedResource,
    pods: Vec<Pod>,
}

impl Default for Cluster {
    fn default() -> Self {
        let mut cluster = Cluster {
            selected_resource: SelectedResource::None,
            pods: vec![],
        };
        let result_pods = resources::get_pods();
        match result_pods {
            Ok(pods) => {
                for pod in pods {
                    cluster.pods.push(Pod {
                        is_selected: false,
                        name: pod["name"].clone(),
                        status: pod["status"].clone(),
                        ip: pod["ip"].clone(),
                        nominated_node: pod["nominated_node"].clone(),
                        start_time: pod["start_time"].clone(),
                    })
                }
                return cluster;
            }
            Err(_) => return cluster,
        }
    }
}

enum SelectedResource {
    None,
    Pods,
    Services,
}
impl eframe::App for Cluster {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal_centered(|ui_buttons| {
                if ui_buttons.button("Pods").clicked() {
                    self.selected_resource = SelectedResource::Pods;
                };
                if ui_buttons.button("Services").clicked() {
                    self.selected_resource = SelectedResource::Services;
                };
            })
        });
        match self.selected_resource {
            SelectedResource::None => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("Welcome");
                });
            }
            SelectedResource::Pods => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("Pods");

                    egui::Grid::new("some_unique_id").show(ui, |ui| {
                        ui.label("NAME");
                        ui.label("STATUS");
                        ui.label("IP");
                        ui.label("NOMINATED NODE");
                        ui.label("START TIME");
                        ui.end_row();

                        for pod in self.pods.iter_mut() {
                            ui.checkbox(&mut pod.is_selected, &pod.name);
                            ui.label(&pod.status);
                            ui.label(&pod.ip);
                            ui.label(&pod.nominated_node);
                            ui.label(&pod.start_time);
                            ui.end_row();
                        }
                    });
                });
            }
            SelectedResource::Services => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("Services");
                });
            }
        }
    }
}
