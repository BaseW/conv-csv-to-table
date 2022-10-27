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
    let mut table = String::new();

    for (row_index, result) in records.into_iter().enumerate() {
        // 各行の処理開始
        let record = result?;
        let mut row = String::new();

        // FIXME: 最初の行以外では使わない値
        let mut col_count = 0;

        for (col_index, field) in record.into_iter().enumerate() {
            // 各列の処理開始
            let mut col = String::new();

            // FIXME: 最初の行以外では使わない値
            col_count = col_count + 1;

            if col_index == 0 {
                // 最初の列だけ処理を変える
                col.push_str(&format!("| {} |", field));
            } else {
                // 最初以外の列の処理
                col.push_str(&format!(" {} |", field));
            }

            row.push_str(&col);
        }

        if row_index == 0 {
            // 最初の行のデータを追加する前に、仮のヘッダーを追加する
            for col_index in 0..col_count {
                if col_index == 0 {
                    // 最初の列だけ処理を変える
                    table.push_str("| column1 |");
                } else {
                    table.push_str(&format!(" column{} |", col_index + 1));
                }
            }
            // 改行を追加
            table.push('\n');

            for col_index in 0..col_count {
                if col_index == 0 {
                    // 最初の列だけ処理を変える
                    table.push_str("| --- |");
                } else {
                    table.push_str(" --- |");
                }
            }
            // 改行を追加
            table.push('\n');
        }

        // 各行のデータを追加
        table.push_str(&row);

        // 改行を追加
        table.push('\n');
    }

    Ok(table)
}

fn get_table_from_stdin() -> Result<String, Box<dyn Error>> {
    let records = read_csv_from_stdin();
    convert_csv_to_table(records)
}

fn main() {
    match get_table_from_stdin() {
        Ok(table) => {
            println!("{}", table);
        }
        Err(err) => {
            println!("error running example: {}", err);
            process::exit(1);
        }
    }
}
