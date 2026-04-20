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
) -> anyhow::Result<()> {
    println!("Starting video processing with input: {}, script: {}, output: {}", input, script_str, output);
    //if input is an image convert that to video with ffmpeg and just return 
    if is_image(input) {
        println!("Input is an image, converting to video...");
        let temp_video = "temp_input.mp4";
        ffmpeg::image_to_video(&app, input).await?;
        return Ok(());
    }
    std::fs::create_dir_all("frames")?;

    ffmpeg::extract_frames(&app, input).await?;
    println!("Frames extracted successfully.");
    let script = script::parse_script(script_str);
    println!("Script parsed successfully: {:?}", script);
    const MY_FONT: &[u8] = include_bytes!("../../fonts/Rockybilly.ttf");
    let font = ab_glyph::FontRef::try_from_slice(MY_FONT)?;

    let mut frames: Vec<_> = std::fs::read_dir("frames")?
        .filter_map(|f| f.ok())
        .collect();
    println!("Found {} frames.", frames.len());
    frames.sort_by_key(|f| f.path());

    let fps = 30.0;

    for (i, frame) in frames.iter().enumerate() {
        let path = frame.path();

        let mut img = image::open(&path)?.to_rgba8();

        let time = i as f32 / fps;

        renderer::render_frame(&mut img, &script, time, &font);
        println!("Rendered frame {} at time {:.2}s", i, time);
        img.save(&path)?;
    }
    println!("All frames rendered successfully, assembling video...");
    ffmpeg::assemble_video(&app, output).await?;
    println!("Video assembled successfully.");
    Ok(())
}