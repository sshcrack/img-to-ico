use std::io::Cursor;

use image::{imageops::FilterType, ImageFormat, ImageReader};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn convert_to_ico(data: &[u8], mime_type: &str) -> Result<Vec<u8>, JsValue> {
    let format = ImageFormat::from_mime_type(mime_type);
    if format.is_none() {
        return Err(JsValue::from_str("Unsupported image format"));
    }

    let mut reader = ImageReader::new(Cursor::new(data));
    reader.set_format(format.unwrap());

    let img = reader.decode();
    if let Err(e) = img {
        return Err(JsValue::from_str(&format!("Failed to decode image: {}", e)));
    }

    let mut img = img.unwrap();
    if img.width() != img.height() {
        return Err(JsValue::from_str(
            "Image dimensions must be square",
        ));
    }

    if img.width() > 256 {
        img = img.resize(256, 256, FilterType::Lanczos3);
    }

    let mut data = Cursor::new(Vec::new());
    if let Err(e) = img.write_to(&mut data, ImageFormat::Ico) {
        return Err(JsValue::from_str(&format!(
            "Failed to write image to ICO format: {}",
            e
        )));
    }

    Ok(data.into_inner())
}
