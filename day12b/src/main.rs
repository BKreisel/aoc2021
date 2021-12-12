use std::collections::HashMap;

fn big(name: &str) -> bool {
    if name.chars().next().unwrap().is_ascii_uppercase() { 
        return true;
    }
    return false;
}

fn traverse_twice(caves: &HashMap<String, Vec<String>>, name: &str, visited: &mut Vec<String>, twice: &str, seen: &mut bool) -> usize {
    if name == "end" {
        return 1;
    }
    if !big(name) {
        if name == twice {
            if *seen {
                visited.push(String::from(name));
            }
            *seen = true;
        } else {
            visited.push(String::from(name));
        }   
    }
    let next = &caves.get(name).unwrap();
    let mut sum: usize = 0;
    for cave in next.iter() {
        if !visited.contains(&cave) {
            sum += traverse_twice(caves, cave, &mut visited.clone(), twice, &mut seen.clone());
        }
    }
    return sum;
}


fn run(input: &'static str) -> usize {
    let mut caves: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.lines() {
        let mut iter = line.split("-");
        let name = String::from(iter.next().unwrap());
        let edge = String::from(iter.next().unwrap());

        let cave = caves.entry(name.clone()).or_insert(vec!());
        cave.push(edge.clone());

        let cave = caves.entry(edge.clone()).or_insert(vec!());
        cave.push(name);
    }

    let small_iter = caves
        .keys()
        .filter(|k| k != &"end" && k != &"start" && !big(k));

    let once = traverse_twice(&caves, "start", &mut vec!(), "throwaway", &mut false);

    let mut twice: usize = 0;
    for small_cave in small_iter {
        twice += traverse_twice(&caves, "start", &mut vec!(), small_cave, &mut false) - once
    }
    return once + twice;
}

fn main() {
    println!("{}", run(include_str!("../input.txt")));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run(include_str!("../test.txt")), 36);
    }
}