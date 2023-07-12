
 */

 fn main() {
    let rect = Rect::new((1., 2.), 5.);
    println!("Bottom right coordinates: {:?}", rect.bottom_right());
    println!("Area: {}", rect.area());
    println!("Perimeter: {}", rect.perimeter());
}

struct Rect {
    top_left: (f32, f32),
    width: f32,
}

impl Rect {
    fn new(top_left: (f32, f32), width: f32) -> Self {
        Rect {
            top_left,
            width,
        }
    }

    fn bottom_right(&self) -> (f32, f32) {
        let (x, y) = self.top_left;
        let bottom_right_x = x + self.width;
        let bottom_right_y = y + self.width;
        (bottom_right_x, bottom_right_y)
    }

    fn area(&self) -> f32 {
        self.width * self.width
    }

    fn perimeter(&self) -> f32 {
        self.width * 4.0
    }
}


// ----> TESTS
#[cfg(test)]
mod tests {
    use crate::Rect;

    #[test]
    fn bottom_right() {
        let rect = Rect::new((1., 2.), 5.);

        assert_eq!((6., 7.), rect.bottom_right())
    }

    #[test]
    fn area() {
        let rect = Rect::new((1., 2.), 5.);

        assert_eq!(25., rect.area())
    }

    #[test]
    fn perimeter() {
        let rect = Rect::new((1., 2.), 5.);

        assert_eq!(20., rect.perimeter())
    }
}
