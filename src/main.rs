#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

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
    pods: Vec<Pod>,
    selected_resource: SelectedResource,
}

impl Default for Cluster {
    fn default() -> Self {
        let pods: Vec<Pod> = vec![
            Pod {
                name: "k8s-depl-back-6c68868d86-967xc".to_string(),
                status: "Running".to_string(),
                ip: "172.17.0.7".to_string(),
            },
            Pod {
                name: "k8s-depl-back-6c68868d86-967xc".to_string(),
                status: "Running".to_string(),
                ip: "172.17.0.7".to_string(),
            },
        ];

        Self {
            pods: pods,
            selected_resource: SelectedResource::None,
        }
    }
}

struct Pod {
    name: String,
    status: String,
    ip: String,
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
                    for pod in self.pods.iter() {
                        ui.label(format!(
                            "Pod: name '{}', status {}, ip {}",
                            pod.name, pod.status, pod.ip
                        ));
                    }
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
