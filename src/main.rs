extern crate clap;
extern crate rand;
use clap::{
    crate_authors, crate_description, crate_name, crate_version, value_t, values_t, App, Arg,
    SubCommand,
};
use rand::seq::SliceRandom;
use std::io::{stdin, stdout, Write};

fn main() {
    let args = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(
            SubCommand::with_name("chord")
                .about("picks a random chord with extension")
                .arg(
                    Arg::with_name("loop")
                        .short("l")
                        .long("loop")
                        .help("runs the generator on a loop, moving forward every keypress"),
                ),
        )
        .get_matches();

    if let Some(matches) = args.subcommand_matches("chord") {
        if matches.is_present("loop") {
            println!("Printing debug info...");
            loop {
                println!("{:?}", rand_chord());
                let mut s = String::new();
                let _ = stdout().flush();
                stdin()
                    .read_line(&mut s)
                    .expect("Did not enter a correct string");
            }
        } else {
        }
    }
}

fn notes() -> String {
    let bases = vec!["A", "B", "C", "D", "E", "F", "G"];
    let augs = vec!["#", "b", ""];
    let base_choice = bases.choose(&mut rand::thread_rng()).unwrap();
    let aug_choice = augs.choose(&mut rand::thread_rng()).unwrap();
    format!("{}{}", base_choice, aug_choice)
}

fn extensions() -> String {
    let bases = vec!["maj", "min", "dim", "aug"];
    let augs = vec!["7", "9", "13", "11", "b9"];
    let base_choice = bases.choose(&mut rand::thread_rng()).unwrap();
    let aug_choice = augs.choose(&mut rand::thread_rng()).unwrap();
    format!("{}{}", base_choice, aug_choice)
}

fn rand_chord() -> String {
    format!("{} {}", notes(), extensions())
}
