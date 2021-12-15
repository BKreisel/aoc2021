#[derive(Debug, PartialEq)]
enum Axis {
    X,
    Y,
}

impl Axis {
    pub fn from_char(axis: char) -> Axis {
        if axis == 'x' { Axis::X } else { Axis::Y }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Fold {
    axis: Axis,
    magnitude: usize
}

impl Fold {
    pub fn from_str(input: &str) -> Fold {
        let mut iter = input.split("=");
        let axis_char = iter.next().unwrap().chars().rev().next().unwrap();
        Fold {
            axis: Axis::from_char(axis_char),
            magnitude: iter.next().unwrap().parse::<usize>().unwrap(),
        }
    }    
}

impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        Point { x: x, y: y}
    }

    pub fn from_str(input: &str) -> Point {
        let mut iter = input.split(",");
        Point {
            x: iter.next().unwrap().parse::<usize>().unwrap(),
            y: iter.next().unwrap().parse::<usize>().unwrap(),
        }
    }
}

fn run(input: &'static str) ->  Vec<Point> {
    let mut iter = input.lines();
    let mut points: Vec<Point> = iter
        .by_ref()
        .take_while(|x| x.len() > 0)
        .map(|x| Point::from_str(x))
        .collect();
    
    let folds: Vec<Fold> = iter
        .map(|x| Fold::from_str(x))
        .collect();

    let mut new_points: Vec<Point>;

    for fold in folds.iter() {
        if fold.axis == Axis::X {
            new_points = points
                .iter()
                .filter(|p| p.x > fold.magnitude)
                .map(|p| {
                    Point::new(fold.magnitude - (p.x - fold.magnitude), p.y)
                })
                .collect();
            points.retain(|p| p.x < fold.magnitude);
        } else {
            new_points = points
            .iter()
            .filter(|p| p.y > fold.magnitude)
            .map(|p| {
                Point::new(p.x, fold.magnitude - (p.y - fold.magnitude))
            })
            .collect();
        points.retain(|p| p.y < fold.magnitude);
        }

        points.append(&mut new_points);
        points.sort();
        points.dedup();
    }

    return points;
}

#[allow(dead_code)]
fn print_points(points: &Vec<Point>) {
    let width = points.iter().map(|p| p.x).max().unwrap();
    let height = points.iter().map(|p| p.y).max().unwrap();

    for y in 0..height+1 {
        for x in 0..width+1 {
            let mut chr = ".";
            if points.contains(&Point::new(x,y)) {
                chr = "#";
            }
            print!("{}", chr);
        }
        println!("");
    }
}

fn main() {
    print_points(&run(include_str!("../input.txt")));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run(include_str!("../test.txt")), 0);
    }
}