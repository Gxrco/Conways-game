use std::fs::File;
use std::io::{Write, BufWriter, Result};
use crate::color::Color;

const BMP_HEADER_SIZE: usize = 54;
const BMP_PIXEL_OFFSET: usize = 54;
const BMP_BITS_PER_PIXEL: usize = 32; 

pub fn write_bmp_file(
    file_path: &str,
    buffer: &[Color],  
    width: u32,
    height: u32,
) -> Result<()> {
    let mut file = BufWriter::new(File::create(file_path)?);

    write_bmp_header(&mut file, width, height)?;
    write_pixel_data(&mut file, buffer, width, height)?;

    Ok(())
}

fn write_bmp_header(
    file: &mut BufWriter<File>,
    width: u32,
    height: u32,
) -> Result<()> {
    let file_size = BMP_HEADER_SIZE as u32 + (width * height * 4);  // Total file size
    let mut header = vec![0u8; BMP_HEADER_SIZE];

    // BMP Header
    header[0] = b'B'; header[1] = b'M';  // Signature
    header[2..6].copy_from_slice(&file_size.to_le_bytes());  // File size
    header[10..14].copy_from_slice(&(BMP_PIXEL_OFFSET as u32).to_le_bytes());  // Pixel data offset
    header[14..18].copy_from_slice(&(40u32).to_le_bytes());  // Header size
    header[18..22].copy_from_slice(&width.to_le_bytes());  // Image width
    header[22..26].copy_from_slice(&height.to_le_bytes());  // Image height
    header[26..28].copy_from_slice(&(1u16).to_le_bytes());  // Planes
    header[28..30].copy_from_slice(&(BMP_BITS_PER_PIXEL as u16).to_le_bytes());  // Bits per pixel
    // Remaining bytes are initialized to 0 which represent no compression, and default resolution.

    file.write_all(&header)?;

    Ok(())
}

fn write_pixel_data(
    file: &mut BufWriter<File>,
    buffer: &[Color],
    width: u32,
    height: u32,
) -> Result<()> {
    let padding = (4 - (width * 4 % 4)) % 4;  // Calculate the necessary row padding
    for y in (0..height).rev() {
        for x in 0..width {
            let idx = (y * width + x) as usize;
            let color = &buffer[idx];
            let pixel = [color.b, color.g, color.r, 0];  // BGRA format with zero alpha
            file.write_all(&pixel)?;
        }
        file.write_all(&vec![0; padding as usize])?;  // Write padding bytes
    }

    Ok(())
}
