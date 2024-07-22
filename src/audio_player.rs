use crate::sound_pack::LoadedSoundPack;
use crossbeam_channel::{bounded, Receiver, Sender};
use rodio::{OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use std::thread;

struct AudioThread {
    _stream: OutputStream,
    sink: Sink,
}

impl AudioThread {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let (_stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;
        Ok(AudioThread { _stream, sink })
    }
}

pub struct SoundPlayer {
    sender: Sender<u32>,
}

impl SoundPlayer {
    pub fn new(
        sound_pack: Arc<LoadedSoundPack>,
        num_threads: usize,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let (sender, receiver) = bounded(32);

        for _ in 0..num_threads {
            let thread_receiver = receiver.clone();
            let thread_sound_pack = Arc::clone(&sound_pack);

            thread::spawn(move || {
                let audio_thread = match AudioThread::new() {
                    Ok(at) => at,
                    Err(e) => {
                        eprintln!("Failed to create AudioThread: {:?}", e);
                        return;
                    }
                };

                Self::audio_thread_loop(thread_receiver, thread_sound_pack, audio_thread);
            });
        }

        Ok(SoundPlayer { sender })
    }

    fn audio_thread_loop(
        receiver: Receiver<u32>,
        sound_pack: Arc<LoadedSoundPack>,
        audio_thread: AudioThread,
    ) {
        for key_code in receiver {
            if let Err(e) = play_sound(&sound_pack, key_code, &audio_thread.sink) {
                eprintln!("Error playing sound: {:?}", e);
            }
        }
    }

    pub fn play(&self, key_code: u32) {
        if let Err(e) = self.sender.try_send(key_code) {
            eprintln!("Failed to send key code: {:?}", e);
        }
    }
}

fn play_sound(
    sound_pack: &LoadedSoundPack,
    key_code: u32,
    sink: &Sink,
) -> Result<(), Box<dyn std::error::Error>> {
    let key_name = key_code.to_string();
    let slice_path = sound_pack.slices_dir.join(format!("{}.ogg", key_name));

    if !slice_path.exists() {
        return Err(format!("Sliced sound file not found: {:?}", slice_path).into());
    }

    let file = BufReader::new(File::open(slice_path)?);
    let source = rodio::Decoder::new(file)?;
    sink.append(source);

    Ok(())
}

