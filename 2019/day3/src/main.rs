extern crate csv;
use csv::ReaderBuilder;
use csv::StringRecord;
use std::error::Error;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Copy)]
struct Line {
    from: Point,
    to: Point,
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

fn calc_lines(path: Vec<String>) -> Result<Vec<Line>, Box<dyn Error>> {
    let mut lines: Vec<Line> = Vec::new();
    let mut current_point: Point = Point { x: 0, y: 0 };

    for mut instruction in path {
        let op = instruction.remove(0);
        let amount: isize = instruction.parse().unwrap();
        let starting_point: Point = current_point.clone();

        match op {
            'U' => current_point.y += amount,
            'D' => current_point.y -= amount,
            'L' => current_point.x -= amount,
            'R' => current_point.x += amount,
            _ => println!("No direction specified!"),
        }

        let line: Line = Line {
            from: starting_point,
            to: current_point,
        };
        lines.push(line);
    }
    Ok(lines)
}

fn find_intersections(lines: Vec<Line>) -> Result<Vec<Point>, Box<dyn Error>> {}

fn main() {
    let file_path: String = String::from("./src/input");
    let prog_input: Vec<StringRecord> = read_input(file_path).unwrap();
    let mut lines: Vec<Line> = Vec::new();
    for row in prog_input {
        let path: Vec<String> = row.deserialize(None).unwrap();
        let lines_for_input: Vec<Line> = calc_lines(path).unwrap();
    }
}
