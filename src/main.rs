mod process;

use clap::{App, Arg};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("seqsample")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Steven Weaver")
        .about("Reduces sequence duplicates")
        .arg(
            Arg::new("fasta")
                .help("The input FASTA file (gzip acceptable).")
                .takes_value(true)
                .required(true)
                .short('f')
                .long("fasta"),
        )
        .arg(
            Arg::new("number")
                .help("Number of sequences to randomly sample. If the number is less than 1, then it will be treated as a percentage of the dataset.")
                .takes_value(true)
                .required(false)
                .short('n')
                .long("number"),
        )
        .arg(
            Arg::new("store-background")
                .help("Write the samples not selected randomly in a separate file.")
                .takes_value(true)
                .required(false)
                .short('i')
                .long("store-background"),
        )
        .get_matches();

    let mut store_background = Some("");

    if !matches.value_of("store-background").is_none() {
        store_background = Some(matches.value_of("store-background").unwrap());
    }

    crate::process::process(
        matches.value_of("fasta").unwrap(),
        matches.value_of("number").unwrap(),
        store_background.unwrap(),
    )
}
