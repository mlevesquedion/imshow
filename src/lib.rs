extern crate image;
use ansi_term::Colour::RGB;
use image::GenericImageView;

mod utils;
pub use utils::*;

pub fn show(im: image::DynamicImage, display_dimension: usize) -> String {
    let (im_width, im_height) = im.dimensions();

    let pixels_per_col = (im_width as f32 / display_dimension as f32).ceil() as usize;
    let pixels_per_row = pixels_per_col * 2; // terminal cells are about twice as high as they are wide

    let mut rendering = String::new();

    for row_indices in chunks_up_to(im_height, pixels_per_row).iter() {
        for col_indices in chunks_up_to(im_width, pixels_per_col).iter() {
            let mean_pixel = subimage_mean(&im, &row_indices, &col_indices);
            rendering += &RGB(mean_pixel.0, mean_pixel.1, mean_pixel.2)
                .paint("█")
                .to_string();
        }
        rendering.push('\n');
    }

    rendering
}

fn subimage_mean(
    im: &image::DynamicImage,
    row_indices: &Vec<u32>,
    col_indices: &Vec<u32>,
) -> (u8, u8, u8) {
    let mut acc = PixelAccumulator::new();
    for row in row_indices {
        for col in col_indices {
            acc.add(im.get_pixel(*col, *row).0);
        }
    }
    acc.mean()
}

struct PixelAccumulator {
    red: u32,
    green: u32,
    blue: u32,
    count: u32,
}

impl PixelAccumulator {
    fn new() -> PixelAccumulator {
        PixelAccumulator {
            red: 0,
            green: 0,
            blue: 0,
            count: 0,
        }
    }

    fn add(&mut self, pixel: [u8; 4]) {
        self.red += pixel[0] as u32;
        self.green += pixel[1] as u32;
        self.blue += pixel[2] as u32;
        self.count += 1;
    }

    fn mean(&self) -> (u8, u8, u8) {
        assert!(
            self.count > 0,
            "tried to take the mean of an empty accumulator"
        );
        (
            (self.red as f32 / self.count as f32).round() as u8,
            (self.green as f32 / self.count as f32).round() as u8,
            (self.blue as f32 / self.count as f32).round() as u8,
        )
    }
}

#[cfg(test)]
mod accumulator_tests {
    use super::*;

    #[test]
    fn test_accumulator_starts_out_empty() {
        let accumulator = PixelAccumulator::new();
        assert_eq!(0, accumulator.red);
        assert_eq!(0, accumulator.green);
        assert_eq!(0, accumulator.blue);
        assert_eq!(0, accumulator.count);
    }

    #[test]
    fn test_addition() {
        let mut accumulator = PixelAccumulator::new();
        accumulator.add([1, 2, 3, 0]);
        assert_eq!(accumulator.mean(), (1, 2, 3));
    }

    #[test]
    fn test_mean() {
        let mut accumulator = PixelAccumulator::new();
        accumulator.add([1, 1, 0, 0]);
        accumulator.add([1, 2, 0, 0]);
        assert_eq!(accumulator.mean(), (1, 2, 0));
    }

    #[test]
    #[should_panic]
    fn test_mean_panics_when_accumulator_is_empty() {
        let accumulator = PixelAccumulator::new();
        accumulator.mean();
    }
}
