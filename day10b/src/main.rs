#[derive(Debug, PartialEq)]
enum ChunkOpen {
    PAREN,
    BRACKET,
    LESSTHAN,
    CURLYBRACKET,
}

fn value(chunk: &ChunkOpen) -> usize {
    return match chunk {
        ChunkOpen::PAREN => 1,
        ChunkOpen::BRACKET => 2,
        ChunkOpen::CURLYBRACKET => 3,
        ChunkOpen:: LESSTHAN => 4,
    }
}

fn score_line(line: &str) -> usize {
    let mut tokens = Vec::with_capacity(line.len());
    for token in line.chars() {
        match token {
            '(' => { tokens.push(ChunkOpen::PAREN)},
            ')' => {
                if tokens.pop().unwrap() != ChunkOpen::PAREN {
                    return 0
                }
            }
            '<' => { tokens.push(ChunkOpen::LESSTHAN)},
            '>' => {
                if tokens.pop().unwrap() != ChunkOpen::LESSTHAN {
                    return 0;
                }
            }
            '{' => { tokens.push(ChunkOpen::CURLYBRACKET)},
            '}' => {
                if tokens.pop().unwrap() != ChunkOpen::CURLYBRACKET {
                    return 0
                }
            }
            '[' => { tokens.push(ChunkOpen::BRACKET)},
            ']' => {
                if tokens.pop().unwrap() != ChunkOpen::BRACKET {
                    return 0;
                }
            },
            _ => panic!("Unknown Token")
        }
    }

    let mut score: usize = 0;

    for token in tokens.iter().rev() {
        score *= 5;
        score += value(token);
    }
    
    return score;

}

fn run(input: &'static str) -> usize {
    let mut scores: Vec<usize> = input
        .lines()
        .map(|x| score_line(x))
        .filter(|x| x != &0)
        .collect();
    
    scores.sort();

    scores[scores.len() / 2]
}

fn main() {
    println!("{}", run(include_str!("../input.txt")));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run(include_str!("../test.txt")), 288957);
    }
}