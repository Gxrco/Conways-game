use minifb::{Window, WindowOptions};
use std::time::Duration;
use std::thread;
use rand::Rng;

mod framebuffer;
mod color;
mod bmp;
use framebuffer::Framebuffer;
use color::Color;

fn main() {
    let window_width = 800;
    let window_height = 800;

    let framebuffer_width = 100;
    let framebuffer_height = 100;

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Conway's Game of Life - Random Patterns",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Initialize the game with random patterns
    initialize_game(&mut framebuffer);

    // Main loop
    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        update_framebuffer(&mut framebuffer);

        let scaled_buffer = scale_buffer(&framebuffer, window_width, window_height);
        window
            .update_with_buffer(&scaled_buffer, window_width, window_height)
            .unwrap();

        thread::sleep(Duration::from_millis(50));  // Faster animation
    }
}

fn initialize_game(framebuffer: &mut Framebuffer) {
    framebuffer.set_current_color(0xFFFFFF);  // Set current color to white

    let patterns = vec![
        vec![(0, 1), (1, 1), (2, 1)],  // Blinker
        vec![(0, 0), (1, 0), (2, 0), (1, 1), (2, 1), (3, 1)],  // Toad
        vec![(0, 0), (1, 0), (0, 1), (1, 1), (2, 2), (3, 2), (2, 3), (3, 3)],  // Beacon
        vec![
            (0, 2), (0, 3), (0, 4), (0, 8), (0, 9), (0, 10),
            (2, 0), (3, 0), (4, 0), (8, 0), (9, 0), (10, 0),
            (2, 5), (3, 5), (4, 5), (8, 5), (9, 5), (10, 5),
            (2, 7), (3, 7), (4, 7), (8, 7), (9, 7), (10, 7),
            (2, 12), (3, 12), (4, 12), (8, 12), (9, 12), (10, 12),
            (5, 2), (5, 3), (5, 4), (5, 8), (5, 9), (5, 10),
            (7, 2), (7, 3), (7, 4), (7, 8), (7, 9), (7, 10),
            (12, 2), (12, 3), (12, 4), (12, 8), (12, 9), (12, 10)
        ],  // Pulsar
    ];

    let mut rng = rand::thread_rng();
    for _ in 0..10 {  // Add 10 random patterns
        let pattern = &patterns[rng.gen_range(0..patterns.len())];
        let x_offset = rng.gen_range(0..framebuffer.width - 15);
        let y_offset = rng.gen_range(0..framebuffer.height - 15);
        
        for &(x, y) in pattern {
            framebuffer.point(x + x_offset, y + y_offset);
        }
    }
}

fn update_framebuffer(framebuffer: &mut Framebuffer) {
    let width = framebuffer.width;
    let height = framebuffer.height;
    let mut new_buffer = framebuffer.buffer.clone();

    for y in 0..height {
        for x in 0..width {
            let idx = (y * width + x) as usize;
            let cell = framebuffer.buffer[idx];
            let alive_neighbors = count_alive_neighbors(framebuffer, x, y);

            let new_color = match (cell, alive_neighbors) {
                (Color { r: 255, g: 255, b: 255 }, n) if n < 2 => Color::new(0, 0, 0),  // Underpopulation
                (Color { r: 255, g: 255, b: 255 }, n) if n == 2 || n == 3 => Color::new(255, 255, 255),  // Survival
                (Color { r: 255, g: 255, b: 255 }, n) if n > 3 => Color::new(0, 0, 0),  // Overpopulation
                (Color { r: 0, g: 0, b: 0 }, n) if n == 3 => Color::new(255, 255, 255),  // Reproduction
                (current_color, _) => current_color,  // No change
            };

            new_buffer[idx] = new_color;
        }
    }

    framebuffer.buffer = new_buffer;
}

fn count_alive_neighbors(framebuffer: &Framebuffer, x: u32, y: u32) -> u32 {
    let width = framebuffer.width;
    let height = framebuffer.height;
    let mut count = 0;

    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }

            let nx = x.wrapping_add(dx as i32 as u32);
            let ny = y.wrapping_add(dy as i32 as u32);

            if nx < width && ny < height {
                let idx = (ny * width + nx) as usize;
                let neighbor = framebuffer.buffer[idx];
                if neighbor.r == 255 && neighbor.g == 255 && neighbor.b == 255 {
                    count += 1;
                }
            }
        }
    }

    count
}

fn scale_buffer(framebuffer: &Framebuffer, window_width: usize, window_height: usize) -> Vec<u32> {
    let mut scaled_buffer = vec![0; window_width * window_height];
    let scale_x = window_width as f32 / framebuffer.width as f32;
    let scale_y = window_height as f32 / framebuffer.height as f32;

    for y in 0..window_height {
        for x in 0..window_width {
            let src_x = (x as f32 / scale_x) as usize;
            let src_y = (y as f32 / scale_y) as usize;
            let src_idx = src_y * framebuffer.width as usize + src_x;
            let dst_idx = y * window_width + x;
            let color = framebuffer.buffer[src_idx];
            scaled_buffer[dst_idx] = ((color.r as u32) << 16) | ((color.g as u32) << 8) | (color.b as u32);
        }
    }

    scaled_buffer
}
