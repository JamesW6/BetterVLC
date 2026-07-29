use std::fs::{File, metadata};
mod my_player;
mod tui;
use crate::tui::Tui;
use crate::my_player::MyPlayer;
fn main() {
    //this guy needs to live
    let mut player=MyPlayer::build();
    let mut tui = Tui::build();
    loop {
        let choice = tui.get_input();
        match choice.as_str() {
            "q" => {break;}
            "s" => {player.skip(); continue;}
            "song" => {player.queue_song(&tui.song_choice);}
        }
    }
}
fn print_type<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}
