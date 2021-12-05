 #[derive(Debug)]
struct BingoBoard {
    board: [[u16; 5]; 5],
    marked: Vec<(usize, usize)>,
}

impl BingoBoard {
    pub fn from_chunk(chunk: &[&str]) -> BingoBoard {
        let mut board = [[0; 5]; 5];
        
        for (x, row) in chunk.iter().skip(1).enumerate() {
            for (y, num) in row.split_whitespace().enumerate() {
                board[x][y] = num.parse::<u16>().unwrap();
            }
        }
        BingoBoard {board: board, marked: vec!()}
    }

    pub fn mark(&mut self, num: u16) {
        for (x, row) in self.board.iter().enumerate() {
            for (y, item) in row.iter().enumerate() {
                if *item == num {
                    self.marked.push((x, y));
                }
            }
        }
    }

    pub fn check(&self) -> bool {
        for z in 0..5 {
            if self.marked.iter().filter(|(x, _)| x == &z).count() == 5 {
                return true;
            }
            if self.marked.iter().filter(|(_, y)| y == &z).count() == 5 {
                return true;
            }
        }
        return false;
    }

    pub fn score(&self, last: &u16) -> usize {
        let mut sum: usize = 0;
        for (x, row) in self.board.iter().enumerate() {
            for (y, item) in row.iter().enumerate() {
                if !self.marked.contains(&(x, y)) {
                    sum += *item as usize;
                }
            }
        } 
        return sum * (*last as usize);
    }
}

fn run(input: &'static str) ->  usize {
    let mut lines: Vec<&str> = input.lines().collect();

    let numbers: Vec<u16> = lines.iter()
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<u16>().unwrap())
        .collect();

    lines.remove(0);

    let mut boards: Vec<BingoBoard> = lines.chunks(6)
        .map(|x| BingoBoard::from_chunk(x))
        .collect();

    for num in numbers {
        for board in boards.iter_mut() {
            board.mark(num);
            if board.check() {
                return board.score(&num);
            }
        }
    }
    panic!("Nobody Won :(");
}

fn main() {
    println!("{}", run(include_str!("../input.txt")));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run(include_str!("../test.txt")), 4512);
    }
}