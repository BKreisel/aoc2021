use std::collections::HashMap;

fn big(name: &str) -> bool {
    if name.chars().next().unwrap().is_ascii_uppercase() { 
        return true;
    }
    return false;
}

fn traverse(caves: &HashMap<String, Vec<String>>, name: &str, visited: &mut Vec<String>) -> usize {
    if name == "end" {
        return 1;
    }
    if !big(name) {
        visited.push(String::from(name));
    }
    let next = &caves.get(name).unwrap();
    let mut sum: usize = 0;
    for cave in next.iter() {
        if !visited.contains(&cave) {
            sum += traverse(caves, cave, &mut visited.clone());
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
    
    traverse(&caves, "start", &mut vec!())   
}

fn main() {
    println!("{}", run(include_str!("../input.txt")));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run(include_str!("../test.txt")), 10);
    }
}