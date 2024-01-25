DL-CLIP
---

Download videos in a format appropriate for social media.

You may clip the source video by providing [start and end timestamps](https://ffmpeg.org//ffmpeg-utils.html#time-duration-syntax). Uses yt-dlp and ffmpeg.

---

Installation on macOS:

1. Install Rust
2. Clone the repo
3. Open the repo directory
4. `cargo build --release`
5. `mv target/release/dl-clip /usr/local/bin/` (or to a directory in your PATH of your choosing)

---

```
Usage: dl-clip [OPTIONS] <URL>

Arguments:
  <URL>

Options:
  -s, --start      <START TIME>
  -e, --end        <END TIME>
  -h, --help       Print help
```
