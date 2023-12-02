use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String, AocError> {
    let output: Vec<Game> = input
        .lines().map(|line| {
        return Game::new(line)
    }).collect();
    let x:u32 = output.iter().filter(|game| game.is_ok()).map(|game| game.id).sum();
    return Ok(x.to_string());
}

struct Game {
   pub id: u32,
    cube_sets: Vec<CubeSet>,
}

impl Game {
    pub fn new(line: &str) -> Game {
        let colon_split: Vec<&str> = line.split(':').collect();
        let full_line_values = colon_split.last().unwrap();
        let cube_sets: Vec<&str> = full_line_values.split(';').collect();
        let color_value_vec:Vec<CubeSet> = cube_sets.iter()
            .map(|value| {
                let values: Vec<&str> = value.split(',').collect();
                let cos:Vec<PairColorValue> = values.iter().map(|cv| {
                    let color_value_t: Vec<&str> = cv.trim().split(' ').collect();
                    let color: &str = color_value_t.last().unwrap();
                    let value: &str = color_value_t.first().unwrap();
                    return PairColorValue { color: color.to_string(), value: value.parse::<u32>().unwrap() };
                }).collect();
                return CubeSet{ color_value:cos};
            }).collect();
        let idf:Vec<&str>  = colon_split.first().unwrap().split(' ').collect();
        Game { id: idf.last().unwrap().parse::<u32>().unwrap(), cube_sets: color_value_vec }
    }
    pub fn is_ok(&self) -> bool {
        // let _z = self.cubeSets.iter().filter(|cubeSet| cubeSet.isOk()).collect();
        // self.cube_sets.iter().filter()
        return self.cube_sets.iter().all(|cs| cs.is_ok());

    }
}

struct PairColorValue {
    color: String,
    value: u32,
}

impl PairColorValue{
    pub fn is_ok(&self) -> bool{
        return if (self.color == "blue" && self.value < 15)
            || (self.color == "red" && self.value < 13)
            || (self.color == "green" && self.value < 14) {
            true
        } else {
            false
        }
    }
}

struct CubeSet {
    color_value: Vec<PairColorValue>
}


impl CubeSet {
    pub fn is_ok(&self) -> bool {
        self.color_value.iter().all(|cv| cv.is_ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("8", process(input)?);
        Ok(())
    }
}
