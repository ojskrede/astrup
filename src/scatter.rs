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
}

impl Scatter {
    pub fn new(x_coords: &Vec<f64>, y_coords: &Vec<f64>) -> Scatter {
        let (min_x, max_x) = vec_range(&x_coords);
        let (min_y, max_y) = vec_range(&y_coords);

        let mut points = Vec::<Point>::new();
        for (&x, &y) in x_coords.iter().zip(y_coords.iter()) {
            let mut scaled_x = 0.5;
            let mut scaled_y = 0.5;
            if max_x != min_x {
                scaled_x = (x - min_x) / (max_x - min_x) * 0.1 + (max_x - x) / (max_x - min_x) * 0.9;
            }
            if max_y != min_y {
                scaled_y = (y - min_y) / (max_y - min_y) * 0.1 + (max_y - y) / (max_y - min_y) * 0.9;
            }
            points.push(Point::new(scaled_x, scaled_y));
        }
        Scatter {
            points: points,
        }
    }
}

impl Drawable for Scatter {
    fn draw_fn(&self, cr: &Context) {
        for point in self.points.iter() {
            point.draw_fn(cr)
        }
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
