// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;

use serde_json::Number;

#[tauri::command]
fn montage(
    selected_images: Vec<String>,
    is_transparent: bool,
    frame_width: Number,
    frame_height: Number,
    frames_per_row: Number,
    frames_per_column: Number,
    filter: String,
    output_directory: String,
    output_filename: String,
) -> bool {
    // montage * -geometry 128x128 -tile 5x5 -background transparent -filter Catrom spritesheet.png

    let geometry = format!("{}x{}", frame_width, frame_height);
    let tile = format!("{}x{}", frames_per_row, frames_per_column);
    let output = format!("{}/{}", output_directory, output_filename);

    let mut command = Command::new("montage");

    selected_images.iter().for_each(|image| {
        command.arg(image);
    });

    command
        .arg("-geometry")
        .arg(geometry)
        .arg("-tile")
        .arg(tile)
        .arg("-filter")
        .arg(filter);

    if is_transparent {
        command.arg("-background").arg("transparent");
    }

    command.arg(output);

    println!("{}", format!("{:?}", command).replace("\"", ""));

    let output = command
        .output()
        .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    return output.status.success();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![montage])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
