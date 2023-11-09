use super::element::Element;

// Enemy
#[derive(Debug, Clone, Copy)]
pub struct Enemy {
    pub position: [usize; 2],
    pub sign: &'static str,
}

impl Default for Enemy {
    fn default() -> Self {
        return Self {
            position: [0, 0],
            sign: "",
        };
    }
}

impl Enemy {
    pub fn new(position: [usize; 2], sign: &'static str) -> Self {
        Self { position, sign }
    }
}

impl Element for Enemy {
    fn get_sign(&self) -> &'static str {
        self.sign
    }
}
