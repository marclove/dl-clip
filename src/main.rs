extern crate regex;
use clap::Parser;
use regex::Regex;
use std::ffi::OsStr;
use which::which;

#[derive(Parser, Default, Debug)]
#[clap(
    name = "dl-clip",
    about = "Download videos in a format appropriate for social media.\n\nYou may clip the source video by providing start and end timestamps. Uses yt-dlp and ffmpeg."
)]
struct Arguments {
    url: String,
    #[clap(short, long)]
    start: Option<String>,
    #[clap(short, long)]
    end: Option<String>,
}

fn find_ffmpeg() -> Option<String> {
    match which("ffmpeg") {
        Ok(path) => Some(path.to_str().unwrap().to_string()),
        Err(_) => {
            panic!("Unable to find ffmpeg binary. Please install ffmpeg and add it to your PATH.")
        }
    }
}

fn find_ytdlp() -> Option<String> {
    match which("yt-dlp") {
        Ok(path) => Some(path.to_str().unwrap().to_string()),
        Err(_) => {
            panic!("Unable to find yt-dlp binary. Please install yt-dlp and add it to your PATH.")
        }
    }
}

fn download_video(ytdlp_path: &str, url: &str) -> Option<String> {
    let output = std::process::Command::new(ytdlp_path)
        // Download the video in codecs that work with most social media sites
        .args(&["-S", "vcodec:h264,res,acodec:m4a", url])
        .output()
        .expect("Failed to execute yt-dlp.");

    if output.status.success() {
        println!("Success!");
        let output_str = String::from_utf8_lossy(&output.stdout);
        for line in output_str.lines() {
            println!("{}", line);

            if line.starts_with("[Merger] Merging formats into") {
                // Extract the printed filename from yt-dlp's output
                let parts: Vec<&str> = line.split('"').collect();
                if parts.len() > 1 {
                    return Some(parts[1].to_string());
                }
            }
            if line.ends_with("has already been downloaded") {
                let re = Regex::new(r"\[download\] (.*?) has already been downloaded").unwrap();
                if let Some(caps) = re.captures(line) {
                    return Some(caps.get(1).unwrap().as_str().to_string());
                }
            }
        }
    } else {
        println!(
            "Failed to download video. yt-dlp error: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    // If there's no output but the command succeeded. Won't happen but handles compiler warning.
    None
}

fn clip_video(
    ffmpeg_path: &str,
    video_filename: &str,
    start: &Option<String>,
    end: &Option<String>,
) -> Option<String> {
    // Confirm the video file exists
    let path = std::path::Path::new(video_filename);
    if !path.exists() {
        panic!("Video file does not exist.");
    }

    let file_stem = path.file_stem().and_then(OsStr::to_str).unwrap_or_default();
    let extension = path.extension().and_then(OsStr::to_str).unwrap_or_default();
    let output_file_name = format!("{} [Processed].{}", file_stem, extension);

    let mut command = std::process::Command::new(ffmpeg_path);
    command.args(&[
        "-nostdin",
        "-i",
        video_filename,
        "-c:v",
        "libx264",
        "-profile:v",
        "baseline",
        "-level",
        "3.0",
        "-pix_fmt",
        "yuv420p",
        "-preset",
        "slow",
        "-b:v",
        "1500k",
        "-maxrate",
        "1500k",
        "-bufsize",
        "3000k",
        "-c:a",
        "aac",
        "-b:a",
        "128k",
        "-ac",
        "2",
        "-ar",
        "44100",
        "-y",
    ]);
    if let Some(start_time) = start {
        command.args(&["-ss", start_time]);
    }
    if let Some(end_time) = end {
        command.args(&["-to", end_time]);
    }
    command.arg(&output_file_name);

    let output = command.status().expect("Failed to execute ffmpeg.");

    if output.success() {
        return Some(output_file_name.to_string());
    } else {
        println!(
            "Failed to clip video. ffmpeg returned error code: {}",
            output
        );
    }
    // If there's no output but the command succeeded. Won't happen but handles compiler warning.
    None
}

fn main() {
    let args = Arguments::parse();

    println!("Downloading video…");
    let video_filename = download_video(&find_ytdlp().unwrap(), &args.url).unwrap();

    println!("Processing video…");
    let output_filename = clip_video(
        &find_ffmpeg().unwrap(),
        &video_filename,
        &args.start,
        &args.end,
    )
    .unwrap();

    println!("File processed: {}", output_filename);
}
