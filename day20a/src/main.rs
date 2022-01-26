struct Image {
    algorithm: Vec<bool>,
    pixels: Vec<Vec<bool>>,
    void: bool,
    height: usize,
    width: usize,
}

impl Image {
    fn read(input: &str) -> Self {
        let mut pixels: Vec<Vec<bool>> = vec!();
        for line in input.lines().skip(2) {
            pixels.push(line.chars().map(|chr| chr == '#').collect());
        }
            
        let algorithm: Vec<bool> = input.lines().next().unwrap()
            .chars().map(|x| if x == '#' {true} else {false})
            .collect();

        let width = pixels.iter().next().unwrap().len();
        let height = pixels.len();

        Image {
            pixels: pixels,
            algorithm: algorithm,
            void: false,
            height: height,
            width: width,
        }
    }

    #[allow(dead_code)]
    fn render(&self) {
        for row in self.pixels.iter() {
            println!();
            for pixel in row.iter() {
                if *pixel {
                    print!("#");
                } else {
                    print!(".");
                }
            }
        }
        println!();
    }


    fn pixel_index(&self, y: usize, x: usize) -> usize {
        let mut bits: [u8; 9] = [self.void as _; 9];

        // top row
        if y >= 2 {
            let prev_row = self.pixels.get(y - 2).unwrap();
            if x >= 2 {
                bits[0] = *prev_row.get(x - 2).unwrap() as _;
            }
            if x >= 1 && x <= (self.width - 2) {
                bits[1] = *prev_row.get(x - 1).unwrap() as _;
            }
            if x <= (self.width - 3) {
                bits[2] = *prev_row.get(x).unwrap() as _;
            }
        }

        // middle row
        if y >= 1 && y <= (self.height - 2) {
            let cur_row = self.pixels.get(y -1).unwrap();
            if x >= 2 {
                bits[3] = *cur_row.get(x - 2).unwrap() as _;
            }
            if x >= 1 && x <= (self.width - 2) {
                bits[4] = *cur_row.get(x - 1).unwrap() as _;
            }
            if x <= (self.width - 3) {
                bits[5] = *cur_row.get(x).unwrap() as _;
            } 
        }

        // bottom row
        if y <= (self.height - 3) {
            let next_row = self.pixels.get(y).unwrap();
            if x >= 2 {
                bits[6] = *next_row.get(x - 2).unwrap() as _;
            }
            if x >= 1 && x <= (self.width - 2) {
                bits[7] = *next_row.get(x - 1).unwrap() as _;
            }
            if x <= (self.width - 3) {
                bits[8] = *next_row.get(x).unwrap() as _;
            } 
        }

        return bits.iter().rev().enumerate()
            .filter(|(_, bit)| bit > &&0)
            .map(|(idx, _)| 2usize.pow(idx as _) as usize)
            .sum();     
    }
    
    fn enhance(&mut self) {
        self.height += 2;
        self.width += 2;

        let mut new_pixels: Vec<Vec<bool>> = Vec::with_capacity(self.height);
        
        for y in 0..self.height {
            let mut new_row: Vec<bool> = Vec::with_capacity(self.width);
            for x in 0..self.width {
                new_row.push(self.algorithm[self.pixel_index(y, x)])
            }
            new_pixels.push(new_row);
        }
    
        self.pixels = new_pixels;
        if self.algorithm[0] {
            self.void = !self.void;
        }
    }

    fn pixel_count(&self) -> usize {
        return self.pixels.iter()
            .map(|row|{
                row.iter().filter(|&x|*x).count()
            })
            .sum();
    }

}

fn run(input: &'static str) ->  usize {
    let mut image = Image::read(input);
    
    //image.render();
    
    for _ in 0..2 {
        image.enhance();
        //image.render();
    }

    return image.pixel_count();

}

fn main() {
    println!("{}", run(include_str!("../input.txt")));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(run(include_str!("../test.txt")), 35);
    }
}