//! Colors. This might be used in the future, but not currently.
//!

pub fn red(alpha: f64) -> [f64; 4] {
    [1.0, 0.0, 0.0, alpha]
}

pub fn green(alpha: f64) -> [f64; 4] {
    [0.0, 1.0, 0.0, alpha]
}

pub fn blue(alpha: f64) -> [f64; 4] {
    [0.0, 0.0, 1.0, alpha]
}

pub fn black(alpha: f64) -> [f64; 4] {
    [0.0, 0.0, 0.0, alpha]
}

pub fn gray(alpha: f64) -> [f64; 4] {
    [0.5, 0.5, 0.5, alpha]
}

pub fn white(alpha: f64) -> [f64; 4] {
    [1.0, 1.0, 1.0, alpha]
}
