mod window;

use window::GridWindow;
use minifb::{Key, MouseButton};

fn main() {
    // window size = 800x600 pixels, each cell = 4x4 pixels
    let mut win = GridWindow::new("Gravity Sim", 800, 600, 4);

    while win.is_open() && !win.is_key_down(Key::Escape) {
        win.clear(0); // black

        // draw mouse drag
        if let Some((mx, my)) = win.mouse_pos() {
            if win.mouse_down(MouseButton::Left) {
                win.draw(mx, my, 0xFFFF00); // yellow
            }
        }

        win.update();
    }
}
