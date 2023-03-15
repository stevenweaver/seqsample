use bio::io::fasta;
use rand::{seq::IteratorRandom, thread_rng};
use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::path::Path;
use std::str;

fn is_path_empty<P: AsRef<Path>>(path: P) -> bool {
    return path.as_ref().to_string_lossy().to_string().is_empty();
}

// String Match Option
pub fn random_sample<P: AsRef<Path> + AsRef<OsStr>>(
    filename: P,
    number: f32,
    store_background: P,
) -> bool {
    let mut null_file = OpenOptions::new().write(true).open("/dev/null").unwrap();

    let mut background_writer = fasta::Writer::new(io::BufWriter::new(null_file));
    let mut rng = thread_rng();

    //FASTA file related
    let file = Path::new(&filename).to_str().unwrap();

    let records = fasta::Reader::from_file(file).unwrap().records();
    let records2 = fasta::Reader::from_file(file).unwrap().records();
    let records3 = fasta::Reader::from_file(file).unwrap().records();

    // If store_background is set, then write to the file

    if !is_path_empty(&store_background) {
        let background_filename = Path::new(&store_background).to_str().unwrap();
        let bg_file = fs::File::create(background_filename).unwrap();
        let handle = io::BufWriter::new(bg_file);
        background_writer = fasta::Writer::new(handle);
    }

    // write JSON file
    let mut writer = fasta::Writer::new(io::stdout());

    // Check if the number is less than 1, if so, then treat it as a percentage
    let whole_number: usize;

    if number < 1.0 {
        //Get proprotion
        let count: f32 = records3.count() as f32;
        whole_number = (count * number) as usize;
    } else {
        whole_number = number as usize;
    }

    let sampled_records = records.choose_multiple(&mut rng, whole_number);

    let sampled_record_ids: String = sampled_records
        .into_iter()
        .map(|x| String::from(x.unwrap().id()))
        .collect();

    // Get records that are note part of choose
    for record in records2 {
        let record = record.unwrap();

        if sampled_record_ids.contains(record.id()) {
            //let sequence_id_bytes = record.id();
            //let sequence_description_bytes = record.desc().unwrap_or("");
            //let entire_header_raw = [sequence_id_bytes, sequence_description_bytes].join(" ");
            //let entire_header = entire_header_raw.trim();
            //println!("I am {}!", entire_header);

            writer
                .write(record.id(), record.desc(), record.seq())
                .expect("Error writing record.");
        } else {
            background_writer
                .write(record.id(), record.desc(), record.seq())
                .expect("Error writing record.");
        }
    }

    return true;
}

pub(crate) fn process<P: AsRef<Path> + AsRef<OsStr>>(
    filename: P,
    number_arg: &str,
    store_background: P,
) -> Result<(), Box<dyn Error>> {
    let number = number_arg.parse::<f32>().unwrap();
    random_sample(filename, number, store_background);
    Ok(())
}
