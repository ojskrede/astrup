//! # Astrup
//!
//! A rust plotting library.
//!
//! Plans:
//! - Plotting Vec<Num> and ndarray Array* types
//!
//!
//! Structurs
//!
//! Figure: The main image. Can contain multiple plots.
//! Plot: An area defined by an x and y axis. Can contain multiple draw methods.
//! Draw: line, scatter, hist, image. These methods draw whatever they specify onto its plot. It
//! should be possible to combine as many as you want of any combination.
//!

extern crate cairo;
extern crate gio;
extern crate gtk;
extern crate num;

pub mod utils;
pub mod figure;
pub mod plot;
pub mod scatter;
pub mod point;
pub mod color;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
