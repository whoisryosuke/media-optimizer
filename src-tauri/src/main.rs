// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rimage::{config::{Codec, EncoderConfig, ResizeConfig, ResizeType}, error::EncoderError, image::{DynamicImage, ImageResult, RgbaImage}, Decoder, Encoder};
use std::fs::File;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(file_name: &str) -> String {

    let path = std::path::PathBuf::from(file_name);

    let decoder = Decoder::from_path(&path).unwrap();

    let image = decoder.decode().unwrap();

    // Setup image
    let file = File::create("output-frontend.png").expect("Failed to create file");

    // Configure encoder with settings like file type, compression, etc
    let resize_config = ResizeConfig::new(ResizeType::Lanczos3)
        .with_width(800)
        .with_height(600);

    let config = EncoderConfig::new(Codec::OxiPng)
        .with_quality(50.0)
        .expect("Quality didn't work");
        // .with_resize(resize_config);
    let encoder = Encoder::new(file, image).with_config(config);

    // Run encoder
    let result = encoder.encode().unwrap();
    
    println!("Image encoded");

    format!("Hello! You've been greeted from Rust!")
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
