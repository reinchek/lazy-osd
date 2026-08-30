mod palette;
mod vcp;

use ddc_hi::{Ddc, Display};
use eframe::egui;
use egui::{Button, Color32, RichText, Stroke, Vec2, Visuals};
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

// @formatter:on
type MonitorId = String;
type MpscSender = Sender<Command>;
type MpscReceiver = Receiver<Command>;
pub enum Command {
    RefreshMonitors,
    SetVcp(Vec<MonitorId>, u8, u16),
}

fn main() -> eframe::Result {
    let (tx, rx): (MpscSender, MpscReceiver) = mpsc::channel();

    // Start the DCC/CI thread
    let _ = thread::spawn(move || {
        for command in rx {
            match command {
                Command::RefreshMonitors => {}
                Command::SetVcp(monitors, vcp_code, vcp_value) => {

                    let all = Display::enumerate();
                    for mut display in all.into_iter().filter(|d| monitors.contains(&d.info.id)) {
                        display
                            .handle
                            .set_vcp_feature(vcp_code, vcp_value)
                            .expect(format!("Cannot set the {} feature.", vcp_code).as_str());
                    }
                }
            }
        }
    });

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
        ..Default::default()
    };

    eframe::run_native(
        "DDC Manager",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(palette::FRAPPE.visuals());
            Ok(Box::new(App::new(tx, Display::enumerate())))
        }),
    )
}

#[derive(Debug)]
pub struct MonitorStatus {
    index: usize,
    monitor_id: MonitorId,
    features: HashMap<u8, vcp::VcpRuntime>,
}

struct App {
    sync: bool,
    current_theme: Visuals,
    is_dark_mode: bool,
    displays: Vec<Display>,
    enabled_displays: Vec<String>,
    monitors: HashMap<MonitorId, MonitorStatus>,
    selected_group: Option<vcp::VcpGroup>,
    tx: MpscSender,
}

impl Default for App {
    fn default() -> Self {
        Self {
            sync: false,
            current_theme: palette::FRAPPE.visuals(),
            is_dark_mode: false,
            displays: Vec::new(),
            enabled_displays: Vec::new(),
            monitors: HashMap::new(),
            selected_group: None,
            tx: mpsc::channel().0,
        }
    }
}

