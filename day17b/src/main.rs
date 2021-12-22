#[derive(Debug)]
struct TargetArea {
    min: Point,
    max: Point,
}

#[derive(Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y}
    }
}

impl TargetArea {
    fn inside(&self, point: &Point) -> bool {
        if point.x >= self.min.x 
            && point.x <= self.max.x 
            && point.y >= self.min.y
            && point.y <= self.max.y {
                return true;
            }
        return false;
    }
}

struct Probe {
    point: Point,
    velocity: Point
}

impl Probe {
    pub fn new(initial_velocity: Point) -> Probe {
        Probe {
            point: Point::new(0,0),
            velocity: initial_velocity,
        }
    }

    fn step(&mut self) {
        self.point.x += self.velocity.x;
        self.point.y += self.velocity.y;

        match self.velocity.x {
            i32::MIN..=-1 => { self.velocity.x += 1},
            0 => {},
            1..=i32::MAX => {self.velocity.x -= 1}
        }
        self.velocity.y -= 1;
    }

    
    pub fn run(&mut self, target: &TargetArea) -> bool {
        while self.point.x <= target.max.x && self.point.y >= target.min.y {
            self.step();
            if target.inside(&self.point) {
                return true;
            }
        }
        return false;
    }
}

fn run(input: &'static str) ->  u32 {
    let mut iter = input.lines().next().unwrap().split(": ").skip(1).next().unwrap().split(", ");
    let mut x_iter = iter.next().unwrap().split("..");
    let mut y_iter = iter.next().unwrap().split("..");

    let x_min = x_iter.next().unwrap()[2..].parse::<i32>().unwrap();
    let x_max = x_iter.next().unwrap().parse::<i32>().unwrap();
    let y_min = y_iter.next().unwrap()[2..].parse::<i32>().unwrap();
    let y_max = y_iter.next().unwrap().parse::<i32>().unwrap();

    let target = TargetArea {
        min: Point::new(x_min, y_min),
        max: Point::new(x_max, y_max),
    };

    let x_start: i32  = -500;
    let x_stop: i32 =  1000;
    let y_start: i32 = -500;
    let y_stop: i32 = 1000;

    let mut count: u32 = 0;

    for vx in x_start..=x_stop {
        for vy in y_start..=y_stop {
           if Probe::new(Point::new(vx, vy)).run(&target) {
                count += 1;
           }
        }
    }

    return count;
}

fn main() {
    println!("{}", run(include_str!("../input.txt")));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run(include_str!("../test.txt")), 112);
    }
}