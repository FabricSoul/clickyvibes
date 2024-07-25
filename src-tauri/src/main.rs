mod audio_player;
mod config;
mod key_mapping;
mod sound_pack;

use crate::audio_player::SoundPlayer;
use crate::key_mapping::key_to_code;
use rdev::{listen, Event};
use sound_pack::load_sound_pack;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sound_pack = Arc::new(load_sound_pack("test")?);
    let player = Arc::new(SoundPlayer::new(Arc::clone(&sound_pack), 5)?);

    let callback = move |event: Event| {
        if let rdev::EventType::KeyPress(key) = event.event_type {
            let key_code = key_to_code(&key);
            println!("Pressed {:?} (code: {})", key, key_code);
            player.play(key_code);
        }
    };

    if let Err(e) = listen(callback) {
        eprintln!("Error in event listener: {:?}", e);
    }

    Ok(())
}
