#[cfg(feature = "export")]
use crate::engine::animation::base::AudioEvent;
#[cfg(feature = "export")]
use std::fs;
#[cfg(feature = "export")]
use std::io;
#[cfg(feature = "export")]
use std::path::Path;
#[cfg(feature = "export")]
use std::process::{ChildStdin, Command, Stdio};

pub fn sanitize_title(title: &str) -> String {
    title
        .trim()
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect::<String>()
        .split('_')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("_")
}

#[cfg(feature = "export")]
pub fn start_ffmpeg(
    title: &str,
    width: u32,
    height: u32,
    fps: u32,
    has_audio: bool,
) -> io::Result<Option<ChildStdin>> {
    let sanitized_title = sanitize_title(title);
    let output_file = if has_audio {
        format!("{}_temp.mkv", sanitized_title)
    } else {
        format!("{}.mkv", sanitized_title)
    };

    Command::new("ffmpeg")
        .args([
            "-y",
            "-f",
            "rawvideo",
            "-pixel_format",
            "rgba",
            "-video_size",
            &format!("{}x{}", width, height),
            "-framerate",
            &fps.to_string(),
            "-i",
            "-",
            "-c:v",
            "libx264rgb",
            &output_file,
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map(|mut child| child.stdin.take())
}

#[cfg(feature = "export")]
pub fn merge_audio(title: &str, audio_events: &[AudioEvent]) -> io::Result<()> {
    let sanitized_title = sanitize_title(title);
    let temp_video = format!("{}_temp.mkv", sanitized_title);
    let final_output = format!("{}.mkv", sanitized_title);
    let temp_path = Path::new(&temp_video);

    if audio_events.is_empty() {
        if temp_path.exists() {
            fs::rename(temp_video, final_output)?;
        }
        return Ok(());
    }

    if !temp_path.exists() {
        eprintln!(
            "Temporary video file {} not found. Skipping audio merge.",
            temp_video
        );
        return Ok(());
    }

    println!("Merging audio with FFmpeg...");

    let mut cmd = Command::new("ffmpeg");
    cmd.arg("-y").arg("-i").arg(&temp_video);

    for event in audio_events {
        cmd.arg("-i").arg(&event.path);
    }

    let mut inputs = String::new();
    let mut filter = String::new();
    let mut active_count = 0;

    for (i, event) in audio_events.iter().enumerate() {
        if event.path.is_empty() {
            continue;
        }

        if !Path::new(&event.path).exists() {
            eprintln!("Warning: Audio file not found: {}", event.path);
            continue;
        }

        let input_idx = i + 1;
        active_count += 1;
        let delay_ms = (event.start_time.as_secs_f64() * 1000.0) as i64;
        println!("  - Event: {}, volume: {:.2}, delay: {}ms, crop: {:.3}s", 
            event.path, event.volume, delay_ms, event.start_crop.as_secs_f64());

        filter.push_str(&format!(
            "[{}:a]atrim=start={:.3},adelay={}|{}[a{}];",
            input_idx,
            event.start_crop.as_secs_f64(),
            delay_ms,
            delay_ms,
            input_idx
        ));
        inputs.push_str(&format!("[a{}]", input_idx));
    }

    if active_count == 0 {
        fs::rename(temp_video, final_output).ok();
        return Ok(());
    }

    filter.push_str(&format!("{}amix=inputs={}[a]", inputs, active_count));

    cmd.args([
        "-filter_complex",
        &filter,
        "-map",
        "0:v",
        "-map",
        "[a]",
        "-c:v",
        "copy",
        "-c:a",
        "aac",
        "-shortest",
        &final_output,
    ]);

    let output = cmd.output()?;
    if !output.status.success() {
        eprintln!("FFmpeg audio merge failed!");
        eprintln!("Filter: {}", filter);
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
        return Ok(());
    }

    fs::remove_file(temp_video).ok();
    println!("Audio successfully merged: {}.mkv", sanitized_title);

    Ok(())
}
