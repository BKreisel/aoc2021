enum Direction {
    Forward,
    Up,
    Down
}

struct Action {
    direction: Direction,
    magnitute: usize,
}

#[derive(Debug)]
struct Submarine {
    depth: i32,
    position: i32,
}

impl Submarine {
    pub fn new() -> Submarine {
        Submarine {depth: 0, position: 0}
    }

    pub fn take_action(&mut self, action: &Action) {
        match action.direction {
            Direction::Up => {self.depth -= action.magnitute as i32},
            Direction::Down => {self.depth += action.magnitute as i32},
            Direction::Forward => {self.position += action.magnitute as i32},
        }
    }

    pub fn multiply_final(&self) -> i32 {
        self.depth * self.position
    }
}

impl Action {
    pub fn from_str(string: &str) -> Action {
        let mut iter = string.split(" ");
        
        let direction = match iter.next().unwrap() {
            "forward" => {Direction::Forward},
            "down" => {Direction::Down},
            "up" => {Direction::Up},
            _ => panic!("Bad Direction"),
        };

        let magnitude = iter.next()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        Action {direction: direction, magnitute: magnitude}
    }
}

fn run(input: &'static str) -> i32 {
    let sub_course: Vec<Action> = input
        .lines()
        .map(|line| Action::from_str(line))
        .collect();
    
    let mut sub = Submarine::new();
    for action in sub_course {
        sub.take_action(&action);
    }

    sub.multiply_final()
}

fn main() {
    println!("{}", run(include_str!("../input.txt")));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run(include_str!("../test.txt")), 150);
    }
}