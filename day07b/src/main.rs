fn run(input: &'static str) ->  usize {
    let crab_subs: Vec<usize> = input
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|x|x.parse::<usize>().unwrap())
        .collect();

    let min = *crab_subs.iter().min().unwrap();
    let max = *crab_subs.iter().max().unwrap();

    let mut fuel: usize;
    let mut min_fuel: usize = usize::MAX;

    for pos in min..(max+1) {
        fuel = 0;
        for sub in crab_subs.iter() {
            let distance = ((*sub as i32) - (pos as i32)).abs() as i32;
            fuel += integer_sum(distance);
        }
        if fuel < min_fuel {
            min_fuel = fuel;
        }
    }
    return min_fuel
}

fn integer_sum(n: i32) -> usize {
    return ((((n * (n - 1)) / 2)) + n) as usize;
}

fn main() {
    println!("{}", run(include_str!("../input.txt")));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run(include_str!("../test.txt")), 168);
    }
}