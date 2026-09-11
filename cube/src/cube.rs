use crate::ray_intersect::{Intersect, Material, RayIntersect};
use nalgebra_glm::Vec3;

const EPSILON: f32 = 1e-4;

pub struct Cube {
    pub center: Vec3,
    pub size: f32,
    pub material: Material,
}

impl Cube {
    pub fn new(center: Vec3, size: f32, material: Material) -> Self {
        Cube {
            center,
            size,
            material,
        }
    }
}

impl RayIntersect for Cube {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<Intersect> {
        let half = self.size / 2.0;
        let min = self.center - Vec3::new(half, half, half);
        let max = self.center + Vec3::new(half, half, half);

        let mut t_min = f32::NEG_INFINITY;
        let mut t_max = f32::INFINITY;
        let mut min_normal = Vec3::new(0.0, 0.0, 0.0);
        let mut max_normal = Vec3::new(0.0, 0.0, 0.0);

        for axis in 0..3 {
            let origin = ray_origin[axis];
            let direction = ray_direction[axis];

            if direction.abs() < EPSILON {
                if origin < min[axis] || origin > max[axis] {
                    return None;
                }
                continue;
            }

            let inv_d = 1.0 / direction;
            let mut unit = Vec3::new(0.0, 0.0, 0.0);
            unit[axis] = 1.0;

            let (t_near, t_far, near_normal) = if direction > 0.0 {
                (
                    (min[axis] - origin) * inv_d,
                    (max[axis] - origin) * inv_d,
                    -unit,
                )
            } else {
                (
                    (max[axis] - origin) * inv_d,
                    (min[axis] - origin) * inv_d,
                    unit,
                )
            };

            if t_near > t_min {
                t_min = t_near;
                min_normal = near_normal;
            }

            if t_far < t_max {
                t_max = t_far;
                max_normal = -near_normal;
            }

            if t_min > t_max {
                return None;
            }
        }

        let (distance, normal) = if t_min > EPSILON {
            (t_min, min_normal)
        } else if t_max > EPSILON {
            (t_max, max_normal)
        } else {
            return None;
        };

        let point = ray_origin + ray_direction * distance;

        Some(Intersect {
            point,
            normal,
            distance,
            material: self.material,
        })
    }
}