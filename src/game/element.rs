use std::usize;

// Element Trait
pub trait Element {
    fn get_sign(&self) -> &'static str;
}

#[derive(Clone)]
pub struct EmptyElement {
    pub position: [usize; 2],
}

impl Element for EmptyElement {
    fn get_sign(&self) -> &'static str {
        " "
    }
}

impl Default for EmptyElement {
    fn default() -> Self {
        Self { position: [0, 0] }
    }
}

impl EmptyElement {
    pub fn new(position: [usize; 2]) -> Self {
        Self { position }
    }

    pub fn with(x: usize, y: usize) -> Self {
        Self::new([x, y])
    }
}
