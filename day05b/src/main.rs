use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone, Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Point {
    pub fn from_str(tuple: &str) -> Point {
        let mut iter = tuple.split(",").map(|x| x.parse::<usize>().unwrap());
        Point {x: iter.next().unwrap(), y: iter.next().unwrap()}
    }
}

impl Line {
    pub fn from_str(line: &str) -> Line {
        let mut iter = line.split(" -> ");
        Line {
            start: Point::from_str(iter.next().unwrap()),
            end: Point::from_str(iter.next().unwrap()),
        }
    }

    #[allow(dead_code)]
    pub fn is_straight(&self) -> bool {
        if self.start.x == self.end.x || self.start.y == self.end.y {
            return true
        }
        false
    }

    pub fn points(&self) -> Vec<Point> {
        let x_pos: bool = self.end.x >= self.start.x;
        let y_pos: bool = self.end.y >= self.start.y;
        let mut current = self.start;
        let mut points: Vec<Point> = vec!(self.start);
        
        loop {
            if current.x == self.end.x && current.y == self.end.y {
                break;
            }

            if current.x != self.end.x {
                current.x = if x_pos {current.x + 1} else {current.x - 1};
            }
            if current.y != self.end.y {
                current.y = if y_pos {current.y + 1} else {current.y - 1};
            }
            points.push(current);
        }
        return points;
    }
}

fn run(input: &'static str) ->  usize {
    let lines: Vec<Line> = input.lines()
        .map(|x| Line::from_str(x))
        .collect();

    let mut map: HashMap<Point, usize> = HashMap::new();

    for line in lines {
        for point in line.points() {
            let count = map.entry(point).or_insert(0);
            *count += 1
        }
    }

    return map.iter()
        .filter(|&(_, v)| v >= &2)
        .count();
}

fn main() {
    println!("{}", run(include_str!("../input.txt")));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run(include_str!("../test.txt")), 12);
    }
}