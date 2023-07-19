use std::f64::consts::PI;

enum Shape {
    Square(f64),
    Rect { a: f64, b: f64 },
    RightTriangle(f64, f64),
    Circle { radius: f64 },
}

#[derive(Debug)]
enum Compare {
    Same,
    Different,
}

#[derive(Debug)]
struct ShapeCompare {
    shape: Compare,
    area: Compare,
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Square(a) => a.powi(2),
            Shape::Rect { a, b } => a * b,
            Shape::RightTriangle(a, b) => a * b / 2.,
            Shape::Circle { radius: r } => PI * r.powi(2),
        }
    }

    fn same_as(&self, other: Shape) -> ShapeCompare {
        let area_same = if self.area() == other.area() {
            Compare::Same
        } else {
            Compare::Different
        };

        let shape_same = match (self, other) {
            (Shape::Square(_), Shape::Square(_))
            | (Shape::Rect { .. }, Shape::Rect { .. })
            | (Shape::RightTriangle(_, _), Shape::RightTriangle(_, _))
            | (Shape::Circle { .. }, Shape::Circle { .. }) => Compare::Same,
            _ => Compare::Different,
        };

        ShapeCompare {
            shape: shape_same,
            area: area_same,
        }
    }
}

fn main() {
    let first = Shape::Square(2.0);
    let second = Shape::Rect { a: 1.0, b: 4.0 };

    println!("Shapes are: {:?}", first.same_as(second));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square() {
        let square = Shape::Square(2.);
        let actual = square.area();
        let expected = 4.0;

        assert_eq!(actual, expected)
    }

    #[test]
    fn rect() {
        let actual = Shape::Rect { a: 2., b: 4. }.area();
        let expected = 8.0;

        assert_eq!(actual, expected)
    }

    #[test]
    fn triangle() {
        let actual = Shape::RightTriangle(2., 4.).area();
        let expected = 4.0;

        assert_eq!(actual, expected)
    }

    #[test]
    fn circle() {
        let actual = Shape::Circle { radius: 1.0 }.area();
        let expected = PI;

        assert_eq!(actual, expected)
    }
}
