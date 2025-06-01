# img-to-ico

A WebAssembly library to convert images to ICO format, built with Rust.

## Overview

img-to-ico is a lightweight utility that allows you to convert various image formats to Windows ICO format directly in the browser using WebAssembly technology.

## Features

- Convert common image formats (PNG, JPEG, etc.) to ICO format
- Browser-based conversion with no server uploads needed
- Automatic resizing of images to optimal ICO dimensions
- Square image validation to ensure proper ICO output
- Fast performance through WebAssembly

## Requirements

- Images must be square (equal width and height)
- Maximum image size of 256x256 pixels (larger images will be automatically resized)
- Supported input formats include PNG, JPEG, and other common image formats

## Usage

### Web Interface

1. Open the provided HTML page in your browser
2. Click the file input to select an image
3. The conversion happens automatically
4. The resulting ICO image will be displayed on the page

### JavaScript API

```javascript
import init, { convert_to_ico } from "./pkg/img_to_ico.js";

// Initialize the WASM module
await init();

// Convert an image to ICO format
try {
  // Assuming you have a Uint8Array of image data
  const imageData = new Uint8Array(...);
  
  // Convert to ICO
  const icoData = convert_to_ico(imageData);
  
  // Use the resulting ICO data
  const blob = new Blob([icoData], { type: 'image/x-icon' });
  const url = URL.createObjectURL(blob);
  
  // Now you can use the URL for the ICO image
  imageElement.src = url;
} catch (error) {
  console.error('Conversion error:', error);
}
```

## Building from Source

1. Ensure you have Rust and wasm-pack installed
2. Clone this repository
3. Build the WebAssembly module:
   ```
   wasm-pack build --target web
   ```
4. The compiled WASM module will be available in the `pkg` directory

## License

This project is licensed under either of:
- MIT License
- Apache License, Version 2.0

at your option.

## Author

Created by [sshcrack](https://github.com/sshcrack)