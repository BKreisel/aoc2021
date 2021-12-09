const ONE_LEN: usize = 2;
const FOUR_LEN: usize = 4;
const SEVEN_LEN: usize = 3;
const EIGHT_LEN: usize = 7;

const UNIQUE_LENS: [usize; 4] = [ONE_LEN, FOUR_LEN, SEVEN_LEN, EIGHT_LEN];

#[derive(Debug)]
struct OutputValue {
    chars: Vec<char>
}

impl OutputValue {
    pub fn from_str(input: &str) -> OutputValue {
        OutputValue { chars: input.chars().collect()}
    }

    pub fn len(&self) -> usize {
        self.chars.len()
    }
}

fn run(input: &'static str) ->  usize {
    let output_iter = input
        .lines()
        .map(|line| {
            line
                .rsplit(" | ")
                .next()
                .unwrap()
        });
       
    let outputs: Vec<Vec<OutputValue>> = output_iter
        .map(|out_str| {
            out_str
                .split(" ")
                .map(|chars| OutputValue::from_str(chars))
                .collect()
        })
        .collect();

    return outputs
        .iter()
        .flat_map(|x|x.iter())
        .filter(|output| UNIQUE_LENS.contains(&output.len()))
        .count();
}

fn main() {
    println!("{}", run(include_str!("../input.txt")));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run(include_str!("../test.txt")), 26);
    }
}