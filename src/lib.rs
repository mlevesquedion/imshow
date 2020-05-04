extern crate image;
use ansi_term::Colour::RGB;
use image::GenericImageView;

#[macro_use]
mod math;
pub use math::*;

pub fn show(im: image::DynamicImage, terminal_dimensions: Dimensions, vertical: bool) -> String {
    let display_dimension = display_dimension(terminal_dimensions, vertical);
    render(mean_pixels(im, display_dimension))
}

pub struct Dimensions {
    pub width: usize,
    pub height: usize,
}

fn display_dimension(terminal: Dimensions, vertical: bool) -> usize {
    if vertical {
        // terminal cells are about twice as high as they are wide
        (terminal.height * 2).min(terminal.width)
    } else {
        terminal.width
    }
}

fn mean_pixels(im: image::DynamicImage, display_dimension: usize) -> Vec<Vec<(u8, u8, u8)>> {
    let (im_width, im_height) = im.dimensions();
    let (im_width, im_height) = (im_width as f64, im_height as f64);

    let mut pixels: Vec<Vec<(u8, u8, u8)>> = Vec::new();
    // terminal cells are about twice as high as they are wide
    for row_step in linspace!(0.0, im_height, display_dimension / 2) {
        let mut row_pixels: Vec<(u8, u8, u8)> = Vec::new();
        for col_step in linspace!(0.0, im_width, display_dimension) {
            let mean_pixel = subimage_mean(&im, &row_step, &col_step);
            row_pixels.push(mean_pixel);
        }
        pixels.push(row_pixels);
    }

    pixels
}

fn render(pixels: Vec<Vec<(u8, u8, u8)>>) -> String {
    let mut rendering = String::new();

    for pixel_row in pixels {
        for pixel in pixel_row {
            rendering += &RGB(pixel.0, pixel.1, pixel.2).paint("█").to_string();
        }
        rendering.push('\n');
    }

    rendering
}

fn subimage_mean(
    im: &image::DynamicImage,
    row_step: &(f64, f64),
    col_step: &(f64, f64),
) -> (u8, u8, u8) {
    let mut acc = FloatPixel::new();
    for (row, row_weight) in weighted_indices(row_step.0, row_step.1) {
        let mut row_acc = FloatPixel::new();
        for (col, col_weight) in weighted_indices(col_step.0, col_step.1) {
            row_acc.add_pixel(im.get_pixel(col as u32, row as u32).0, col_weight);
        }
        acc.add_other(row_acc, row_weight);
    }
    acc.to_rgb()
}

struct FloatPixel(f64, f64, f64);

impl FloatPixel {
    fn new() -> Self {
        Self(0.0, 0.0, 0.0)
    }

    fn add_other(&mut self, pixel: Self, weight: f64) {
        self.0 += pixel.0 * weight;
        self.1 += pixel.1 * weight;
        self.2 += pixel.2 * weight;
    }

    fn add_pixel(&mut self, pixel: [u8; 4], weight: f64) {
        self.0 += pixel[0] as f64 * weight;
        self.1 += pixel[1] as f64 * weight;
        self.2 += pixel[2] as f64 * weight;
    }

    fn to_rgb(&self) -> (u8, u8, u8) {
        (
            self.0.round() as u8,
            self.1.round() as u8,
            self.2.round() as u8,
        )
    }
}

#[cfg(test)]
mod display_dimension_tests {
    use super::*;

    #[test]
    fn test_uses_terminal_width_when_not_vertical() {
        let terminal_dimensions = Dimensions {
            width: 10,
            height: 20,
        };
        assert_eq!(10, display_dimension(terminal_dimensions, false));
    }

    #[test]
    fn test_uses_twice_terminal_height_when_vertical() {
        let terminal_dimensions = Dimensions {
            width: 50,
            height: 20,
        };
        assert_eq!(40, display_dimension(terminal_dimensions, true));
    }

    #[test]
    fn test_uses_terminal_width_when_vertical_and_twice_terminal_height_is_greater_than_width() {
        let terminal_dimensions = Dimensions {
            width: 30,
            height: 20,
        };
        assert_eq!(30, display_dimension(terminal_dimensions, true));
    }
}

#[cfg(test)]
mod float_pixel_tests {
    use super::*;

    impl FloatPixel {
        fn from(r: f64, g: f64, b: f64) -> Self {
            Self(r, g, b)
        }
    }

    #[test]
    fn test_new_is_zeros() {
        let pixel = FloatPixel::new();
        assert_eq!(0.0, pixel.0);
        assert_eq!(0.0, pixel.1);
        assert_eq!(0.0, pixel.2);
    }

    #[test]
    fn test_add_pixel() {
        let mut fp = FloatPixel::new();
        fp.add_pixel([255, 10, 62, 255], 0.5);
        assert_eq!((128, 5, 31), fp.to_rgb());
    }

    #[test]
    fn test_add_other() {
        let mut fp = FloatPixel::new();
        let other = FloatPixel::from(120.0, 233.0, 180.0);
        fp.add_other(other, 0.5);
        assert_eq!((60, 117, 90), fp.to_rgb());
    }
}
