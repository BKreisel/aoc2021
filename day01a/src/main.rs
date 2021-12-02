fn run(input: &'static str) -> usize {
    let sonar_depths: Vec<usize> = input
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    return sonar_depths.windows(2)
        .filter(|x| x[1] > x[0])
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
        assert_eq!(run(include_str!("../test.txt")), 7);
    }
}