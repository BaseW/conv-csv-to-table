use std::error::Error;
use std::io;
use std::process;

fn read_csv_from_stdin() -> Result<(), Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    if let Err(err) = read_csv_from_stdin() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
