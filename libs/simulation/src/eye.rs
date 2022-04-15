use std::f32::consts::*;
use nalgebra as na;

use super::world;

// Animal's vision range i.e. how far can they see
const FOV_RANGE: f32 = 0.25;
// Animal's field of view
// The reader(presumably a human being)
// has 120 degrees or (2/3)*PI FOV
const FOV_ANGLE: f32 = PI + FRAC_PI_4;
// Number of photoreceptors
// More photoreceptors will help the animal to assess
// the "general direction" of the food more precisely
// However, too many will cause the simulation to progress
// at a snail's pace
const PHOTORECEPTORS: usize = 9;

#[derive(Debug)]
pub struct Eye {
    fov_range: f32,
    fov_angle: f32,
    photoreceptors: usize
}

impl Eye {
    fn new(fov_range: f32, fov_angle: f32, photoreceptors: usize) -> Self {
        assert!(fov_range > 0.0);
        assert!(fov_angle > 0.0);
        assert!(photoreceptors > 0);

        Self {
            fov_range,
            fov_angle,
            photoreceptors
        }
    }

    pub fn photoreceptors(&self) -> usize {
        self.photoreceptors
    }

    pub fn process_vision(
        &self,
        position: na::Point2<f32>,
        rotation: na::Rotation2<f32>,
        foods: &[world::Food]
    ) -> Vec<f32> {
        let mut photoreceptors = vec![0.0; self.photoreceptors];

        for food in foods {
            // relative vector from animal's position to food's position
            let vector = food.position - position;
            // distance b/w animal and food
            let dist = vector.norm();
            // angle b/w vector and x-axis
            let angle = na::Rotation2::rotation_between(
                &na::Vector2::x(),
                &vector
            ).angle();
            // angle b/w vector and animal
            let angle = na::wrap(
                angle - rotation.angle(),
                -PI,
                PI
            );

            // check whether out of fov_range
            if dist >= self.fov_range {
                continue;
            }

            // check whether out of fov_angle
            if angle < -self.fov_angle || angle > self.fov_angle
            {
                continue;
            }

            // angle_wrt_animal is in [-fov_range / 2, fov_range / 2]
            // transform it to range [0, fov_range]
            let angle = angle + (self.fov_angle / 2.0);
            // transform to range [0, 1]
            // Now, multiplying with no. of photorecpetors ~= index of photoreceptor "sees" the food
            let photoreceptor = (angle / self.fov_angle) * (self.photoreceptors as f32);
            let photoreceptor = (photoreceptor as usize)
                .min(photoreceptors.len() - 1);

            // heat -> how close the food is
            let heat = (self.fov_range - dist) / self.fov_range;
            photoreceptors[photoreceptor] += heat
        }

        todo!()
    }
}

impl Default for Eye {
    fn default() -> Self {
        Self::new(
            FOV_RANGE,
            FOV_ANGLE,
            PHOTORECEPTORS
        )
    }
}
