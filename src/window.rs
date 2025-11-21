use minifb::{Window, WindowOptions};
use std::vec::Vec;

pub struct GridWindow {
    pub width: usize,
    pub height: usize,
    pub scale: usize,
    pub buffer: Vec<u32>,
    pub window: Window,
}

impl GridWindow {
    pub fn new(title: &str, width: usize, height: usize, scale: usize) -> Self {
        let window = Window::new(
            title,
            width,
            height,
            WindowOptions{
                resize: false,
                ..WindowOptions::default()
            },
        )
        .unwrap_or_else(|e| {
            panic!("Unable to open window: {}", e);
        });

        let buffer = vec![0u32; width * height];

        GridWindow {
            width,
            height,
            scale,
            buffer,
            window,
        }
    }

    // draw a cell at grid cords (x, y) with color
    pub fn draw(&mut self, cell_x: usize, cell_y: usize, color: u32) {
        let start_x = cell_x * self.scale;
        let start_y = cell_y * self.scale;

        for dy in 0..self.scale {
            for dx in 0..self.scale {
                let px = start_x + dx;
                let py = start_y + dy;

                if px < self.width && py < self.height {
                    self.buffer[py * self.width + px] = color;
                }
            }
        }
    }

    // overwrite whole buffer with color
    pub fn clear(&mut self, color: u32) {
        self.buffer.fill(color);
    }

    // update window with frame buffer
    pub fn update(&mut self) {
        self.window.update_with_buffer(&self.buffer, self.width, self.height).unwrap();
    }

    // check if window is still open
    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }

    // check if a key is pressed
    pub fn is_key_down(&self, key: minifb::Key) -> bool {
        self.window.is_key_down(key)
    }

    // get mouse pos (grid coordinates)
    pub fn mouse_pos(&self) -> Option<(usize, usize)> {
        self.window
            .get_mouse_pos(minifb::MouseMode::Discard)
            .map(|(x, y)| (x as usize / self.scale, y as usize / self.scale))
    }

    // check if a mouse button is pressed
    pub fn mouse_down(&self, button: minifb::MouseButton) -> bool {
        self.window.get_mouse_down(button)
    }
}