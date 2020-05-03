extern crate image;
extern crate term_size;

use imshow::{show, Dimensions};

use clap::{App, Arg};

macro_rules! u32_pair {
    ($t:expr) => {{
        let (a, b) = $t;
        (a as u32, b as u32)
    }};
}

fn main() {
    let matches = App::new("imshow")
        .version("0.1")
        .about("Show images in the terminal.")
        .arg(
            Arg::with_name("path")
                .short("f")
                .long("file")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let paths = &matches.value_of("path");

    let term_dimensions =
        Dimensions::from_width_height(u32_pair!(term_size::dimensions().unwrap()));

    println!("Terminal dimensions: {}", term_dimensions);

    for p in paths.iter() {
        show(image::open(p).unwrap(), &term_dimensions);
    }
}
