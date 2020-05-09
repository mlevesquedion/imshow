extern crate image;
use ansi_term::Colour::RGB;
use image::GenericImageView;

use rayon::prelude::*;

pub fn show(image: image::DynamicImage, terminal_dimensions: Dimensions, vertical: bool) -> String {
    let resized_image = resize_image(image, terminal_dimensions, vertical);
    render(resized_image)
}

fn resize_image(
    image: image::DynamicImage,
    terminal_dimensions: Dimensions,
    vertical: bool,
) -> image::DynamicImage {
    let (image_width, image_height) = image.dimensions();
    let image_dimensions = Dimensions {
        width: image_width,
        height: image_height,
    };

    let target_dimensions = resizing_dimensions(image_dimensions, terminal_dimensions, vertical);
    image.resize_exact(
        target_dimensions.width,
        target_dimensions.height,
        image::imageops::Gaussian,
    )
}

fn render(image: image::DynamicImage) -> String {
    let (image_width, image_height) = image.dimensions();

    (0..image_height)
        .collect::<Vec<_>>()
        .par_iter()
        .chunks(2)
        .map(|row_pair| {
            let top_row = *row_pair[0];
            let bottom_row = *row_pair[1];
            (0..image_width)
                .map(|col| paint_pixel(&image, top_row as u32, bottom_row as u32, col as u32))
                .collect::<Vec<_>>()
                .join("")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn paint_pixel(image: &image::DynamicImage, top_row: u32, bottom_row: u32, col: u32) -> String {
    let top_pixel = image.get_pixel(col, top_row).0;
    let bottom_pixel = image.get_pixel(col, bottom_row).0;
    RGB(bottom_pixel[0], bottom_pixel[1], bottom_pixel[2])
        .on(RGB(top_pixel[0], top_pixel[1], top_pixel[2]))
        .paint("▄")
        .to_string()
}

#[derive(Debug, PartialEq)]
pub struct Dimensions {
    pub width: u32,
    pub height: u32,
}

fn resizing_dimensions(
    image_dimensions: Dimensions,
    terminal_dimensions: Dimensions,
    vertical: bool,
) -> Dimensions {
    let mut width: u32;
    let mut height: u32;
    if vertical {
        height = (terminal_dimensions.height - 1) * 2;
        width = cross(image_dimensions.width, image_dimensions.height, height);
        if width > terminal_dimensions.width {
            println!("{}", 1);
            width = terminal_dimensions.width;
            height = previous_even(cross(
                image_dimensions.height,
                image_dimensions.width,
                width,
            ));
        }
    } else {
        width = terminal_dimensions.width;
        height = previous_even(cross(
            image_dimensions.height,
            image_dimensions.width,
            width,
        ));
    }

    Dimensions { width, height }
}

fn previous_even(n: u32) -> u32 {
    if n % 2 == 0 {
        n
    } else {
        n - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_when_input_is_even_then_returns_input() {
        assert_eq!(previous_even(12), 12);
    }

    #[test]
    fn test_when_input_is_odd_then_returns_previous_even() {
        assert_eq!(previous_even(11), 10);
    }
}

fn cross(a: u32, b: u32, d: u32) -> u32 {
    ((a as f32 / b as f32) * d as f32).ceil() as u32
}

#[cfg(test)]
mod resizing_dimensions_tests {
    use super::*;

    #[test]
    fn test_vertical_resizing() {
        let image_dimensions = Dimensions {
            width: 250,
            height: 300,
        };
        let terminal_dimensions = Dimensions {
            width: 200,
            height: 100,
        };
        assert_eq!(
            resizing_dimensions(image_dimensions, terminal_dimensions, true),
            Dimensions {
                width: 165,
                height: 198
            }
        );
    }

    #[test]
    fn test_vertical_resizing_too_wide() {
        let image_dimensions = Dimensions {
            width: 500,
            height: 300,
        };
        let terminal_dimensions = Dimensions {
            width: 200,
            height: 100,
        };
        assert_eq!(
            resizing_dimensions(image_dimensions, terminal_dimensions, true),
            Dimensions {
                width: 200,
                height: 120
            }
        );
    }

    #[test]
    fn test_non_vertical() {
        let image_dimensions = Dimensions {
            width: 250,
            height: 300,
        };
        let terminal_dimensions = Dimensions {
            width: 200,
            height: 100,
        };
        assert_eq!(
            resizing_dimensions(image_dimensions, terminal_dimensions, false),
            Dimensions {
                width: 200,
                height: 240
            }
        );
    }
}
