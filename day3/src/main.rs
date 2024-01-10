#[cfg(feature = "part2")]
use std::collections::HashMap;
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

#[cfg(feature = "part2")]
fn get_closeby_gears(lines: &[&str], num_vec: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut output: Vec<(usize, usize)> = vec![];
    num_vec.iter().for_each(|(x, y)| {
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
        positions.iter().for_each(|(x, y)| {
            let symbol = lines.get(*x).unwrap_or(&"").chars().nth(*y).unwrap_or('0');
            if symbol == '*' {
                output.push((*x, *y));
            }
        });
    });
    output
}

fn main() {
    let grid_text = fs::read_to_string("day3/input.txt").unwrap();
    let lines: Vec<&str> = grid_text.split('\n').collect();

    let mut numbers: Vec<u32> = vec![];
    let sum: u32;

    #[cfg(not(feature = "part2"))]
    {
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
        sum = numbers.iter().sum();
    }

    #[cfg(feature = "part2")]
    {
        let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
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

                if !num_str.is_empty() {
                    let num = num_str.parse::<u32>().unwrap();
                    let gear_vec = get_closeby_gears(&lines, num_positions.as_slice());

                    gear_vec.iter().for_each(|pos| {
                        gears.entry(*pos).or_insert_with(Vec::new).push(num);
                    });
                }
                num_str.clear();
                num_positions.clear();
            }
        }

        sum = gears
            .iter()
            .filter_map(|(_key, value)| {
                if value.len() >= 2 {
                    let mut v = value.clone();
                    v.sort();
                    v.dedup();
                    let out = if v.len() > 1 {
                        println!("elems = {:#?}", v);
                        Some(v.iter().product::<u32>())
                    } else {
                        None
                    };
                    out
                } else {
                    None
                }
            })
            .sum();
    }

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
