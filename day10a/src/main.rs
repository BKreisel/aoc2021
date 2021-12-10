#[derive(Debug, PartialEq)]
enum ChunkOpen {
    PAREN,
    BRACKET,
    LESSTHAN,
    CURLYBRACKET,
}

fn value(chunk: ChunkOpen) -> usize {
    return match chunk {
        ChunkOpen::PAREN => 3,
        ChunkOpen::BRACKET => 57,
        ChunkOpen::CURLYBRACKET => 1197,
        ChunkOpen:: LESSTHAN => 25137,
    }
}

fn score(line: &str) -> usize {
    let mut tokens = Vec::with_capacity(line.len());
    for token in line.chars() {
        match token {
            '(' => { tokens.push(ChunkOpen::PAREN)},
            ')' => {
                if tokens.pop().unwrap() != ChunkOpen::PAREN {
                    return value(ChunkOpen::PAREN)
                }
            }
            '<' => { tokens.push(ChunkOpen::LESSTHAN)},
            '>' => {
                if tokens.pop().unwrap() != ChunkOpen::LESSTHAN {
                    return value(ChunkOpen::LESSTHAN)
                }
            }
            '{' => { tokens.push(ChunkOpen::CURLYBRACKET)},
            '}' => {
                if tokens.pop().unwrap() != ChunkOpen::CURLYBRACKET {
                    return value(ChunkOpen::CURLYBRACKET)
                }
            }
            '[' => { tokens.push(ChunkOpen::BRACKET)},
            ']' => {
                if tokens.pop().unwrap() != ChunkOpen::BRACKET {
                    return value(ChunkOpen::BRACKET)
                }
            },
            _ => panic!("Unknown Token")
        }
    }
    0
}

fn run(input: &'static str) -> usize {
    input
        .lines()
        .map(|x| score(x))
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
        assert_eq!(run(include_str!("../test.txt")), 26397);
    }
}