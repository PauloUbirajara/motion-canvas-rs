#[cfg(feature = "export")]
use std::process::{ChildStdin, Command, Stdio};
#[cfg(feature = "export")]
use std::io;

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
