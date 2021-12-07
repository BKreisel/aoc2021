use std::collections::HashMap;

const DAYS: usize = 256;

fn run(input: &'static str) -> usize {
    let mut timers: HashMap<u8, usize> = HashMap::new();

    for x in 0..9 {
        timers.insert(x, 0);
    }

    for timer in input.lines().next().unwrap().split(",") {
        let entry = timers.entry(timer.parse::<u8>().unwrap()).or_insert(0);
        *entry += 1 
    }

    let mut prev: usize;
    let mut cur: usize;
    let mut spawns: usize;

    for _ in 0..DAYS {
        spawns = *timers.get(&0).unwrap();
        timers.insert(0,0);
    
        for key in 1..9 {
            cur = *timers.get(&key).unwrap();
            if cur > 0 {
                prev = *timers.get(&(key -1)).unwrap();
                timers.insert(key -1, cur + prev);
                timers.insert(key, 0);
            }
        }

        if spawns > 0 {
            timers.insert(8, spawns);
            prev = *timers.get(&6).unwrap();
            timers.insert(6, spawns + prev);
        }
    }
    return timers.iter().map(|(_, v)|v).sum();
}

fn main() {
    println!("{}", run(include_str!("../input.txt")));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run(include_str!("../test.txt")), 26984457539);
    }
}