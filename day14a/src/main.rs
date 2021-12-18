use std::collections::HashMap;

#[derive(Debug)]
struct Rule {
    pair: Vec<char>,
    insert: char,
}

impl Rule {
    pub fn from_str(input: &str) -> Rule {
        let mut iter = input.split(" -> ");
        Rule {
            pair: iter.next().unwrap().chars().collect(),
            insert: iter.next().unwrap().chars().next().unwrap(),
        }
    }
}

fn run(input: &'static str) ->  usize {
    let mut iter = input.lines();
    let mut template: Vec<char> = iter.next().unwrap().chars().collect();
    let rules: Vec<Rule> = iter.skip(1).map(|x| Rule::from_str(x)).collect();

    let mut insertions: Vec<(usize, char)> = vec!();

    for _ in 0..10 {
        for (idx, window) in template.windows(2).enumerate() {
            for rule in rules.iter() {
                if window == rule.pair {
                    insertions.push(((idx * 2) + 1, rule.insert));
                }
            }
        }

        for (idx, chr) in insertions.iter() {
            template.insert(*idx, *chr);
        }
        insertions.clear();
    }
    
    let mut counts: HashMap<char, usize> = HashMap::new();
    for chr in template {
        *counts.entry(chr).or_insert(0) += 1;
    }

    let max = counts.iter().max_by_key(|(_, v)| *v).map(|(k, _)| k).unwrap();
    let min = counts.iter().min_by_key(|(_, v)| *v).map(|(k, _)| k).unwrap();

    return counts[&max] - counts[&min];
}

fn main() {
    println!("{}", run(include_str!("../input.txt")));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run(include_str!("../test.txt")), 1588);
    }
}