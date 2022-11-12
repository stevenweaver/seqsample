use bio::io::fasta;
use std::error::Error;
use std::ffi::OsStr;
use std::io;
use std::path::Path;
use std::str;
use rand::{seq::IteratorRandom, thread_rng};

// String Match Option
pub fn random_sample<P: AsRef<Path> + AsRef<OsStr>>(filename: P, number: usize) -> bool {

    let mut rng = thread_rng();

    //FASTA file related
    let file = Path::new(&filename).to_str().unwrap();
    let records = fasta::Reader::from_file(file).unwrap().records();

    // write JSON file
    let mut writer = fasta::Writer::new(io::stdout());

    let sampled_records = records.choose_multiple(&mut rng, number);

    for seqrec in sampled_records {

        let record = seqrec.unwrap();

        //let sequence_id_bytes = record.id();
        //let sequence_description_bytes = record.desc().unwrap_or("");
        //let entire_header_raw = [sequence_id_bytes, sequence_description_bytes].join(" ");
        //let entire_header = entire_header_raw.trim();
        //println!("I am {}!", entire_header);

        writer
            .write(record.id(), record.desc(), record.seq())
            .expect("Error writing record.");
    }

    return true;

}

pub(crate) fn process<P: AsRef<Path> + AsRef<OsStr>>(
    filename: P,
    number_arg: &str
) -> Result<(), Box<dyn Error>> {
    let number =  number_arg.parse::<usize>().unwrap();
    random_sample(filename, number);
    Ok(())
}
