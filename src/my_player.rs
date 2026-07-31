use rodio::{DeviceSinkBuilder, Decoder, Player, MixerDeviceSink};
use std::fs::{File};
use crate::tui::Tui;
pub struct MyPlayer{
    _sink_handle: MixerDeviceSink,
    player: Player,
}
impl MyPlayer{
    pub fn build() -> Self{

        // Get an OS-Sink handle to the default physical sound device.
        // Note that the playback stops when the sink_handle is dropped.
        let mut sink_handle = DeviceSinkBuilder::open_default_sink()
                .expect("open default audio stream");
        sink_handle.log_on_drop(false);
        // Note that the playback stops when the player is dropped
        let player=Player::connect_new(&sink_handle.mixer());
        Self {
            _sink_handle: sink_handle,
            player: player,
        }
    }

    pub fn queue_song(&mut self, tui: &Tui){
        let file = File::open(tui.song_choice.to_string()).unwrap();
        let decoder = Decoder::try_from(file).unwrap();
        self.player.append(decoder);
    }
    pub fn skip(&self){
        self.player.skip_one();
    }

    pub fn adjust_volume(&self, mut volume: u32){
        volume = std::cmp::min(volume, 100);
        let adjusted_volume = (volume as f32)/100.0;
        self.player.set_volume(adjusted_volume);
    }
}
