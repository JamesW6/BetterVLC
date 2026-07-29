use rodio::{DeviceSinkBuilder, Decoder, Player, MixerDeviceSink};
use std::fs::{File};
pub struct MyPlayer{
    sink_handle: MixerDeviceSink,
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
            sink_handle: sink_handle,
            player: player,
        }
    }

    pub fn queue_song(&mut self, song_path: &String){
        let file = File::open(song_path).unwrap();
        let decoder = Decoder::try_from(file).unwrap();
        self.player.append(decoder);
    }
    pub fn skip(&self){
        self.player.skip_one();
    }
}
