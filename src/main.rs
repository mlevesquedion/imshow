extern crate image;
extern crate term_size;

use clap::{App, Arg};

use imshow::show;
use imshow::Dimensions;

fn main() {
    let matches = App::new("imshow")
        .version("0.1")
        .about("Show images in the terminal.")
        .arg(
            Arg::with_name("path")
                .help("Path of the image file to show.")
                .short("f")
                .long("file")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("vertical")
                .help("Fill the terminal's height. (otherwise, fills the terminal's width)")
                .short("v")
                .long("vertical")
                .required(false),
        )
        .get_matches();

    let path = matches.value_of("path").unwrap();
    let vertical = matches.is_present("vertical");

    let (width, height) = term_size::dimensions().unwrap();
    let term_dimensions = Dimensions { width, height };

    print!(
        "{}",
        show(image::open(path).unwrap(), term_dimensions, vertical)
    );
}
