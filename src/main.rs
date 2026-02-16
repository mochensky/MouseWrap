use enigo::*;
use std::io::{self, BufRead};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Enter desired edge buffer size in pixels");
    println!("↓ ");

    let stdin = io::stdin();
    let mut buffer = String::new();

    stdin.lock()
        .read_line(&mut buffer)
        .expect("Failed to read input");

    let edge_buffer: i32 = match buffer.trim().parse() {
        Ok(num) if num >= 0 => num,
        _ => {
            println!("Invalid value → using default: 1 pixel");
            1
        }
    };

    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    let (screen_width, screen_height) = enigo.main_display().unwrap();

    println!("\nScreen resolution: {} × {}", screen_width, screen_height);
    println!("Edge buffer: {} px", edge_buffer);
    println!("Press Ctrl+C to exit.\n");

    loop {
        let (x, y) = match enigo.location() {
            Ok(pos) => pos,
            Err(_) => continue,
        };

        let mut new_x = x;
        let mut new_y = y;
        let mut teleported = false;

        // Left
        if x <= edge_buffer {
            new_x = screen_width - edge_buffer - 1;
            teleported = true;
        }

        // Right
        else if x >= screen_width - edge_buffer - 1 {
            new_x = edge_buffer;
            teleported = true;
        }

        // Top
        if y <= edge_buffer {
            new_y = screen_height - edge_buffer - 1;
            teleported = true;
        }

        // Bottom
        else if y >= screen_height - edge_buffer - 1 {
            new_y = edge_buffer;
            teleported = true;
        }

        if teleported {
            let _ = enigo.move_mouse(new_x, new_y, Coordinate::Abs);
            // thread::sleep(Duration::from_millis(1));
        }

        thread::sleep(Duration::from_millis(1));
    }
}