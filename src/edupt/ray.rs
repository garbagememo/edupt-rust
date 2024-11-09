use super::vec::Vector;

pub struct Ray {
    pub org: Vector,
    pub dir: Vector,
}

impl Ray {
    pub fn new(org: Vector, dir: Vector) -> Ray {
        Ray { org, dir }
    }
}