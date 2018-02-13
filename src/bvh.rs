use math::{Vector3, Zero};
use shapes::Ray;
use std::iter::*;
use std::f32;

#[derive(Clone, Copy, Debug)]
struct BBox {
    pub min: Vector3,
    pub max: Vector3,
}

impl BBox {
    pub fn ray_intersect(&self, ray: &Ray, tmin: f32, tmax: f32) -> bool {
        let mut min_max: [Vector3; 2] = [Vector3::zero(), Vector3::zero()];
        let intersect = |i, x| (x - ray.origin[i]) / ray.direction[i];
        let intersect_min = |i| intersect(i, self.min[i]);
        let intersect_max = |i| intersect(i, self.max[i]);

        for i in 0..3 {
            min_max[ray.neg[i] as usize][i] = intersect_min(i);
            min_max[1 - ray.neg[i] as usize][i] = intersect_max(i);
            let left = min_max[0][i];
            let right = min_max[1][i];
            //这里用 !(x > y) 是为了糊 NaN，下面 fold 中劳什子类似。
            assert!(!(left > right));
        }

        let left = BBox::vec3_max(min_max[0]);
        let right = BBox::vec3_min(min_max[1]);    
        left < right && left >= tmin && right < tmax
    }

    fn vec3_min(vec3: Vector3) -> f32 {
        <Vector3 as Into<[f32; 3]>>::into(vec3)
            .into_iter()
            .fold(f32::INFINITY, |x, y| if !(x > *y) { x } else { *y })
    }

    fn vec3_max(vec3: Vector3) -> f32 {
        <Vector3 as Into<[f32; 3]>>::into(vec3)
            .into_iter()
            .fold(f32::NEG_INFINITY, |x, y| if !(x < *y) { x } else { *y })
    }
}

#[cfg(test)]
mod tests {
    use super::BBox;
    use shapes::{Ray, RayBuilder};
    use math::*;
    #[test]
    fn intersect_2d() {
        let bbox = BBox {
            min: vec3(1.0, 1.0, 0.0),
            max: vec3(3.0, 3.0, 0.0),
        };

        let ray = RayBuilder {
            origin: vec3(0.0, 0.0, 0.0),
            direction: vec3(1.0, 1.0, 0.0),
        }.build();

        assert!(bbox.ray_intersect(&ray, 0.1, 10.0));
    }
}
