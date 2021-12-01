
fn main() {
    let sonar_depths: Vec<usize> = include_str!("../input.txt")
        .lines()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let increases = sonar_depths.windows(2)
        .filter(|x| x[1] > x[0])
        .count();
    
    println!("# Depth Increases: {}", increases);
}
