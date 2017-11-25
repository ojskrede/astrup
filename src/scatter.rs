//! ## Scatter
//!
//! Module that defines the Scatter struct
//!

use cairo::Context;

use point::Point;
use draw::Drawable;

#[derive(Clone, Debug)]
pub struct Scatter {
    points: Vec<Point>,
    min_x: f64,
    max_x: f64,
    min_y: f64,
    max_y: f64,
}

impl Scatter {
    pub fn new(x_coords: &Vec<f64>, y_coords: &Vec<f64>) -> Scatter {
        let (min_x, max_x) = vec_range(&x_coords);
        let (min_y, max_y) = vec_range(&y_coords);
        let (new_min_x, new_max_x, new_min_y, new_max_y) = (0.11, 0.89, 0.11, 0.89);

        let mut points = Vec::<Point>::new();
        for (&x, &y) in x_coords.iter().zip(y_coords.iter()) {
            let mut scaled_x = 0.5;
            if max_x != min_x {
                scaled_x = (x - min_x) / (max_x - min_x) * new_max_x + (max_x - x) / (max_x - min_x) * new_min_x;
            }
            let mut scaled_y = 0.5;
            if max_y != min_y {
                scaled_y = (y - min_y) / (max_y - min_y) * new_max_y + (max_y - y) / (max_y - min_y) * new_min_y;
                scaled_y = 1.0 - scaled_y;
            }
            points.push(Point::new(scaled_x, scaled_y));
        }
        Scatter {
            points: points,
            min_x: min_x,
            max_x: max_x,
            min_y: min_y,
            max_y: max_y,
        }
    }
}

impl Drawable for Scatter {
    fn draw_fn(&self, cr: &Context) {
        for point in self.points.iter() {
            point.draw_fn(cr)
        }
    }

    fn min_x(&self) -> f64 {
        self.min_x
    }

    fn max_x(&self) -> f64 {
        self.max_x
    }

    fn min_y(&self) -> f64 {
        self.min_y
    }

    fn max_y(&self) -> f64 {
        self.max_y
    }
}

fn vec_range(vec: &Vec<f64>) -> (f64, f64) {
    let mut max_val = vec[0];
    let mut min_val = vec[0];
    for val in vec.iter() {
        max_val = val.max(max_val);
        min_val = val.min(min_val);
    }
    (min_val, max_val)
}
