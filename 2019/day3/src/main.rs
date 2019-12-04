extern crate csv;
use csv::ReaderBuilder;
use csv::StringRecord;
use std::error::Error;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug, Clone, Copy)]
struct Line {
    from: Point,
    to: Point,
}

impl Line {
    fn is_vertical(&self) -> bool {
        self.from.x == self.to.x
    }
    fn intersects(&self, other: &Self) -> bool {
        let mut collision: bool = false;
        if self.from == other.from || self.to == other.to {
            collision = false;
        }
        if self.is_vertical() == other.is_vertical() {
            collision = false;
        }
        if (self.from.x < other.to.x && self.from.x > other.from.x
            || self.from.x > other.to.x && self.from.x < other.from.x)
            && (self.from.y < other.from.y && self.to.y > other.from.y
                || self.from.y > other.from.y && self.to.y < other.from.y)
        {
            collision = true;
        }
        collision
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

fn find_intersections(mut lines: Vec<Line>) -> Result<Vec<Point>, Box<dyn Error>> {
    let mut intersections: Vec<Point> = Vec::new();
    loop {
        match lines.pop() {
            None => {
                println!("No more lines to check!");
                break;
            }
            Some(line) => {
                for checked_line in &lines {
                    let intersects = checked_line.intersects(&line);
                    if intersects {
                        println!("Intersection found for {:?} and {:?}!", line, checked_line);
                    //intersections.push(calc_intersection(checked_line, line));
                    } else {
                        println!("No intersections found!");
                    }
                }
            }
        }
    }
    Ok(intersections)
}

fn main() {
    let file_path: String = String::from("./src/input");
    let prog_input: Vec<StringRecord> = read_input(file_path).unwrap();
    for row in prog_input {
        let path: Vec<String> = row.deserialize(None).unwrap();
        let lines_for_input: Vec<Line> = calc_lines(path).unwrap();
        let intersections_for_input: Vec<Point> = find_intersections(lines_for_input).unwrap();
    }
}
