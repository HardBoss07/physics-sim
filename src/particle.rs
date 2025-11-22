
pub struct Particle {
    pub x: usize,
    pub y: usize,
    pub color: u32,
}

impl Particle {
    pub fn new(x: usize, y: usize, color: u32) -> Self {
        Particle { x, y, color }
    }

    pub fn is_down_free() -> bool {
        // Placeholder for checking if the space below the particle is free
        true
    }

    pub fn is_left_free() -> bool {
        // Placeholder for checking if the space to the left of the particle is free
        true
    }

    pub fn is_right_free() -> bool {
        // Placeholder for checking if the space to the right of the particle is free
        true
    }
}