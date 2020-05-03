extern crate image;

use std::fmt;

use ansi_term::Colour::RGB;

use image::GenericImageView;

pub struct Dimensions {
    height: u32,
    width: u32,
}

impl Dimensions {
    pub fn from_width_height(pair: (u32, u32)) -> Dimensions {
        let (width, height) = pair;
        Dimensions { height, width }
    }
}

impl fmt::Display for Dimensions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.height, self.width)
    }
}

pub fn show(im: image::DynamicImage, display_dimensions: &Dimensions) {
    // TODO:
    r#"
    ASSUMPTIONS TO RELAX:
    * Image is wider than terminal

    Find width of terminal window and pass it as an argument.
    Find ratio of window to image.
    Find number of pixels to chunk per row and per column.
    For each row, for each column chunk, average the pixel values.
    println! each row.
    "#;
    let im_dimensions = Dimensions::from_width_height(im.dimensions());

    let pixels_per_column =
        (im_dimensions.width as f32 / display_dimensions.width as f32).ceil() as u32;

    println!("Image dimensions: {}", im_dimensions);
    println!("Pixels per column: {}", pixels_per_column);

    for block_row in (0..im_dimensions.height).step_by(pixels_per_column as usize) {
        for block_col in (0..im_dimensions.width).step_by(pixels_per_column as usize) {
            let mut cum_red = 0;
            let mut cum_green = 0;
            let mut cum_blue = 0;
            let mut count = 0;
            let max_row = (im_dimensions.height).min(block_row + pixels_per_column) as u32;
            let max_col = (im_dimensions.width).min(block_col + pixels_per_column) as u32;

            if block_row == max_row || block_col == max_col {
                continue;
            }

            for row in block_row..max_row {
                for col in block_col..max_col {
                    count += 1;
                    cum_red += im.get_pixel(col, row).0[0] as u32;
                    cum_green += im.get_pixel(col, row).0[1] as u32;
                    cum_blue += im.get_pixel(col, row).0[2] as u32;
                }
            }
            let average_red = (cum_red / count) as u8;
            let average_green = (cum_green / count) as u8;
            let average_blue = (cum_blue / count) as u8;
            print!(
                "{}",
                RGB(average_red, average_green, average_blue).paint("█")
            );
        }
        println!("");
    }

    println!("dimensions {:?}", im.dimensions());
    println!("{:?}", im.color());
    println!("{:?}", im.get_pixel(0, 0).0[0]);
}
