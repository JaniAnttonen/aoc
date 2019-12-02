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

fn run_row(row: Vec<usize>, noun: usize, verb: usize) -> Result<Vec<usize>, OpCodeError> {
    const PROG_SIZE: usize = 4;
    let mut prog: Vec<usize> = row.clone();

    // Restore gravity assist to 1202 program alarm
    prog[1] = noun;
    prog[2] = verb;

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

    let row: Vec<usize> = code_vec[0].deserialize(None).unwrap();

    let result1: Vec<usize> = run_row(row.clone(), 12, 2).unwrap();
    println!("Part One: {:?}", result1[0]);

    let mut corr_noun: usize = 0;
    let mut corr_verb: usize = 0;
    const WANTED_RESULT: usize = 19690720;
    let mut result2: Vec<usize>;

    for noun in 0..99 {
        for verb in 0..99 {
            result2 = run_row(row.clone(), noun, verb).unwrap();
            if result2[0] == WANTED_RESULT {
                corr_noun = noun;
                corr_verb = verb;
                break;
            }
        }
    }

    println!("Part Two: {}", 100 * corr_noun + corr_verb);
}
