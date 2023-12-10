use std::fs;

use regex::Regex;

#[derive(Debug, Default)]
struct Set {
    red: Option<usize>,
    green: Option<usize>,
    blue: Option<usize>,
}

#[derive(Debug)]
struct Game {
    id: usize,
    sets: Vec<Set>,
}

impl Game {
    fn get_max_cubes(&self) -> (Option<usize>, Option<usize>, Option<usize>) {
        (
            self.sets.iter().filter_map(|set| set.red).max(),
            self.sets.iter().filter_map(|set| set.green).max(),
            self.sets.iter().filter_map(|set| set.blue).max(),
        )
    }

    #[cfg(feature = "part2")]
    fn get_cubes_power(&self) -> usize {
        let (red, green, blue) = self.get_max_cubes();
        red.unwrap_or(1) * green.unwrap_or(1) * blue.unwrap_or(1)
    }
}
impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let re = Regex::new(r"Game (?P<game_id>\d+):(?P<rest>.*)").unwrap();
        let caps = re.captures(value).unwrap();
        let game_id = caps["game_id"].parse::<usize>().unwrap();
        let rest: &str = &caps["rest"];

        let sets_str: Vec<&str> = rest.split(';').collect();
        let mut sets: Vec<Set> = vec![];
        // parse the sets
        let re_sets = Regex::new(r"(\d+)\s(\w+),?").unwrap();

        for set in sets_str {
            let mut set_obj: Set = Set::default();
            for (_, [num, color]) in re_sets.captures_iter(set).map(|c| c.extract()) {
                let n: usize = num.parse().unwrap();
                match color {
                    "blue" => set_obj.blue = Some(n),
                    "red" => set_obj.red = Some(n),
                    "green" => set_obj.green = Some(n),
                    _ => {}
                }
            }
            sets.push(set_obj);
        }

        Self { id: game_id, sets }
    }
}

fn main() {
    let games: Vec<Game> = fs::read_to_string("day2/input.txt")
        .unwrap()
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.into())
        .collect();

    // target cubes
    const REDS: usize = 12;
    const GREENS: usize = 13;
    const BLUES: usize = 14;

    let sum: usize = games
        .iter()
        .filter(|&game| {
            let (r, g, b) = game.get_max_cubes();
            r.unwrap_or(0) <= REDS && g.unwrap_or(0) <= GREENS && b.unwrap_or(0) <= BLUES
        })
        .map(|g| g.id)
        .sum();

    #[cfg(feature = "part2")]
    let sum: usize = games.iter().map(|game| game.get_cubes_power()).sum();

    println!("The sum is = {sum}");
}
