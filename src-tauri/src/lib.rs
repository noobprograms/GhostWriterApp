// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
mod video_process;

#[tauri::command]
async fn process_video(
    app: tauri::AppHandle,
    input: String,
    script: String,
    output: String,
    should_draw_glow: bool,
) -> Result<String, String> {
    video_process::pipeline::process_video(app, &input, &script, &output, &should_draw_glow)
        .await
        .map_err(|e| e.to_string())?;

    Ok("done".into())
}

pub fn run() {
    tauri::Builder::default()
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![process_video])
        .run(tauri::generate_context!())
        .expect("error running tauri app");
}

