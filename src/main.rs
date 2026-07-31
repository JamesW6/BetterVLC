mod app;
use crate::app::TemplateApp;
mod tui;
use crate::tui::Tui;
mod my_player;
use crate::my_player::MyPlayer;
fn main() {
    //this guy needs to live
    let mut player=MyPlayer::build();
    let mut tui = Tui::build();

    //make gui
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]),
        ..Default::default()
    };
    let _ = eframe::run_native(
        "BetterVLC",
        native_options,
        Box::new(|cc| Ok(Box::new(TemplateApp::new(cc)))),
    );
}
// fn print_type<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>());
// }
