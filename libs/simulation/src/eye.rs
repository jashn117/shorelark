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
            if angle < -self.fov_angle / 2.0 || angle > self.fov_angle / 2.0
            {
                continue;
            }

            // angle is in [-fov_range / 2, fov_range / 2]
            // transform it to range [0, fov_range]
            let angle = angle + (self.fov_angle / 2.0);
            // transform to range [0, 1]
            let photoreceptor = angle / self.fov_angle;
            // Now, multiplying with no. of photorecpetors ~= idx of photoreceptor that "sees" the food
            let photoreceptor = photoreceptor * (self.photoreceptors as f32);
            // eliminate edge case where idx exceeds bounds for photoreceptors
            let photoreceptor = (photoreceptor as usize)
                .min(photoreceptors.len() - 1);

            // heat -> how close the food is
            let heat = (self.fov_range - dist) / self.fov_range;
            photoreceptors[photoreceptor] += heat
        }

        photoreceptors
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

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_PHOTORECEPTORS: usize = 13;

    struct TestCase {
        foods: Vec<world::Food>,
        fov_range: f32,
        fov_angle: f32,
        x: f32,
        y: f32,
        rotation: f32,
        expected_vision: &'static str
    }

    impl TestCase {
        fn run(self) {
            let eye = Eye::new(
                self.fov_range,
                self.fov_angle,
                TEST_PHOTORECEPTORS
            );

            let actual_vision = eye
                .process_vision(
                    na::Point2::new(self.x, self.y),
                    na::Rotation2::new(self.rotation),
                    &self.foods
                );

            // make actual_vision result human readable
            let actual_vision: Vec<_> = actual_vision
                .into_iter()
                .map(|heat| {
                    if heat >= 0.7 {
                        //* HOT: when food is really close
                        "$"
                    } else if heat >= 0.3 {
                        //* WARM: when food is moderately close
                        "~"
                    } else if heat > 0.0 {
                        //* COLD: when food si at the edge of fov_range
                        "."
                    } else {
                        //* VOID: when food is out of fov_range
                        " "
                    }
                })
                .collect();

            let actual_vision = actual_vision
                .join("");

            assert_eq!(actual_vision, self.expected_vision);
        }

        fn food_helper(x: f32, y: f32) -> world::Food {
            world::Food {
                position: na::Point2::new(x, y)
            }
        }
    }

    mod different_fov_ranges {
        use super::*;
        use test_case::test_case;

        #[test_case(1.0, "      $      ")]
        #[test_case(0.9, "      $      ")]
        #[test_case(0.8, "      $      ")]
        #[test_case(0.7, "      $      ")]
        #[test_case(0.6, "      ~      ")]
        #[test_case(0.5, "      ~      ")]
        #[test_case(0.4, "      ~      ")]
        #[test_case(0.3, "      ~      ")]
        #[test_case(0.2, "      .      ")]
        #[test_case(0.1, "             ")]
        fn test(fov_range: f32, expected_vision: &'static str) {
            TestCase {
                // food is vertically centered and is located
                // close to the right most edge
                foods: vec![TestCase::food_helper(0.7, 0.5)],
                // fov_range varies b/w [1.0, 0.1]
                fov_range,
                // PI / 2 or 90 degrees
                fov_angle: FRAC_PI_2,
                // animal is fixed at center of the world
                x: 0.5,
                y: 0.5,
                // animal is looking to the right
                rotation: 0.0,
                expected_vision
            }.run();
        }
    }

    mod different_rotations {
        use super::*;
        use test_case::test_case;

        #[test_case(0.00 * PI, "      ~      ")]
        #[test_case(0.25 * PI, "    ~        ")]
        #[test_case(0.50 * PI, "   ~         ")]
        #[test_case(0.75 * PI, " ~           ")]
        #[test_case(1.00 * PI, "            ~")]
        #[test_case(1.25 * PI, "           ~ ")]
        #[test_case(1.50 * PI, "         ~   ")]
        #[test_case(1.75 * PI, "        ~    ")]
        #[test_case(2.00 * PI, "      ~      ")]
        #[test_case(2.25 * PI, "    ~        ")]
        #[test_case(2.50 * PI, "   ~         ")]
        fn test(rotation: f32, expected_vision: &'static str) {
            TestCase {
                // food is vertically centered and is located
                // at the right most edge
                foods: vec![TestCase::food_helper(1.0, 0.5)],
                // vision only limited by fov_angle
                fov_range: 1.0,
                // 2 * PI or 360 degrees
                fov_angle: 2.0 * PI,
                // animal is fixed at center of the world
                x: 0.5,
                y: 0.5,
                // varies b/w [0.0, 2.5 * PI] or [0 degrees, 450 degrees]
                rotation,
                expected_vision
            }.run();
        }
    }

    mod different_positions {
        use super::*;
        use test_case::test_case;

        #[test_case(0.9, 0.5, "$           $")]
        #[test_case(0.8, 0.5, "  $       $  ")]
        #[test_case(0.7, 0.5, "   ~     ~   ")]
        #[test_case(0.6, 0.5, "    ~   ~    ")]
        #[test_case(0.5, 0.5, "    ~   ~    ")]
        #[test_case(0.4, 0.5, "     ~ ~     ")]
        #[test_case(0.3, 0.5, "     . .     ")]
        #[test_case(0.2, 0.5, "     . .     ")]
        #[test_case(0.1, 0.5, "     . .     ")]
        #[test_case(0.0, 0.5, "             ")]
        // ------------------------------------
        #[test_case(0.5, 0.9, ". ~          ")]
        #[test_case(0.5, 0.8, "~  ~         ")]
        #[test_case(0.5, 0.7, "  ~ ~        ")]
        #[test_case(0.5, 0.6, "   ~  ~      ")]
        #[test_case(0.5, 0.51, "    ~  ~     ")]
        #[test_case(0.5, 0.4, "      ~  ~   ")]
        #[test_case(0.5, 0.3, "        ~ ~  ")]
        #[test_case(0.5, 0.2, "         ~  ~")]
        #[test_case(0.5, 0.1, "          ~ .")]
        #[test_case(0.5, 0.0, "            ~")]
        fn test(x: f32, y: f32, expected_vision: &'static str) {
            TestCase {
                // food
                foods: vec![
                    TestCase::food_helper(1.0, 0.4),
                    TestCase::food_helper(1.0, 0.6)
                ],
                // vision only limited by fov_angle
                fov_range: 1.0,
                // PI / 2 or 90 degrees
                fov_angle: FRAC_PI_2,
                // varies b/2 [0.9, 0.1] while y is fixed at 0.5
                x,
                // varies b/2 [0.9, 0.1] while x is fixed at 0.5
                y,
                // animal is looking to the right
                rotation: 0.0,
                expected_vision
            }.run();
        }
    }

    mod different_fov_angles {
        use super::*;
        use test_case::test_case;

        #[test_case(0.25 * PI, " ~         ~ ")] // FOV is narrow = 2 foods
        #[test_case(0.50 * PI, ".  ~     ~  .")]
        #[test_case(0.75 * PI, "  . ~   ~ .  ")] // FOV gets progressively
        #[test_case(1.00 * PI, "   . ~ ~ .   ")] // wider and wider...
        #[test_case(1.25 * PI, "   . ~ ~ .   ")]
        #[test_case(1.50 * PI, ".   .~ ~.   .")]
        #[test_case(1.75 * PI, ".   .~ ~.   .")]
        #[test_case(2.00 * PI, "~.  .~ ~.  .~")] // FOV is wide = 8 foods
        fn test(fov_angle: f32, expected_vision: &'static str) {
            TestCase {
                // food
                foods: vec![
                    TestCase::food_helper(0.0, 0.0),
                    TestCase::food_helper(0.0, 0.33),
                    TestCase::food_helper(0.0, 0.66),
                    TestCase::food_helper(0.0, 1.0),
                    TestCase::food_helper(1.0, 0.0),
                    TestCase::food_helper(1.0, 0.33),
                    TestCase::food_helper(1.0, 0.66),
                    TestCase::food_helper(1.0, 1.0)
                ],
                // vision only limited by fov_angle
                fov_range: 1.0,
                // varies b/w [0.0, 2 * PI] or [0 degrees, 360 degrees]
                fov_angle,
                // fixed at the world center
                x: 0.5,
                y: 0.5,
                // animal is looking to the right
                rotation: 0.0,
                expected_vision
            }.run();
        }
    }
}
