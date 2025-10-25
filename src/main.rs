use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use zip::ZipArchive;
use image::{DynamicImage, ImageOutputFormat, GenericImageView, RgbaImage};

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let input = args.next().ok_or("Missing input path/URI")?;
    let output = args.next().ok_or("Missing output path")?;
    let size_str = args.next().unwrap_or_else(|| "256".to_string());
    let size: u32 = size_str.parse().map_err(|_| "Size must be an integer")?;

    let input_path = uri_to_path(&input)?;
    let file = File::open(&input_path)
    .map_err(|e| format!("Failed to open {:?}: {}", input_path, e))?;
    let mut archive = ZipArchive::new(file)
    .map_err(|e| format!("Not a valid ZIP archive: {}", e))?;

    let prefer_names = [
        "QuickLook/Thumbnail.png",
        "QuickLook/thumbnail.png",
        "QuickLook/Preview.png",
        "QuickLook/preview.png",
        "QuickLook/Preview.jpg",
        "QuickLook/preview.jpg",
        "QuickLook/Preview.jpeg",
        "QuickLook/preview.jpeg",
    ];

    let mut entries = Vec::new();
    for i in 0..archive.len() {
        if let Ok(file) = archive.by_index(i) {
            entries.push((i, file.name().to_string()));
        }
    }

    let mut found_index: Option<usize> = None;
    for pref in &prefer_names {
        let pref_low = pref.to_lowercase();
        if let Some((idx, _)) =
            entries.iter().find(|(_, name)| name.to_lowercase() == pref_low)
            {
                found_index = Some(*idx);
                break;
            }
    }

    if found_index.is_none() {
        if let Some((idx, _)) = entries.iter().find(|(_, name)| {
            let low = name.to_lowercase();
            low.starts_with("quicklook/")
            && (low.ends_with(".png")
            || low.ends_with(".jpg")
            || low.ends_with(".jpeg"))
        }) {
            found_index = Some(*idx);
        }
    }

    let found_index = found_index.ok_or("No suitable thumbnail found in QuickLook/ folder")?;

    let mut file = archive
    .by_index(found_index)
    .map_err(|e| format!("Failed to read zip entry: {}", e))?;
    let mut buf = Vec::with_capacity(file.size() as usize);
    file.read_to_end(&mut buf)
    .map_err(|e| format!("Failed to read thumbnail data: {}", e))?;

    let img = image::load_from_memory(&buf)
    .map_err(|e| format!("Failed to decode image: {}", e))?;

    let out_img = resize_to_max(img, size);

    let out_path = Path::new(&output);
    let mut out_file =
    File::create(out_path).map_err(|e| format!("Failed to create output file {:?}: {}", out_path, e))?;
    out_img
    .write_to(&mut out_file, ImageOutputFormat::Png)
    .map_err(|e| format!("Failed to write PNG: {}", e))?;

    Ok(())
}

fn uri_to_path(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    if input.starts_with("file://") {
        let rest = &input["file://".len()..];
        if rest.starts_with('/') {
            Ok(rest.to_string())
        } else if let Some(slash_pos) = rest.find('/') {
            Ok(rest[slash_pos..].to_string())
        } else {
            Err("Malformed file:// URI".into())
        }
    } else {
        Ok(input.to_string())
    }
}

/// Resize so both width and height ≤ max_dim, preserving aspect ratio.
fn resize_to_max(img: DynamicImage, max_dim: u32) -> DynamicImage {
    let (w, h) = img.dimensions();
    if w <= max_dim && h <= max_dim {
        return img;
    }

    // thumbnail() already preserves aspect ratio
    let thumb: RgbaImage = image::imageops::thumbnail(&img, max_dim, max_dim);
    DynamicImage::ImageRgba8(thumb)
}
