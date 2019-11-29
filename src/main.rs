extern crate clap;
extern crate ansi_term;

mod chapter1;
mod chapter3;

use clap::{Arg, App};
use ansi_term::{Color, Style};


fn run_chapter_1() {
    println!("{}", Color::Yellow.bold().paint("Running Chapter 1"));
    chapter1::chapter_1_random_nums();
    chapter1::chapter_1_vec_sort();
    println!("{}", Color::Green.bold().paint("Finished Chapter 1"));
}

fn run_chapter_2() {
    println!("{}", Color::Green.bold().paint("Chapter 2 is the CLI you're running right now!"));
}

fn run_chapter_3() {
    println!("{}", Color::Yellow.bold().paint("Running Chapter 3"));
    chapter3::run_chapter_3_code();
    println!("{}", Color::Green.bold().paint("Finished Chapter 3"));
}

fn main() {
    type ChapFn = fn();
    let chapfns:Vec<ChapFn> = vec![
        run_chapter_1,
        run_chapter_2,
        run_chapter_3
    ];

    let matches = App::new("Andy's Rust Cookbook")
        .version("0.1.0")
        .author("Andy")
        .about("Learning Rust")
        .arg(Arg::with_name("chapter")
                    .multiple(true)
                    .short("c")
                    .long("chapter")
                    .takes_value(true)
                    .help("Chapter number to run"))
        .get_matches();

        if let Some(chapters) = matches.values_of("chapter") {
            for chapter in chapters {
                let chapter_num: i32 = chapter.parse::<i32>().unwrap();
                
                if chapter_num as usize > chapfns.len() {
                    println!("{}", Color::Red.bold().paint(format!("Oops, haven't done chapter {} yet", chapter_num)));
                } else {
                    chapfns[chapter_num as usize - 1]();
                }
            }
        }
}
