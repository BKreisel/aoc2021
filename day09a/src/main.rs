

fn run(input: &'static str) -> usize {
    let map: Vec<Vec<u8>> = input
        .lines()
        .map(|line|{
            line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    // y down/up
    // x left/right

    let low_points: Vec<u8> = map
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row
                .iter()
                .enumerate()
                .filter(|(x, col)|{
                    col < &neighbors(&map, *x, y).iter().min().unwrap()
                })
                .map(|(_, col)| *col)
                .collect::<Vec<u8>>()
        })
        .flat_map(|x| x)
        .collect();

    // risk level is height + 1
    low_points
        .iter()
        .map(|x| (x + 1) as usize)
        .sum()
}

fn neighbors(map: &Vec<Vec<u8>>, x: usize, y: usize) -> Vec<u8> {
    let row_max = map.len() - 1;
    let col_max = map[0].len() - 1;
    let mut values: Vec<u8> = Vec::with_capacity(4);

    // Left
    if x != 0 {
        values.push(map[y][x-1])
    }
    // Right
    if x != col_max {
        values.push(map[y][x+1])
    }
    // Up
    if y != 0 {
        values.push(map[y-1][x])
    }
    // Down 
    if y != row_max {
        values.push(map[y+1][x])
    }

    values
}

fn main() {
    println!("{}", run(include_str!("../input.txt")));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run(include_str!("../test.txt")), 15);
    }
}