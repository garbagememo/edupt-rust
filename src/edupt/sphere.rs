use super::vec::*;
use super::ray::Ray;


pub const  EPS: f64 = 1e-6;

pub type Color = Vector;

pub enum ReflectionType {
    Diffuse,
    Specular,
    Refraction,
}

pub const IOR: f64 = 1.5;

pub struct Hitpoint {
    pub distance: f64,
    pub normal: Vector,
    pub position: Vector,
}

pub struct Intersection {
    pub hitpoint: Hitpoint,
    pub object_id: usize,
}

pub struct Sphere {
    radius: f64,
    position: Vector,
    pub emission: Color,
    pub color: Color,
    pub reflection_type: ReflectionType,
}

impl Sphere {
    pub fn new(radius: f64, position: Vector, emission: Color, color: Color, reflection_type: ReflectionType) -> Sphere {
        Sphere { radius, position, emission, color, reflection_type }
    }
    pub fn intersect(&self, ray: &Ray) -> Option<Hitpoint> {
        let po = self.position - ray.org;
        let b = dot(&po, &ray.dir);
        let d4 = b * b - dot(&po, &po) + self.radius * self.radius;

        if d4 < 0.0 {
            return None;
        }

        let sqrt_d4 = d4.sqrt();
        let t1 = b - sqrt_d4;
        let t2 = b + sqrt_d4;

        if t1 < EPS && t2 < EPS {
            return None;
        }

        let distance;

        if t1 > EPS {
            distance = t1;
        } else {
            distance = t2;
        }

        let position = ray.org + distance * ray.dir;
        let normal = normalize(&mut(position - self.position));
        Some(Hitpoint { distance, position, normal })
    }
}
