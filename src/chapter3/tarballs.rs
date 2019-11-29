extern crate flate2;
extern crate tar;

use std::fs::File;
use std::io::prelude::*;
use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use tar::Archive;

pub fn create_a_tarball() -> Result<(), std::io::Error>{
    // Create a directory
    std::fs::create_dir("stuff_to_zip")?;
    // Fill it up with some text files
    for i in 1..10 {
        let mut file = std::fs::File::create(format!("stuff_to_zip/{}.txt", i))?;
        file.write_all(b"Hello World")?;
    }
    
    // Get size of dir
    let dir_metadata = std::fs::metadata("stuff_to_zip")?;
    let directory_size = dir_metadata.len();

    // Create archive and add files to it.
    let tar_gz = File::create("archive.tar.gz")?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all("backup/", "stuff_to_zip")?;

    // Get size of zip
    let archive_metadata = std::fs::metadata("archive.tar.gz")?;
    let zip_size = archive_metadata.len();

    // Print size of original file and zip
    let compression_factor = directory_size / zip_size;
    println!("File size reduced from {}->{}, for a compression factor of {}", directory_size, zip_size, compression_factor);
    
    Ok(())
}


pub fn decompress_tarball(path: String) -> Result<(), std::io::Error> {
    let tar_gz = File::open(path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(".")?;

    Ok(())
}

pub fn clean_up_files() -> Result<(), std::io::Error> {

    std::fs::remove_dir_all("stuff_to_zip")?;

    std::fs::remove_file("archive.tar.gz")?;

    Ok(())
}