impl App {
    fn new(tx: MpscSender, displays: Vec<Display>) -> Self {
        let mut app = Self {
            sync: false,
            current_theme: palette::FRAPPE.visuals(),
            is_dark_mode: false,
            displays,
            enabled_displays: Vec::new(),
            selected_group: Some(vcp::VcpGroup::Base),
            monitors: HashMap::new(),
            tx,
        };

        // Looping through all monitors' capabilities.
        // I need to check for N monitors because could be some differences between them.
        for (index, display) in &mut app.displays.iter_mut().enumerate() {
            // 1-indexed.
            let index = index + 1;

            // Add the current display data into app.monitors hashmap.
            app.monitors.insert(
                display.info.id.clone(),
                MonitorStatus {
                    index,
                    monitor_id: display.info.id.clone(),
                    features: HashMap::new(),
                },
            );

            // Loop through display's capabilites.
            for capability in display.handle.capabilities() {
                // Loop through capability's features.
                for (vcp_code, vcp_desc) in capability.vcp_features {
                    // Only looking for KNWON_VCP (common features) excluding the rest.
                    // So look at KNOWN_VCP as a filter, that filter out all uncommon features.
                    if let Some(_vcp_info) = vcp::KNOWN_FEATURES.iter().find(|vcp| vcp.code == vcp_code) {
                        let feature_value = display.handle.get_vcp_feature(vcp_code).unwrap();
                        app.monitors
                            .get_mut(&display.info.id)
                            .unwrap()
                            .features
                            .insert(
                                vcp_code,
                                vcp::VcpRuntime {
                                    allowed: vcp_desc.values.keys().map(|&k| k as u16).collect(),
                                    current: feature_value.value(),
                                    max: feature_value.maximum(),
                                },
                            );
                    }
                }
            }
        }

        // Enable the first display in `displays` list.
        app.enabled_displays
            .push(app.displays.first().unwrap().info.id.clone());

        app
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Top panel
        egui::Panel::top("top_panel")
            .resizable(false)
            .frame(
                egui::Frame::new()
                    .inner_margin(egui::Margin::symmetric(12, 10))
                    .fill(ui.visuals().window_fill()),
            )
            .min_size(30.0)
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.heading(
                        RichText::new("DCC/CI OSD Manager")
                            .strong()
                            .color(palette::TEXT_PRIMARY)
                            .size(18.0),
                    );

                    ui.horizontal(|ui| {
                        for display in &self.displays {
                            let is_active = self.enabled_displays.contains(&display.info.id);
                            let index = self.monitors[&display.info.id].index;

                            let model_name = display
                                .info
                                .clone()
                                .model_name
                                .unwrap_or("no_name".to_string());

                            let display_id = display.info.clone().id;

                            let button_display = ui.add(
                                Button::new(
                                    RichText::new(format!("🖵 {index} {model_name} )")).color(
                                        if is_active {
                                            palette::TEXT_PRIMARY
                                        } else {
                                            palette::TEXT_SECONDARY
                                        },
                                    ),
                                )
                                .fill(if is_active {
                                    palette::ACCENT.gamma_multiply(0.25)
                                } else {
                                    Color32::TRANSPARENT
                                })
                                .stroke(Stroke::new(
                                    1.0,
                                    if is_active {
                                        palette::ACCENT
                                    } else {
                                        palette::ACCENT_DIM
                                    },
                                ))
                                .min_size(Vec2::new(120.0, 32.0)),
                            );

                            // Add or remove enabled display.
                            // Enabled displays will be affected by vcp changes triggered by the user.
                            if button_display.clicked() {
                                // Check if the display is already enabled.
                                if self.enabled_displays.contains(&display.info.id) {
                                    let pos = self
                                        .enabled_displays
                                        .iter()
                                        .position(|a| *a == display_id)
                                        .unwrap();
                                    self.enabled_displays.remove(pos);
                                } else {
                                    self.enabled_displays.push(display_id);
                                }
                            }
                        }
                    });

                    ui.horizontal(|ui| {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                            let save_button = ui
                                .button("💾 Save settings")
                                .on_hover_text("Write the current status of all VCP settings to the monitor's non-volatile memory (EEPROM/NVRAM). Without this command, many monitors store changes made via DDC/CI only in volatile RAM - so when the monitor is turned off (or even put into standby mode, depending on the model), the settings are lost and revert to their previous values upon restart.");
                            if save_button.clicked() {
                                for display_id in &self.enabled_displays {
                                    let display = self.displays.iter_mut().find(|d| d.info.id == *display_id).unwrap();
                                    match display.handle.save_current_settings() {
                                        Ok(_) => { println!("Saved settings for display {}", display.info.id); },
                                        Err(e) => { eprintln!("Failed to save settings for display {}: {}", display.info.id, e); }
                                    };
                                }
                            }
                        });

                    });
                });
            });

        // Left panel
        // Group all vcp features using VcpGroup
        egui::Panel::left("left_panel")
            .resizable(false)
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    // Features' groups.
                    for group in vcp::VcpGroup::list() {
                        let available_width = ui.available_width();
                        let group_button = ui.add(
                            Button::new(
                                RichText::new(format!("{} {}", group.icon(), group.label())).color(
                                    if self.selected_group == Some(group) {
                                        palette::TEXT_PRIMARY
                                    } else {
                                        palette::TEXT_SECONDARY
                                    },
                                ),
                            )
                            .fill(if self.selected_group == Some(group) {
                                palette::ACCENT.gamma_multiply(0.25)
                            } else {
                                Color32::TRANSPARENT
                            })
                            .min_size(Vec2::new(available_width, 16.0)),
                        );
                        if group_button.clicked() {
                            self.selected_group.replace(group);
                        }
                    }
                });
                egui::Panel::bottom("bottom_panel").show(ui, |ui| {
                    // Dark/light switch buttons.
                    ui.horizontal_wrapped(|ui| {
                        let dark_response = ui
                            .add(egui::RadioButton::new(self.is_dark_mode, "🌙"))
                            .on_hover_text("Switch to dark theme");
                        let light_response = ui
                            .add(egui::RadioButton::new(!self.is_dark_mode, "☀"))
                            .on_hover_text("Switch to light theme");

                        if dark_response.clicked() {
                            self.is_dark_mode = true;
                            self.current_theme = palette::MOCHA.visuals();
                        }
                        if light_response.clicked() {
                            self.is_dark_mode = false;
                            self.current_theme = palette::FRAPPE.visuals();
                        }

                        if light_response.clicked() || dark_response.clicked() {
                            ui.set_visuals(self.current_theme.clone());
                        }
                    });
                });
            });

        // Central panel
        // ####################
        // Features widgets ###
        // ####################
        egui::CentralPanel::default().show(ui, |ui| {
            ui.vertical(|ui| {
                // Sync checkbox: synchronize values through all enabled monitors.
                let _sync = ui.checkbox(&mut self.sync, "Sync values across all enabled monitors");
                ui.add_space(10.0);

                if let Some(selected_group) = &self.selected_group {
                    for display in &self.displays {
                        if self.enabled_displays.contains(&display.info.id) {
                            let monitor = self.monitors.get_mut(&display.info.id).unwrap();
                            let feature_in_group = vcp::VcpInfo::by_group(*selected_group);

                            let display_collapsing_label = if self.sync {
                                format!(
                                    "Apply on {} enabled monitor{}",
                                    self.enabled_displays.len(),
                                    if self.enabled_displays.len() == 1 {
                                        ""
                                    } else {
                                        "s"
                                    }
                                )
                            } else {
                                format!(
                                    "#({}) {}",
                                    monitor.index,
                                    display
                                        .info
                                        .model_name
                                        .clone()
                                        .unwrap_or("<noname_display>".to_string())
                                )
                            };

                            egui::CollapsingHeader::new(display_collapsing_label)
                                .default_open(true)
                                .show(ui, |ui| {
                                    for feature in feature_in_group {
                                        if let Some(vcp_runtime) =
                                            &mut monitor.features.get_mut(&feature.code)
                                        {
                                            ui.label(
                                                RichText::new(feature.name)
                                                    .color(palette::TEXT_PRIMARY)
                                                    .strong(),
                                            );
                                            match feature.kind {
                                                vcp::VcpKind::Continuous => {
                                                    let slider = ui.add(egui::Slider::new(
                                                        &mut vcp_runtime.current,
                                                        0..=vcp_runtime.max,
                                                    ));
                                                    if slider.drag_stopped() {
                                                        // send message to channel to update dcc/ci (feature: 0x10) brightness value.
                                                        self.tx
                                                            .send(Command::SetVcp(
                                                                if self.sync {
                                                                    self.enabled_displays.clone()
                                                                } else {
                                                                    Vec::from([display
                                                                        .info
                                                                        .id
                                                                        .clone()])
                                                                },
                                                                feature.code,
                                                                vcp_runtime.current.clone(),
                                                            ))
                                                            .unwrap();
                                                    }
                                                }
                                                vcp::VcpKind::Enum => {
                                                    // @todo: show a select
                                                    let selected_opt_label = format!(
                                                        "{}",
                                                        vcp::VcpInfo::enum_label(
                                                            feature.code,
                                                            vcp_runtime.current
                                                        )
                                                        .unwrap_or(
                                                            format!("{}", vcp_runtime.current)
                                                                .as_str()
                                                        )
                                                    );
                                                    egui::ComboBox::from_id_salt((
                                                        "vcp_enum",
                                                        &monitor.monitor_id,
                                                        feature.code,
                                                    ))
                                                    .selected_text(format!(
                                                        "{}",
                                                        selected_opt_label
                                                    ))
                                                    .show_ui(ui, |ui| {
                                                        for &opt in &vcp_runtime.allowed {
                                                            let opt_label = format!(
                                                                "{}",
                                                                vcp::VcpInfo::enum_label(
                                                                    feature.code,
                                                                    opt
                                                                )
                                                                .unwrap_or(
                                                                    format!("{opt}").as_str()
                                                                )
                                                            );
                                                            let select = ui.selectable_label(
                                                                vcp_runtime.current == opt,
                                                                opt_label,
                                                            );
                                                            if select.clicked() || select.changed()
                                                            {
                                                                self.tx
                                                                    .send(Command::SetVcp(
                                                                        self.enabled_displays
                                                                            .clone(),
                                                                        feature.code,
                                                                        opt,
                                                                    ))
                                                                    .ok();
                                                            }
                                                        }
                                                    });
                                                }
                                                vcp::VcpKind::Action => {
                                                    let action_button = ui.button("Execute");
                                                    if action_button.clicked() {
                                                        self.tx.send(Command::SetVcp(
                                                            if self.sync {
                                                                self.enabled_displays.clone()
                                                            } else {
                                                                Vec::from([display.info.id.clone()])
                                                            },
                                                            feature.code,
                                                            vcp_runtime.current.clone(),
                                                        ));
                                                    }
                                                }
                                                vcp::VcpKind::ReadOnly => {
                                                    // @todo: show a label
                                                    ui.label(
                                                        egui::RichText::new(format!(
                                                            "{}",
                                                            vcp_runtime.current
                                                        ))
                                                        .weak(),
                                                    );
                                                }
                                            };
                                        }
                                    }
                                });

                            // If sync is enabled doesn't need to continue to loop,
                            // just break at the first cycle.
                            if self.sync {
                                break;
                            }
                        }
                    }
                }
            });
        });
    }
}
