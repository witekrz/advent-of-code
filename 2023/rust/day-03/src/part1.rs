use std::collections::BTreeMap;

use itertools::Itertools;

use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<String, AocError> {
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(
                move |(x, character)| {
                    (
                        (y as i32, x as i32),
                        match character {
                            '.' => Value::Empty,
                            c if c.is_ascii_digit() => {
                                Value::Number(
                                    c.to_digit(10).expect(
                                        "should be nubcer",
                                    ),
                                )
                            }
                            c => Value::Symbol(c),
                        },
                    )
                }, )
        }).collect::<BTreeMap<(i32, i32), Value>>();

    let mut numbers: Vec<Vec<((i32, i32), u32)>> = vec![];
    for ((y, x), value) in map.iter() {
        if let Value::Number(num) = value {
            match numbers.iter().last() {
                Some(v) => {
                    let last_num = v.iter().last();
                    match last_num {
                        Some(((last_num_x, _), _)) => {
                            if last_num_x + 1 == *x {
                                let last = numbers
                                    .iter_mut()
                                    .last()
                                    .expect("should exist");
                                last.push(((*x, *y), *num));
                            } else {
                                numbers.push(vec![(
                                    (*x, *y),
                                    *num,
                                )]);
                            }
                        }
                        None => unimplemented!(
                            "shouldn't happen"
                        ),
                    }
                }
                None => {
                    numbers.push(vec![((*x, *y), *num)]);
                }
            }
        }
    }
    // dbg!(&numbers);
    let mut total =0;
    for num_list in numbers {
        // (x,y)
        let positions = [
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1)
        ];

        let num_position: Vec<(i32, i32)> = num_list
            .iter()
            .map(|((y, x), _)| (*x as i32, *y as i32))
            .collect();
        let pos_to_check: Vec<(i32, i32)> = num_list
            .iter()
            .flat_map(|(pos, _)| {
                positions.iter().map(|outer_pos| {
                    (
                        outer_pos.0 + pos.1 as i32,
                        outer_pos.1 + pos.0 as i32
                    )
                })
            })
            .unique()
            .filter(|num| !num_position.contains(num))
            .collect();
        let is_part_number = pos_to_check.iter().any(|pos| {
            let value = map.get(&(pos.0, pos.1));
            if let Some(Value::Symbol(_)) = value {
                true
            } else {
                false
            }
        });

        if is_part_number{
            total += num_list
                .iter()
                .map(|(_, num)| num.to_string())
                .collect::<String>()
                .parse::<u32>()
                .unwrap();
        }
        println!("{total} - {is_part_number}")
    }

    return Ok(total.to_string());
}

#[derive(Debug)]
enum Value {
    Symbol(char),
    Empty,
    Number(u32),
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!("4361", process(input)?);
        Ok(())
    }
}
