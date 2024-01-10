use std::fs;

trait Symbol {
    fn is_symbol(&self) -> bool;
}

impl Symbol for char {
    fn is_symbol(&self) -> bool {
        self.is_ascii() && !self.is_ascii_alphanumeric() && *self != '.'
    }
}

fn is_next_to_symbol(lines: &[&str], num_vec: &[(usize, usize)]) -> bool {
    let result = num_vec
        .iter()
        .map(|(x, y)| {
            let mut positions = vec![(*x, *y + 1), (*x + 1, *y), (*x + 1, *y + 1)];
            // (*x, *y - 1)
            if *y > 0 {
                positions.push((*x, *y - 1));
                positions.push((*x + 1, *y - 1));
            }

            // (*x - 1, *y)
            if *x > 0 {
                positions.push((*x - 1, *y));
                positions.push((*x - 1, *y + 1));
            }

            // (*x - 1, *y - 1)
            if *x > 0 && *y > 0 {
                positions.push((*x - 1, *y - 1));
            }
            positions
                .iter()
                .map(|(x, y)| {
                    let line = lines.get(*x).unwrap_or(&"").chars().nth(*y).unwrap_or('0');
                    line.is_symbol()
                })
                .any(|x| x)
        })
        .any(|x| x);
    result
}

fn main() {
    let grid_text = fs::read_to_string("day3/input.txt").unwrap();
    let lines: Vec<&str> = grid_text.split('\n').collect();

    let mut numbers: Vec<u32> = vec![];

    for (line_pos, line) in lines.iter().enumerate() {
        let mut num_positions = vec![];
        let mut num_str: String = String::new();

        for (i, c) in line.chars().enumerate() {
            if c.is_numeric() {
                num_positions.push((line_pos, i));
                num_str.push(c);

                // contniue if the line still has more characters
                if i < line.len() - 1 {
                    continue;
                }
            }

            if !num_str.is_empty() && is_next_to_symbol(&lines, num_positions.as_slice()) {
                let num = num_str.parse::<u32>().unwrap();
                numbers.push(num);
            }
            num_str.clear();
            num_positions.clear();
        }
    }
    // numbers.sort();
    // numbers.dedup();
    let sum: u32 = numbers.iter().sum();
    println!("The sum of the parts is {sum}.");
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_is_symbol() {
        assert!(!'a'.is_symbol());
        assert!('%'.is_symbol());
        assert!(!'6'.is_symbol());
        assert!('#'.is_symbol());
        assert!('='.is_symbol());
        assert!('@'.is_symbol());
        assert!('/'.is_symbol());
        assert!('&'.is_symbol());
        assert!('*'.is_symbol());
        assert!('-'.is_symbol());
        assert!('+'.is_symbol());
    }
}
