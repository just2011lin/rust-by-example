#![allow(dead_code)]

pub struct Point {
    x: f32,
    y: f32,
}

pub struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn rect_area(&self) -> f32 {
        let Rectangle {
            top_left,
            bottom_right,
        } = self;
        let width = bottom_right.x - top_left.x;
        let height = bottom_right.y - top_left.y;
        width * height
    }
    pub fn square(top_left: Point, f: f32) -> Self {
        let bottom_right = Point {
            x: top_left.x + f,
            y: top_left.y + f,
        };
        Self {
            top_left,
            bottom_right,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rect_area() {
        let rect = Rectangle {
            top_left: Point { x: 1.1, y: 1.1 },
            bottom_right: Point { x: 6.6, y: 6.6 },
        };
        let area = rect.rect_area();
        assert_eq!(area, 5.5 * 5.5);
    }

    #[test]
    fn test_square() {
        let rect = Rectangle::square(Point { x: 1.1, y: 1.1 }, 5.5);
        assert_eq!(rect.rect_area(), 5.5 * 5.5);
    }
}
