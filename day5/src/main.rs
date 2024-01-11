use std::fs;

use regex::Regex;

#[derive(Debug)]
struct Map {
    source: usize,
    destination: usize,
    size: usize,
}

trait Convert {
    fn get_destination(&self, src: usize) -> usize;
}

impl Convert for Vec<Map> {
    fn get_destination(&self, src: usize) -> usize {
        self.iter()
            .find_map(|m| {
                if (m.destination..m.destination + m.size + 1).contains(&src) {
                    Some(src - m.destination + m.source)
                } else {
                    None
                }
            })
            .unwrap_or(src)
    }
}

#[derive(Default, Debug)]
struct Container {
    maps: Vec<Map>,
}

impl Convert for Container {
    fn get_destination(&self, src: usize) -> usize {
        self.maps.get_destination(src)
    }
}

impl From<&str> for Container {
    fn from(value: &str) -> Self {
        let maps: Vec<Map> = value
            .trim()
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| {
                let list: Vec<usize> = line
                    .split(' ')
                    .filter_map(|num| num.parse::<usize>().ok())
                    .collect();
                Map {
                    source: list[0],
                    destination: list[1],
                    size: list[2],
                }
            })
            .collect();

        Self { maps }
    }
}

fn main() {
    // parse the input
    let input = fs::read_to_string("day5/input.txt").unwrap();

    let re = Regex::new(r"seeds: (?P<seeds>[\d\s]+)\nseed-to-soil map:\n(?P<seed_to_soil>(?:[\d\s]+\n?)+)\nsoil-to-fertilizer map:\n(?P<soil_to_fert>(?:[\d\s]+\n?)+)\nfertilizer-to-water map:\n(?P<fert_to_water>(?:[\d\s]+\n?)+)\nwater-to-light map:\n(?P<water_to_light>(?:[\d\s]+\n?)+)\nlight-to-temperature map:\n(?P<light_to_temp>(?:[\d\s]+\n?)+)\ntemperature-to-humidity map:\n(?P<temp_to_humid>(?:[\d\s]+\n?)+)\nhumidity-to-location map:\n(?P<humid_to_location>(?:[\d\s]+\n?)+)").unwrap();
    let caps = re.captures(&input).unwrap();

    let seeds: Vec<usize> = caps["seeds"]
        .trim()
        .split(' ')
        .filter_map(|num| num.parse().ok())
        .collect();

    let locations: Container = caps["humid_to_location"].into();
    let humidity: Container = caps["temp_to_humid"].into();
    let temperature: Container = caps["light_to_temp"].into();
    let light: Container = caps["water_to_light"].into();
    let water: Container = caps["fert_to_water"].into();
    let fertilizer: Container = caps["soil_to_fert"].into();
    let soil: Container = caps["seed_to_soil"].into();

    let location = seeds
        .iter()
        .map(|&seed| {
            locations.get_destination(humidity.get_destination(temperature.get_destination(
                light.get_destination(
                    water.get_destination(fertilizer.get_destination(soil.get_destination(seed))),
                ),
            )))
        })
        .min()
        .unwrap();

    println!("The right location is {location}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_destination() {
        let maps = vec![
            Map {
                source: 50,
                destination: 98,
                size: 2,
            },
            Map {
                source: 52,
                destination: 50,
                size: 48,
            },
        ];

        assert!(maps.get_destination(79) == 81);
        assert!(maps.get_destination(5) == 5);
        assert!(maps.get_destination(49) == 49);
        assert!(maps.get_destination(99) == 51);
    }
}
