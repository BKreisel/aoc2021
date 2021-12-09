const ONE_LEN: usize = 2;
const FOUR_LEN: usize = 4;
const SEVEN_LEN: usize = 3;
const EIGHT_LEN: usize = 7;

fn decode_output(line: &str) -> usize {
    let mut iter = line.split(" | ");
    let signals: Vec<Vec<char>> = iter
        .next()
        .unwrap()
        .split(" ")
        .map(|x| x.chars().collect())
        .collect();
    
    let outputs_signals: Vec<Vec<char>> = iter
        .next()
        .unwrap()
        .split(" ")
        .map(|x| x.chars().collect())
        .collect();

    let values = map_values(signals);

    let mut output: usize = 0;

    for (idx, signal) in outputs_signals.iter().rev().enumerate() {
        let mut sorted_signal = signal.to_vec();
        sorted_signal.sort();
        let value_index = values.iter().position(|x| x == &sorted_signal).unwrap();
        
        output += 10usize.pow(idx as u32) * value_index as usize;

    }

    output
}

fn map_values(mut signals: Vec<Vec<char>>) -> Vec<Vec<char>> {

    let mut one = signals.iter().filter(|x| x.len() == ONE_LEN).next().unwrap().to_vec();
    signals.retain(|x| x != &one);

    let mut four = signals.iter().filter(|x| x.len() == FOUR_LEN).next().unwrap().to_vec();
    signals.retain(|x| x != &four);

    let mut seven = signals.iter().filter(|x| x.len() == SEVEN_LEN).next().unwrap().to_vec();
    signals.retain(|x| x != &seven);

    let mut eight = signals.iter().filter(|x| x.len() == EIGHT_LEN).next().unwrap().to_vec();
    signals.retain(|x| x != &eight);

    // 2 3 5 have length 5. 3 has all the "1" segments the others don't
    let mut three = signals
        .iter()
        .filter(|x| x.len() == 5)
        .filter(|y| {
            y
                .iter()
                .filter(|c| one.contains(c))
                .count() == one.len()
        })
        .next()
        .unwrap()
        .to_vec();  
    signals.retain(|x| x != &three);

    // 0 6 9 have length 6. 6 doesn't have all the "1" segments but 0 9 do.
    let mut six = signals
        .iter()
        .filter(|x| x.len() == 6)
        .filter(|y| {
            y
                .iter()
                .filter(|c| one.contains(c))
                .count() != one.len()
        })
        .next()
        .unwrap()
        .to_vec();  
    signals.retain(|x| x != &six);

    // 2 and 5 have len 5.  all of 5 is in 6 but not all of 2
    let mut five = signals
        .iter()
        .filter(|x| x.len() == 5)
        .filter(|y| {
            y
                .iter()
                .filter(|c| six.contains(c))
                .count() == y.len()
        })
        .next()
        .unwrap()
        .to_vec();  
    signals.retain(|x| x != &five);

    // last of len 5 has to be 2
    let mut two = signals
        .iter()
        .filter(|x| x.len() == 5)
        .next()
        .unwrap()
        .to_vec();  
    signals.retain(|x| x != &two);


    // only lens of 6 are left.

    // 0 and 9 left. all of 5 is in 9 (but not all of 0)
    let mut nine = signals
        .iter()
        .filter(|y| {
            y
                .iter()
                .filter(|c| five.contains(c))
                .count() == five.len()
        })
        .next()
        .unwrap()
        .to_vec();  
    signals.retain(|x| x != &nine);

    // gotta b 0
    let mut zero = signals
    .iter()
    .next()
    .unwrap()
    .to_vec();  

    zero.sort();
    one.sort();
    two.sort();
    three.sort();
    four.sort();
    five.sort();
    six.sort();
    seven.sort();
    eight.sort();
    nine.sort();

    return Vec::from([zero, one, two, three, four, five, six, seven, eight, nine]);
}

fn run(input: &'static str) ->  usize {
    return input
        .lines()
        .map(|x| decode_output(x))
        .sum()
}

fn main() {
    println!("{}", run(include_str!("../input.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run(include_str!("../test.txt")), 61229);
    }
}