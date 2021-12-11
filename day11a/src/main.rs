#[derive(Debug)]
struct Octopus {
    pub energy: u8,
    pub flashes: usize,
    flashed: bool,
}

fn flat_index(x: usize, y: usize) -> usize {
    return (y * 10) + x;
}

impl Octopus {
    pub fn from_char(energy: char) -> Octopus {
        Octopus {
            energy: energy.to_digit(10).unwrap() as _,
            flashes: 0,
            flashed: false,
        }
    }

    pub fn step(&mut self) -> bool {
        if !self.flashed {
            self.energy += 1;
            if self.energy > 9 {
                self.energy = 0;
                self.flashes += 1;
                self.flashed = true;
                return true;
            }
        }
        return false;
    }

    pub fn reset(&mut self) {
        self.flashed = false;
    }
}

fn step_walk(octopi: &mut Vec<Octopus>, friend_map: &Vec<Vec<usize>>, index: usize) {
    let octopus = octopi.get_mut(index).unwrap();
    let neighbors = friend_map.get(index).unwrap();

    if octopus.step() {
        for neighbor in neighbors {
            step_walk(octopi, friend_map, *neighbor);
        }
    }
}

fn run(input: &'static str) ->  usize {
    let mut octopi: Vec<Octopus> = Vec::with_capacity(100);
    let mut friend_map: Vec<Vec<usize>> = Vec::with_capacity(100);

    for (y, line) in input.lines().enumerate() {
        for (x, energy) in line.chars().enumerate() {
            let mut friends: Vec<usize> = Vec::with_capacity(8);

            // Diag Top-Left
            if y != 0 && x != 0 { friends.push(flat_index(x-1, y-1)); }
            // Top
            if y != 0 { friends.push(flat_index(x, y-1)); }
            // Diag Top-Right
            if y != 0 && x < 9 { friends.push(flat_index(x+1, y-1)); }
            // Right
            if x < 9 { friends.push(flat_index(x+1, y)); }
            // Diag Bottom-Right
            if y < 9 && x < 9 { friends.push(flat_index(x+1, y+1)); }
            // Bottom
            if y < 9 { friends.push(flat_index(x, y+1)); }
            // Diag Bottom-Left
            if y < 9 && x != 0 { friends.push(flat_index(x-1, y+1)); }
            // Left
            if x != 0  { friends.push(flat_index(x-1, y)); }
            
            octopi.push(Octopus::from_char(energy));
            friend_map.push(friends);
        }
    }

    for _ in 0..100 {
        for octopus in octopi.iter_mut() {
            octopus.reset();
        }
        for index in 0..100 {
            step_walk(&mut octopi, &friend_map, index);
        }
    }

    return octopi
        .iter()
        .map(|x| x.flashes)
        .sum();
}

fn main() {
    println!("{}", run(include_str!("../input.txt")));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run(include_str!("../test.txt")), 1656);
    }
}