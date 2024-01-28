mod shapes {
    pub struct Circle {
        radius: f32,
    }

    impl Circle {
        pub fn new(radius: f32) -> Circle {
            Circle { radius }
        }

        pub fn new_1(radius: f32) -> Result<Circle, String> {
            match radius {
                ..=0.0 => Err("radius must be positive".to_string()),
                _ => Ok(Circle { radius }),
            }
        }

        pub fn new_2(radius: f32) -> Circle {
            match radius {
                ..=0.0 => panic!("radius must be positive"),
                _ => Circle { radius },
            }
        }

        pub fn contains(&self, other: &Circle) -> bool {
            self.radius > other.radius
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_circle_should_contain_smaller() {
        let larger = shapes::Circle::new(5.0);
        let smaller = shapes::Circle::new(3.0);

        assert_eq!(
            larger.contains(&smaller),
            true,
            "larger should contain smaller"
        );
    }

    #[test]
    fn smaller_circle_should_not_contain_larger() {
        let larger = shapes::Circle::new(5.0);
        let smaller = shapes::Circle::new(3.0);

        assert_eq!(
            smaller.contains(&larger),
            false,
            "smaller should not contain larger"
        );
    }

    #[test]
    fn should_not_create_circle() -> Result<(), String> {
        let circle = shapes::Circle::new_1(-1.0)?;
        Ok(())
    }

    #[test]
    #[should_panic(expected = "radius must be positive")]
    fn should_not_create_and_panic() {
        let circle = shapes::Circle::new_2(-1.0);
    }
}
