use tauri_plugin_shell::ShellExt;
pub async fn assemble_video(
    app: &AppHandle,
    output_path: &str,
) -> anyhow::Result<()> {
    let sidecar = app.shell().sidecar("ffmpeg")?;

    sidecar
        .args([
            "-framerate",
            "30",
            "-i",
            "../frames/frame_%04d.png",
            "-c:v",
            "libx264",
            "-pix_fmt",
            "yuv420p",
            output_path,
        ])
        .output()
        .await?;

    Ok(())
}
use tauri::AppHandle;

pub async fn extract_frames(
    app: &AppHandle,
    input: &str,
) -> anyhow::Result<()> {
    let sidecar = app.shell().sidecar("ffmpeg")?;

    let output = sidecar
        .args([
            "-i",
            input,
            "-qscale:v",
            "2",
            "../frames/frame_%04d.png",
        ])
        .output()
        .await?;

    println!("FFmpeg output: {:?}", output);

    Ok(())
}
pub async fn image_to_video(
    app: &AppHandle,
    input: &str,
) -> anyhow::Result<String> {
    let sidecar = app.shell().sidecar("ffmpeg").map_err(|e| anyhow::anyhow!("Failed to create sidecar: {}", e))?;
    println!("Converting image {} to video...", input);
    let output = "../temp_input.mp4";
    println!("Output video will be saved as: {}", output);
    //if a temp video already exists. we have to overwrite it because ffmpeg will fail if the output file already exists
    if std::path::Path::new(output).exists() {
        std::fs::remove_file(output).map_err(|e| anyhow::anyhow!("Failed to remove existing temp video: {}", e))?;
    }
    sidecar
        .args([
            "-loop", "1",
            "-i", input,
            "-t", "5",                 // 5 seconds duration
            "-vf", "fps=30",
            "-pix_fmt", "yuv420p",
            output,
        ])
        .output()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to convert image to video: {}", e))?;
    
    println!("Image converted to video successfully: {}", output);
    Ok(output.to_string())
}