use std::collections::HashSet;
use std::error::Error;
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Copy, Clone)]
enum Directions {
    North,
    South,
    East,
    West,
}

impl Directions {
    fn get_direction(direction: char) -> Self {
        match direction {
            '^' => Self::North,
            'v' => Self::South,
            '<' => Self::West,
            '>' => Self::East,
            _ => panic!("this direction: {} is wrong!!!", direction),
        }
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, Hash, PartialEq)]
struct Position(i32, i32);

impl Add for Position {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Position {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl AddAssign for Position {
    #[inline(always)]
    fn add_assign(&mut self, other: Self) {
        *self = *self + other
    }
}

impl SubAssign for Position {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other
    }
}

#[derive(Debug, Default)]
struct Map {
    actual_position: Position,
    history: HashSet<Position>,
}

impl Map {
    fn new() -> Self {
        let actual_position = Position(0, 0);
        let mut history: HashSet<Position> = HashSet::new();
        history.insert(actual_position);
        Self {
            actual_position,
            history,
        }
    }

    fn update(&mut self, direction: Directions) {
        match direction {
            Directions::North => {
                self.actual_position += Position(0, 1);
                self.history.insert(self.actual_position);
            }
            Directions::South => {
                self.actual_position -= Position(0, 1);
                self.history.insert(self.actual_position);
            }
            Directions::East => {
                self.actual_position += Position(1, 0);
                self.history.insert(self.actual_position);
            }
            Directions::West => {
                self.actual_position -= Position(1, 0);
                self.history.insert(self.actual_position);
            }
        }
    }
}

fn part1(directions: &[Directions]) -> usize {
    let mut map = Map::new();

    for direction in directions {
        map.update(*direction);
    }
    map.history.len()
}

fn part2(directions: &[Directions]) -> usize {
    let mut map_santa = Map::default();
    let mut map_robot = Map::default();
    let directions_santa: Vec<_> = directions.iter().step_by(2).collect();
    let directions_robot: Vec<_> = directions.iter().skip(1).step_by(2).collect();
    for direction in directions_santa {
        map_santa.update(*direction);
    }
    for direction in directions_robot {
        map_robot.update(*direction);
    }

    let result: HashSet<_> = map_santa.history.union(&map_robot.history).collect();
    result.len()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input.txt");
    let directions: Vec<_> = input
        .chars()
        .filter(|&c| c != '\n')
        .map(|c| Directions::get_direction(c))
        .collect();

    let result1 = part1(&directions);

    println!(
        "the number of houses that receive at least one present is: {}",
        result1
    );

    let result2 = part2(&directions);

    println!(
        "the number of houses that receive at least one present is: {}",
        result2
    );

    Ok(())
}
