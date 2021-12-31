use itertools::Itertools;
use std::ops::Add;
use std::fmt;

#[derive(Copy, Clone, Debug)]
enum NumberPart {
    LParen,
    RParen,
    Comma,
    Value(usize),

}

struct Number {
    components: Vec<NumberPart>
}

impl fmt::Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut repr = String::new();
        for item in self.components.iter() {
            match item {
                NumberPart::LParen => repr.push_str("["),
                NumberPart::RParen => repr.push_str("]"),
                NumberPart::Comma => repr.push_str(","),
                NumberPart::Value(x) => repr.push_str(&*x.to_string())
            }
        }
        write!(f, "{}", repr)
    }
}

impl Number {
    pub fn from_str(input: &str) -> Number {
        let mut temp: Vec<NumberPart> = vec!();
        let mut components: Vec<NumberPart> = vec!();
        let mut component: NumberPart;
    
        for chr in input.chars(){
            component = match chr {
                '['       => NumberPart::LParen,
                ']'       => NumberPart::RParen,
                ','       => NumberPart::Comma,
                '0'..='9' => NumberPart::Value(chr.to_digit(10).unwrap() as _),
                _         => panic!("Unknown char"),
            };
            temp.push(component);
        }

        // Fixup multi-char nums
        for item in temp.iter() {
            if let NumberPart::Value(current) = item {
                if !components.is_empty() {
                    if matches!(components.last().unwrap(), NumberPart::Value(_)) {
                        if let NumberPart::Value(old) = components.pop().unwrap() {
                            components.push(NumberPart::Value((old * 10) + current));
                        }
                        continue;
                    }
                }
            }
            components.push(*item);
        }
        Number { components: components}
    }

    pub fn explode(&mut self, idx: usize) {
        let mut lvalue: usize = 0;
        let mut rvalue: usize = 0;
        if let NumberPart::Value(x) = self.components.get(idx).unwrap() {
            lvalue = *x;
        }
        if let NumberPart::Value(x) = self.components.get(idx+2).unwrap() {
            rvalue = *x;
        }

        //fix left
        for lidx in (0..idx-2).rev() {
            let lval: usize;
            if let NumberPart::Value(y) = self.components.get(lidx).unwrap() {
                lval = *y;
            } else {
                continue;
            }
            self.components.remove(lidx);
            self.components.insert(lidx, NumberPart::Value(lvalue + lval));
            break;
        }

        //fix right
        for ridx in idx+3..self.components.len() {
            let rval: usize;
            if let NumberPart::Value(y) = self.components.get(ridx).unwrap() {
                rval = *y;
            } else {
                continue;
            }
            self.components.remove(ridx);
            self.components.insert(ridx, NumberPart::Value(rvalue + rval));
            break;
        }

        //delete node
        for _ in 0..5 {
            self.components.remove(idx - 1);
        }
        self.components.insert(idx - 1, NumberPart::Value(0));
    }

    pub fn reduce(&mut self) {
        loop {
            if !self._reduce() {
                break;
            }
        }
    }

    fn _reduce(&mut self) -> bool {

        //Explode
        let mut depth: usize = 0;
        for idx in 0..self.components.len() {
            let item = self.components.get(idx).unwrap();
            match item {
                NumberPart::LParen => depth += 1,
                NumberPart::RParen => depth -= 1,
                NumberPart::Comma => {},
                NumberPart::Value(_) => {
                    if depth == 5 {
                        self.explode(idx);
                        return true;
                    }
                    
                }
            }
        }

        
        // Split
        for idx in 0..self.components.len() {
            let item = self.components.get(idx).unwrap();
            if let NumberPart::Value(x) = item {
                if x >= &10 {
                    let mut other: Vec<NumberPart> = Vec::from([
                        NumberPart::LParen,
                        NumberPart::Value((*x as f32 / 2f32).floor() as _),
                        NumberPart::Comma,
                        NumberPart::Value((*x as f32 / 2f32).ceil() as _),
                        NumberPart::RParen,
                    ]);
                    merge_vec(&mut self.components, &mut other, idx, true);
                    return true;
                }
            }
        }
        return false;
    }

    fn depth(components: &Vec<NumberPart>) -> usize {
        let mut max_depth: usize = 0;
        let mut depth: usize = 0;
        for idx in 0..components.len() {
            let item = components.get(idx).unwrap();
            match item {
                NumberPart::LParen => {
                    depth += 1;
                    if depth > max_depth {
                        max_depth = depth;
                    }
                },
                NumberPart::RParen => depth -= 1,
                _ => {},
            }
        }
        return max_depth;
    }

    pub fn magnitude(self) -> usize {
        let mut components = self.components.clone();
        loop {      
            let max_depth = Number::depth(&components);
            if let NumberPart::Value(x) = components.get(0).unwrap() {
                return *x;
            }

            let mut depth: usize = 0;
            let mut idx: usize = 0;
            while idx < components.len() {
                let item = components.get(idx).unwrap_or(&NumberPart::Comma);
                match item {
                    NumberPart::LParen => {depth += 1;},
                    NumberPart::RParen => {depth -= 1;},
                    NumberPart::Comma => {},
                    NumberPart::Value(_) => { 
                        if depth == max_depth {
                            Number::_magnitude(&mut components, idx);
                            depth -= 1;
                            idx -= 1;
                        }
                    }
                }
                idx += 1;
            }
        }
    }

    fn _magnitude(components: &mut Vec<NumberPart>, start_idx: usize) {
        let left: usize;
        let right: usize;

        if let NumberPart::Value(x) = components.get(start_idx).unwrap() {
            left = *x;
        } else {
            panic!("Magnitude of Pair has no left value");
        }

        if let NumberPart::Value(x) = components.get(start_idx+2).unwrap() {
            right = *x;
        } else {
            panic!("Magnitude of Pair has no right value");
        }

        let new_value: usize = (3 * left) + (2 * right);

        for _ in 0..5 {
            components.remove(start_idx - 1);
        }
        components.insert(start_idx - 1, NumberPart::Value(new_value));
    }
}

fn merge_vec<T>(recv: &mut Vec<T>, other: &mut Vec<T>, index: usize, overwrite: bool) {
    if overwrite {
        recv.remove(index);
    }
    while !other.is_empty() {
        recv.insert(index, other.pop().unwrap());
    }
}

impl Add for Number {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut new = self.components.clone();
        new.insert(0, NumberPart::LParen);
        new.push(NumberPart::Comma);
        new.append(&mut other.components.clone());
        new.push(NumberPart::RParen);
        Self {components: new}
    }
}

fn run(input: &'static str) ->  usize {
    return input
        .lines()
        .permutations(2)
        .map(|v| {
            let mut n1 = Number::from_str(v[0]) + Number::from_str(v[1]);
            let mut n2 = Number::from_str(v[1]) + Number::from_str(v[0]);
            
            n1.reduce();
            n2.reduce();

            let m1 = n1.magnitude();
            let m2 = n2.magnitude();
            if m1 > m2 {
                return m1;
            }
            return m2;
            0
        })
        .max()
        .unwrap();
}

fn main() {
    println!("{}", run(include_str!("../input.txt")));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complete() {
        assert_eq!(run(include_str!("../test.txt")), 3993);
    }
}
