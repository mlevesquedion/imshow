extern crate image;
extern crate term_size;

use imshow::show;

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
        .arg(
            Arg::with_name("no_scroll")
                .short("n")
                .long("no_scroll")
                .required(false),
        )
        .get_matches();

    let path = matches.value_of("path").unwrap();

    let no_scroll = matches.is_present("no_scroll");

    let (term_width, term_height) = term_size::dimensions().unwrap();

    let term_dimension = if no_scroll { term_height } else { term_width };

    print!("{}", show(image::open(path).unwrap(), term_dimension));
}
