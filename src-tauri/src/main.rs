// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rimage::{config::{Codec, EncoderConfig, ResizeConfig, ResizeType}, error::EncoderError, image::{DynamicImage, ImageResult, RgbaImage}, Decoder, Encoder};
use std::{ffi::OsStr, fs::File};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(file_name: &str, quality: i32) -> String {

    println!("filename: {}", file_name);
    let path = std::path::PathBuf::from(file_name);

    let default_file_type = OsStr::new("png");
    let file_type = path.extension().or(Some(default_file_type)).expect("No file extension found").to_str().expect("Couldn't convert to string");

    // Get file name from path.
    let output_file_base_name = &path.file_stem().expect("No file name").to_str().expect("Couldn't convert filename to string");
    let output_file_name = format!("{output_file_base_name}.{file_type}");

    // Load image data
    let decoder = Decoder::from_path(&path).unwrap();
    let image = decoder.decode().unwrap();

    // Setup new image file
    let file = File::create(output_file_name).expect("Failed to create file");

    // Configure encoder with settings like file type, compression, etc
    let resize_config = ResizeConfig::new(ResizeType::Lanczos3)
        .with_width(800)
        .with_height(600);

    let codec = match file_type {
        "png" => Codec::OxiPng,
        "jpg" => Codec::MozJpeg,
        _ => Codec::OxiPng
    };

    let config = EncoderConfig::new(codec)
        .with_quality(quality as f32)
        .expect("Quality didn't work");
        // .with_resize(resize_config);
    let encoder = Encoder::new(file, image).with_config(config);

    // Run encoder and generate image
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
