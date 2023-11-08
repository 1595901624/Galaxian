/// Plane
/// player control the plane
#[derive(Debug, Clone)]
pub(crate) struct Plane {
    pub(crate) position: [usize; 2],
}

impl Plane {
    pub(crate) fn new(position: [usize; 2]) -> Self {
        Self { position }
    }
}
