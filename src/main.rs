extern crate image;
extern crate term_size;

use imshow::{show, Dimensions};

use clap::{App, Arg};

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

    let path = &matches.value_of("path").unwrap();

    let (term_width, term_height) = term_size::dimensions().unwrap();
    let term_dimensions = Dimensions::from_width_height((term_width as u32, term_height as u32));

    print!("{}", show(image::open(path).unwrap(), &term_dimensions));
}
