use crate::tui::Tui;
use crate::my_player::MyPlayer;
// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: u32,
    
    #[serde(skip)]
    player: MyPlayer,
    #[serde(skip)]
    tui: Tui,
    
}

impl Default for TemplateApp {
    fn default() -> Self {

        //this guy needs to live
        let mut player=MyPlayer::build();
        let mut tui = Tui::build();
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 100,
            player: player,
            tui: tui,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }
}

impl eframe::App for TemplateApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        self.player.adjust_volume(self.value);
        egui::Panel::top("top_panel").show_inside(ui, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ui.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);
                ui.menu_button("Open a song (GUI)", |ui| {
                    let paths=self.tui.get_paths().clone();
                    for path in paths {
                        if ui.button(&*path).clicked(){
                            if self.tui.update_cur_path(&path){
                                self.player.queue_song(&self.tui);
                            }
                            println!("{}",&path);
                        }
                    }
                });
                if ui.button("Open a song (TUI)").clicked() {
                    
                    let mut choice="".to_string();
                    while choice != "song".to_string() {
                        choice = self.tui.get_input();
                        println!("{}",choice);
                        match choice.as_str() {
                            "q" => {break;},
                            "s" => {self.player.skip(); continue;},
                            "song" => {self.player.queue_song(&self.tui); continue;},
                            "directory" => {continue},
                            &_ => {break;},
                        }
                    }
                }
                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("BetterVLC");
            


            ui.add(egui::Slider::new(&mut self.value, 0..=100).text("volume"));
            if ui.button("Increment").clicked() {
                self.value += 1;
            }

            ui.separator();
            
            ui.horizontal(|ui| {
                ui.label("Now playing: ");
                ui.label(&self.tui.song_choice);
            });
            ui.add(egui::github_link_file!(
                "https://github.com/jamesw6/BetterVLC",
                "Source code."
            ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
