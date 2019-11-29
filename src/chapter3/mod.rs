pub mod tarballs;

use ansi_term::Color;

pub fn run_chapter_3_code() {
    match tarballs::create_a_tarball() {
        Ok(_v) => println!("Made A Tar Archive!"),
        Err(e) => println!("{}", Color::Red.paint(format!("{:?}", e))),
    }

    match tarballs::decompress_tarball("archive.tar.gz".to_string()) {
        Ok(_v) => println!("Tarball Decompressed!"),
        Err(e) => println!("{}", Color::Red.paint(format!("{:?}", e))),
    }

    match tarballs::clean_up_files() {
        Ok(_v) => println!("Files cleaned up."),
        Err(e) => println!("{}", Color::Red.paint(format!("{:?}", e))),
    }
}