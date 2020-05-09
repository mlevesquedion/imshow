extern crate image;
extern crate term_size;

use imshow::show;
use imshow::Dimensions;

fn main() {
    let matches = clap::App::new("imshow")
        .version(clap::crate_version!())
        .about("Show images in the terminal.")
        .arg(
            clap::Arg::with_name("path")
                .help("Path of the image file to show.")
                .short("f")
                .long("file")
                .required(true)
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("vertical")
                .help("Fill the terminal's height. (otherwise, fills the terminal's width)")
                .short("v")
                .long("vertical")
                .required(false),
        )
        .get_matches();

    let path = matches.value_of("path").unwrap();
    let vertical = matches.is_present("vertical");

    let (terminal_width, terminal_height) = term_size::dimensions().unwrap();
    let terminal_dimensions = Dimensions {
        width: terminal_width as u32,
        height: terminal_height as u32,
    };

    let image = image::open(path).unwrap();

    print!("{}", show(image, terminal_dimensions, vertical));
}
