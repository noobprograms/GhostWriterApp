use tauri::AppHandle;
use crate::video_process::ffmpeg;
use crate::video_process::renderer;
use crate::video_process::script;
use std::path::PathBuf;
fn get_temp_dir() -> PathBuf {
    let mut dir = std::env::temp_dir();
    dir.push("ghostwriter_frames");

    std::fs::create_dir_all(&dir).unwrap();

    dir
}
fn is_image(input: &str) -> bool {
    input.ends_with(".png")
        || input.ends_with(".jpg")
        || input.ends_with(".jpeg")
}
pub async fn process_video(
    app: AppHandle,
    input: &str,
    script_str: &str,
    output: &str,
    should_draw_glow: &bool,
) -> anyhow::Result<Vec<u8>> {
    println!("Starting video processing with input: {}, script: {}, output: {}", input, script_str, output);
    //if input is an image convert that to video with ffmpeg and just return 
    let mut temp_video_path = input;
    if is_image(input) {
        println!("Input is an image, converting to video...");

        let temp_video = ffmpeg::image_to_video(&app, input).await?;
        temp_video_path = &temp_video;
        // read the temp video, return bytes and remove the temp file
        let data = std::fs::read(&temp_video_path)?;
        if std::path::Path::new(&temp_video_path).exists() {
            let _ = std::fs::remove_file(&temp_video_path);
        }
        return Ok(data);
    }
    std::fs::create_dir_all("../frames")?;
    println!("Extracting frames from video...{}", temp_video_path);
    ffmpeg::extract_frames(&app, temp_video_path).await?;
    println!("Frames extracted successfully.");
    let script = script::parse_script(script_str);
    println!("Script parsed successfully: {:?}", script);
    const MY_FONT: &[u8] = include_bytes!("../../fonts/Rockybilly.ttf");
    let font = ab_glyph::FontRef::try_from_slice(MY_FONT)?;

    let mut frames: Vec<_> = std::fs::read_dir("../frames")?
        .filter_map(|f| f.ok())
        .collect();
    println!("Found {} frames.", frames.len());
    frames.sort_by_key(|f| f.path());

    let fps = 30.0;

    for (i, frame) in frames.iter().enumerate() {
        let path = frame.path();

        let mut img = image::open(&path)?.to_rgba8();

        let time = i as f32 / fps;

        renderer::render_frame(&mut img, &script, time, &font, &should_draw_glow);
        println!("Rendered frame {} at time {:.2}s", i, time);
        img.save(&path)?;
    }
    println!("All frames rendered successfully, assembling video...");
    ffmpeg::assemble_video(&app, output).await?;
    // read assembled output into memory
    let data = std::fs::read(output)?;
    // Clean up frames and output file
    std::fs::remove_dir_all("../frames")?;
    if std::path::Path::new(output).exists() {
        let _ = std::fs::remove_file(output);
    }
    println!("Video assembled successfully.");
    Ok(data)
}