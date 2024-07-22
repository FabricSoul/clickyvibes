use crate::config::SoundPackConfig;
use std::collections::HashMap;
use std::fs;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct LoadedSoundPack {
    // pub config: SoundPackConfig,
    // pub sound_path: PathBuf,
    pub slices_dir: PathBuf,
}

pub fn load_sound_pack(pack_name: &str) -> Result<LoadedSoundPack, Box<dyn std::error::Error>> {
    let config_path = format!("src/audio/{}/config.toml", pack_name);
    println!("Loading sound pack from {}", config_path);
    let config = SoundPackConfig::load(&config_path)?;

    println!(
        "Loading sound_path src/audio/{}/{}",
        pack_name, config.sound
    );
    let sound_path = PathBuf::from(format!("src/audio/{}/{}", pack_name, config.sound));
    println!("Loaded sound path");
    if !sound_path.exists() {
        return Err(format!("Sound file not found: {:?}", sound_path).into());
    }

    let slices_dir = sound_path.with_file_name(format!("{}_slices", config.id));
    if !slices_dir.exists() {
        create_dir_all(&slices_dir)?;
        slice_audio_file(&sound_path, &slices_dir, &config.defines)?;
    }

    Ok(LoadedSoundPack {
        // config,
        // sound_path,
        slices_dir,
    })
}

fn slice_audio_file(
    input_path: &Path,
    output_dir: &Path,
    defines: &HashMap<String, [u64; 2]>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create output directory if it doesn't exist
    fs::create_dir_all(output_dir)?;

    for (key_code, &[start_time, duration]) in defines {
        let output_path = output_dir.join(format!("{}.ogg", key_code));

        // Construct FFmpeg command
        let status = Command::new("ffmpeg")
            .arg("-i")
            .arg(input_path)
            .arg("-ss")
            .arg(format!("{:.3}", start_time as f64 / 1000.0))
            .arg("-t")
            .arg(format!("{:.3}", duration as f64 / 1000.0))
            .arg("-c:a")
            .arg("libvorbis")
            .arg("-q:a")
            .arg("4")
            .arg("-y") // Overwrite output files without asking
            .arg(output_path)
            .status()?;

        if !status.success() {
            return Err(format!("FFmpeg command failed for key_code: {}", key_code).into());
        }
    }

    Ok(())
}
