extern crate csv;
use csv::ReaderBuilder;
use csv::StringRecord;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct OpCodeError;

impl fmt::Display for OpCodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid opcode encountered!")
    }
}

impl Error for OpCodeError {
    fn description(&self) -> &str {
        "Invalid opcode encountered!"
    }
}

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

fn run_row(row: Vec<usize>) -> Result<Vec<usize>, OpCodeError> {
    const PROG_SIZE: usize = 4;
    let mut prog: Vec<usize> = row.clone();

    // Restore gravity assist to 1202 program alarm
    prog[1] = 12;
    prog[2] = 2;

    let mut instruction: usize = 0;
    let mut index: usize = instruction * PROG_SIZE;

    while index + 3 < row.len() {
        let opcode = prog[index];
        let input1 = prog[index + 1];
        let input2 = prog[index + 2];
        let output = prog[index + 3];
        match opcode {
            1 => prog[output] = prog[input1] + prog[input2],
            2 => prog[output] = prog[input1] * prog[input2],
            99 => break,
            _ => return Err(OpCodeError),
        }
        instruction += 1;
        index = instruction * PROG_SIZE;
    }

    Ok(prog)
}

fn main() {
    let file_path: String = String::from("./src/input");
    let code_vec: Vec<StringRecord> = read_input(file_path).unwrap();

    let mut row: Vec<usize>;
    let mut result: Vec<usize>;

    for rec in code_vec {
        row = rec.deserialize(None).unwrap();
        result = run_row(row).unwrap();
        println!("Program result: {:?}", result[0]);
    }
}
