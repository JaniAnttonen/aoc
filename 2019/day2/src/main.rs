extern crate csv;
use csv::ReaderBuilder;
use csv::StringRecord;
use std::error::Error;
//use std::str::FromStr;

fn read_input(file_path: String) -> Result<Vec<StringRecord>, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .from_path(file_path)?;
    let mut output: Vec<StringRecord> = Vec::new();
    for result in reader.records() {
        let record = result?;
        output.push(record);
    }
    Ok(output)
}

fn run_row(row: StringRecord) -> Result<Vec<i32>, Box<dyn Error>> {
    let program_state: Vec<i32> = row.deserialize(None).unwrap();
    Ok(program_state)
}

fn main() {
    let file_path: String = String::from("./src/input");
    let code_vec: Vec<StringRecord> = read_input(file_path).unwrap();
    let mut row: StringRecord;
    let mut result: Vec<i32>;

    for rec in code_vec {
        row = rec;
        result = run_row(row).unwrap();
    }
}
