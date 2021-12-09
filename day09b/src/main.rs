use std::cmp::Ordering;

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
    let points: Vec<Point> = map
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row
                .iter()
                .enumerate()
                .map(|(x, _)| {
                    Point {x: x, y: y, height: map[y][x]}
                })
                .collect::<Vec<Point>>()
        })
        .flat_map(|x| x)
        .collect();

    let low_points: Vec<Point> = points
        .iter()
        .filter(|x| {
            x < &neighbors(&map, x.x, x.y).iter().min().unwrap()
        })
        .map(|x| *x)
        .collect();

    let mut basins = low_points
        .iter()
        .map(|x| basin(&map, x))
        .collect::<Vec<usize>>();
    
    basins.sort();

    return basins
        .iter()
        .rev()
        .take(3)
        .product();

}

#[derive(Copy, Clone, Debug, Eq)]
struct Point {
    x: usize,
    y: usize,
    height: u8,
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.height.cmp(&other.height)
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.height == other.height
    }
}

fn neighbors(map: &Vec<Vec<u8>>, x: usize, y: usize) -> Vec<Point> {
    let row_max = map.len() - 1;
    let col_max = map[0].len() - 1;
    let mut values: Vec<Point> = Vec::with_capacity(4);

    // Left
    if x != 0 {
        values.push(Point{x: x-1, y: y, height: map[y][x-1]})
    }
    // Right
    if x != col_max {
        values.push(Point{x: x+1, y: y, height: map[y][x+1]})
    }
    // Up
    if y != 0 {
        values.push(Point{x: x, y: y-1, height: map[y-1][x]})
    }
    // Down 
    if y != row_max {
        values.push(Point{x: x, y: y+1, height: map[y+1][x]})
    }

    values
}

fn basin(map: &Vec<Vec<u8>>, low_point: &Point) -> usize {
    let mut visited: Vec<(usize, usize)> = vec!();
    _walk_basin(map, low_point, &mut visited);
    return visited.len();
}

fn _walk_basin(map: &Vec<Vec<u8>>, point: &Point, visited: &mut Vec<(usize, usize)>)  {
    if visited.contains(&(point.x, point.y)) {
        return;
    }
    visited.push((point.x, point.y));

    let members: Vec<Point> = neighbors(map, point.x, point.y)
        .iter()
        .filter(|x| x.height < 9)
        .filter(|x| !visited.contains(&(x.x, x.y)))
        .map(|x| *x)
        .collect();
    
    for member in members {
        _walk_basin(map, &member, visited);
    }
}

fn main() {
    println!("{}", run(include_str!("../input.txt")));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run(include_str!("../test.txt")), 1134);
    }
}