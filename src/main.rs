use csv::StringRecord;
use std::error::Error;
use std::io;
use std::process;

fn read_csv_from_stdin<'a>() -> Vec<Result<StringRecord, csv::Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_reader(io::stdin());
    rdr.records().collect::<Vec<_>>()
}

fn convert_csv_to_table(
    records: Vec<Result<StringRecord, csv::Error>>,
) -> Result<String, Box<dyn Error>> {
    for result in records {
        let record = result?;
        println!("{:?}", record);
    }
    Ok("ok".to_string())
}

fn get_table_from_stdin() -> Result<String, Box<dyn Error>> {
    let records = read_csv_from_stdin();
    convert_csv_to_table(records)
}

fn main() {
    if let Err(err) = get_table_from_stdin() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
