use std::fs;
fn main() {
    // read the input
    let lines = fs::read_to_string("input.txt").expect("invalid text file");

    // Part 1
    let sum: usize = lines
        .split('\n')
        .map(|l| {
            if !l.is_empty() {
                let nums: Vec<char> = l.chars().filter(|c| c.is_numeric()).collect();
                let number = format!("{}{}", nums.first().unwrap(), nums.last().unwrap(),);
                number.parse::<usize>().unwrap()
            } else {
                0
            }
        })
        .sum();

    // Part2
    #[cfg(feature = "part2")]
    let sum: usize = lines
        .split('\n')
        .map(|l| {
            if !l.is_empty() {
                let num_lit = [
                    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
                ];

                let mut positions = vec![];

                num_lit.iter().enumerate().for_each(|(i, &n)| {
                    let mut start = 0;
                    while let Some(pos) = &l[start..].find(n) {
                        let abs_pos = start + pos;
                        positions.push((abs_pos, (i + 1) as u32));
                        start = abs_pos + n.len();
                    }
                });

                let nums: Vec<(usize, u32)> = l
                    .chars()
                    .enumerate()
                    .filter_map(|(i, c)| c.to_digit(10).map(|n| (i, n)))
                    .collect();

                positions.extend(nums);

                positions.sort_by(|a, b| a.0.cmp(&b.0));

                let first = positions.first().unwrap().1;
                let last = positions.last().unwrap().1;

                let number = format!("{}{}", first, last);
                number.parse::<usize>().unwrap()
            } else {
                0
            }
        })
        .sum();

    println!("The sum is = {}", sum);
}
