use super::element::Element;

/// Plane
/// player control the plane
#[derive(Debug, Clone, Copy)]
pub struct Plane {
    pub position: [usize; 2],
    pub sign: &'static str,
}

impl Plane {
    pub fn new(position: [usize; 2], sign: &'static str) -> Self {
        Self { position, sign }
    }
}

impl Element for Plane {
    fn get_sign(&self) -> &'static str {
        self.sign
    }
}
