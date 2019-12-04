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

impl Point {
    fn abs(&self) -> isize {
        self.x.abs() + self.y.abs()
    }
}

#[derive(Debug, Clone, Copy)]
struct Line {
    from: Point,
    to: Point,
}

impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from && self.to == other.to
    }
}

impl Line {
    fn height(&self) -> isize {
        self.from.y - self.to.y
    }
    fn width(&self) -> isize {
        self.from.x - self.to.x
    }
    fn min_x(&self) -> isize {
        if self.from.x > self.to.x {
            self.to.x
        } else {
            self.from.x
        }
    }
    fn min_y(&self) -> isize {
        if self.from.y > self.to.y {
            self.to.y
        } else {
            self.from.y
        }
    }
    fn is_vertical(&self) -> bool {
        self.from.x == self.to.x
    }
    fn intersects(&self, other: &Self) -> bool {
        let mut collision: bool = false;
        if self.from == other.to
            || self.to == other.from
            || self.from == other.from
            || self.to == other.to
        {
            collision = false;
        } else if self.min_x() <= other.min_x() + other.width()
            && other.min_x() <= self.min_x() + self.width()
            && self.min_y() <= other.min_y() + other.height()
            && other.min_y() <= self.min_y() + self.height()
        {
            collision = true;
        }
        collision
    }
    fn intersects_at(&self, other: &Self) -> Point {
        if self.is_vertical() {
            Point {
                x: other.from.x,
                y: self.from.y,
            }
        } else {
            Point {
                x: self.from.x,
                y: other.from.y,
            }
        }
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
        let starting_point: Point = current_point;

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

fn find_intersections(lines1: Vec<Line>, lines2: Vec<Line>) -> Result<Vec<Point>, Box<dyn Error>> {
    let mut intersections: Vec<Point> = Vec::new();
    for line1 in &lines1 {
        for line2 in &lines2 {
            let intersects = line1.intersects(&line2);
            if intersects {
                intersections.push(line1.intersects_at(&line2));
            }
        }
    }
    Ok(intersections)
}

fn nearest_from_origin(mut points: Vec<Point>) -> Point {
    let mut nearest: Point = Point { x: 0, y: 0 };
    if points.len() > 0 {
        nearest = points.pop().unwrap();
        loop {
            match points.pop() {
                None => {
                    break;
                }
                Some(point) => {
                    if nearest.abs() > point.abs() {
                        nearest = point;
                    }
                }
            }
        }
    }
    nearest
}

fn main() {
    let file_path: String = String::from("./src/input");
    let prog_input: Vec<StringRecord> = read_input(file_path).unwrap();

    let mut intersections: Vec<Point> = Vec::new();
    let mut wires: Vec<Vec<Line>> = Vec::new();

    for row in prog_input {
        let path: Vec<String> = row.deserialize(None).unwrap();
        wires.push(calc_lines(path).unwrap());
    }

    intersections.append(
        &mut find_intersections(
            wires.get(0).unwrap().to_vec(),
            wires.get(1).unwrap().to_vec(),
        )
        .unwrap(),
    );
    let nearest_intersection = nearest_from_origin(intersections);
    let distance_to_nearest = nearest_intersection.abs();

    println!(
        "The manhattan distance to nearest intersection is {:?}",
        distance_to_nearest
    );
}
