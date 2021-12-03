#[derive(Clone, Copy, Debug)]
struct Diagnostic {
    value: u16, // 
    len: usize,
}

impl Diagnostic {
    pub fn from_bstr(bstr: &str) -> Diagnostic {
        let len = bstr.len();
        let mut value: u16 = 0;
        if  len > 16 {
            panic!("Diagnostic String Too Long.");
        }

        for (idx, bit) in bstr.chars().rev().enumerate() {
            if bit == '1' {
                value |= 2u16.pow(idx as u32);
            }
        }
        Diagnostic { value: value, len: len}
    }
}

// Integer Division with a ceiling
fn div_ceil(dividend: usize, divisor: usize) -> usize {
    (dividend / divisor) + if (dividend % divisor) > 0 {1} else {0}
}

fn oxygen(diags: &Vec<Diagnostic>) -> u16 {
    let mut canidates = diags.to_owned();
    
    for idx in (0..canidates[0].len).rev() {
        let count = canidates.iter()
            .filter(|x| (x.value & 2u16.pow(idx as u32)) > 0)
            .count(); 

        if count >= div_ceil(canidates.len(), 2) {
            canidates.retain(|x| (x.value & 2u16.pow(idx as u32)) > 0);
        } else {
            canidates.retain(|x| (x.value & 2u16.pow(idx as u32)) == 0);
        }

        if canidates.len() == 1 {
            return canidates[0].value;
        }
    }
    panic!("Oxygen Determination Failed")
}

fn co2(diags: &Vec<Diagnostic>) -> u16 {
    let mut canidates = diags.to_owned();
    
    for idx in (0..canidates[0].len).rev() {
        let count = canidates.iter()
            .filter(|x| (x.value & 2u16.pow(idx as u32)) > 0)
            .count(); 

        if count >= div_ceil(canidates.len(), 2) {
            canidates.retain(|x| (x.value & 2u16.pow(idx as u32)) == 0);
        } else {
            canidates.retain(|x| (x.value & 2u16.pow(idx as u32)) > 0);
        }

        if canidates.len() == 1 {
            return canidates[0].value;
        }
    }
    panic!("CO2 Determination Failed")
}

fn run(input: &'static str) -> usize {
    let diags: Vec<Diagnostic> = input.lines().map(|x| Diagnostic::from_bstr(x)).collect();
    
    return oxygen(&diags) as usize * co2(&diags) as usize;  
}

fn main() {
    println!("{}", run(include_str!("../input.txt")));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run(include_str!("../test.txt")), 230);
    }
}