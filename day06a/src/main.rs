const DAYS: usize = 80;

#[derive(Debug)]
struct LanternFish {
    timer: usize,
    spawning: bool,
}

impl LanternFish {
    pub fn default() -> LanternFish {
        LanternFish {timer: 8, spawning: false}
    }

    pub fn new(time: usize) -> LanternFish {
        LanternFish { timer: time, spawning: false}
    }

    fn reset(&mut self) {
        self.timer = 6;
        self.spawning = false;
    }

   pub fn new_day(&mut self) {
       if self.timer == 0 {
           self.spawning = true;
       } else {
           self.timer -= 1;
       }
   }

   pub fn spawn(&mut self) -> bool {
       if !self.spawning {
           return false;
       }
       self.reset();
       return true;
   }

}

fn run(input: &'static str) -> usize {
    let initial_fish: Vec<usize> = input
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    
    let mut fishes: Vec<LanternFish> = initial_fish
        .iter()
        .map(|x| LanternFish::new(*x))
        .collect();
    
    for _ in 0..DAYS {
        let mut spawns: usize = 0;
        for fish in fishes.iter_mut() {
            fish.new_day();
            if fish.spawn() {
                spawns += 1;
            }
        }
        for _ in 0..spawns {
            fishes.push(LanternFish::default());
        }

    }

    return fishes.len();
}

fn main() {
    println!("{}", run(include_str!("../input.txt")));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run(include_str!("../test.txt")), 5934);
    }
}