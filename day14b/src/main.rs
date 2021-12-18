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
    let template: Vec<char> = iter.next().unwrap().chars().collect();
    let rules: Vec<Rule> = iter.skip(1).map(|x| Rule::from_str(x)).collect();
    
    
    let mut counts: HashMap<char, usize> = HashMap::new();
    for chr in template.iter() {
        *counts.entry(*chr).or_insert(0) += 1;
    }

    let mut pairs: HashMap<Vec<char>, i64> = HashMap::new();
    for window in template.windows(2) {
        *pairs.entry(window.to_vec()).or_insert(0) += 1;
    }

    let mut insertions: HashMap<Vec<char>, i64> = HashMap::new();

    for _ in 0..40 {
        for (k, v) in pairs.iter().filter(|(_,v)| v > &&0) {
            for rule in rules.iter() {
                if &rule.pair == k {
                    *insertions.entry(Vec::from([*&rule.pair[0], rule.insert])).or_insert(0) += *v as i64;
                    *insertions.entry(Vec::from([rule.insert, *&rule.pair[1]])).or_insert(0) += *v as i64;
                    *insertions.entry(k.to_vec()).or_insert(0) -= *v as i64;
                    *counts.entry(rule.insert).or_insert(0) += *v as usize;
                    break;
                }
            }
        } 

        for (k, v) in insertions.iter() {
            *pairs.entry(k.to_vec()).or_insert(0) += *v as i64;
        }
        insertions.clear();

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
        assert_eq!(run(include_str!("../test.txt")), 2188189693529);
    }
}