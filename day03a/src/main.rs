#[derive(Debug)]
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

fn gamma(diags: &Vec<Diagnostic>) -> u16 {
    let mut value: u16 = 0;
    
    for idx in 0..diags[0].len {
        let count = diags.iter()
            .filter(|x| (x.value & 2u16.pow(idx as u32) > 0))
            .count();

        if count > (diags.len() / 2) {
            value |= 2u16.pow(idx as u32);
        }
    }

    value
}

fn epsilon(diags: &Vec<Diagnostic>) -> u16 {
    let mut value: u16 = 0;
    
    for idx in 0..diags[0].len {
        let count = diags.iter()
            .filter(|x| (x.value & 2u16.pow(idx as u32) > 0))
            .count();

        if count < (diags.len() / 2) {
            value |= 2u16.pow(idx as u32);
        }
    }

    value
}


fn run(input: &'static str) -> usize {
    let diags: Vec<Diagnostic> = input.lines().map(|x| Diagnostic::from_bstr(x)).collect();
    
    return gamma(&diags) as usize * epsilon(&diags) as usize;
    
}

fn main() {
    println!("{}", run(include_str!("../input.txt")));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run(include_str!("../test.txt")), 198);
    }
}