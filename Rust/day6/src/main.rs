// Day6
// Cosas que cambiaria:
// - No me sirvio de nada al final usar Range, porque cuando se lo pasas a una
//   funcion como parametro despues no lo podes usar como iterador
use std::ops::{Index, IndexMut, Range};

#[derive(Debug, Default)]
struct Grid<T> {
    size: (usize, usize),
    lights: Vec<T>,
}

impl<T: Default + Copy> Grid<T> {
    fn new((x, y): (usize, usize)) -> Self {
        Self {
            size: (x, y),
            lights: vec![T::default(); x * y],
        }
    }
}

impl Grid<bool> {
    fn turn_on(&mut self, range1: Range<usize>, range2: Range<usize>) {
        let x1 = range1.start;
        let y1 = range1.end;
        let x2 = range2.start;
        let y2 = range2.end;
        for i in x1..=x2 {
            for j in y1..=y2 {
                self[(i, j)] = true;
            }
        }
    }

    fn turn_off(&mut self, range1: Range<usize>, range2: Range<usize>) {
        let x1 = range1.start;
        let y1 = range1.end;
        let x2 = range2.start;
        let y2 = range2.end;
        for i in x1..=x2 {
            for j in y1..=y2 {
                self[(i, j)] = false;
            }
        }
    }

    fn toggle(&mut self, range1: Range<usize>, range2: Range<usize>) {
        let x1 = range1.start;
        let y1 = range1.end;
        let x2 = range2.start;
        let y2 = range2.end;
        for i in x1..=x2 {
            for j in y1..=y2 {
                self[(i, j)] = !self[(i, j)];
            }
        }
    }

    fn do_operation(&mut self, operation: Operations) {
        match operation {
            Operations::Toggle(range1, range2) => self.toggle(range1, range2),
            Operations::TurnOn(range1, range2) => self.turn_on(range1, range2),
            Operations::TurnOff(range1, range2) => self.turn_off(range1, range2),
        }
    }

    fn count_lights(&self) -> usize {
        self.lights.iter().filter(|&light| *light).count()
    }
}

impl Grid<usize> {
    fn turn_on(&mut self, range1: Range<usize>, range2: Range<usize>) {
        let x1 = range1.start;
        let y1 = range1.end;
        let x2 = range2.start;
        let y2 = range2.end;
        for i in x1..=x2 {
            for j in y1..=y2 {
                self[(i, j)] += 1;
                // self[(i, j)] = self[(i, j)].wrapping_add(1usize);
            }
        }
    }

    fn turn_off(&mut self, range1: Range<usize>, range2: Range<usize>) {
        let x1 = range1.start;
        let y1 = range1.end;
        let x2 = range2.start;
        let y2 = range2.end;
        for i in x1..=x2 {
            for j in y1..=y2 {
                self[(i, j)] -= 1;
                // self[(i, j)] = self[(i, j)].wrapping_sub(1usize);
            }
        }
    }

    fn toggle(&mut self, range1: Range<usize>, range2: Range<usize>) {
        let x1 = range1.start;
        let y1 = range1.end;
        let x2 = range2.start;
        let y2 = range2.end;
        for i in x1..=x2 {
            for j in y1..=y2 {
                self[(i, j)] += 2;
                // self[(i, j)] = self[(i, j)].wrapping_add(2usize);
            }
        }
    }

    fn do_operation(&mut self, operation: Operations) {
        match operation {
            Operations::Toggle(range1, range2) => self.toggle(range1, range2),
            Operations::TurnOn(range1, range2) => self.turn_on(range1, range2),
            Operations::TurnOff(range1, range2) => self.turn_off(range1, range2),
        }
    }

    fn count_lights(&self) -> usize {
        self.lights.iter().sum()
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        let (w, _) = self.size;
        &self.lights[y * w + x]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        let (w, _) = self.size;
        &mut self.lights[y * w + x]
    }
}

#[derive(Debug)]
enum Operations {
    TurnOn(Range<usize>, Range<usize>),
    TurnOff(Range<usize>, Range<usize>),
    Toggle(Range<usize>, Range<usize>),
}

use std::str::FromStr;

fn parse_pairs<T: FromStr>(s: &str, separator: char) -> Option<Range<T>> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some(l..r),
            _ => None,
        },
    }
}

fn parse_line(line: &str) -> Option<Operations> {
    let words: Vec<_> = line.split_whitespace().collect();
    if words[0] == "toggle" {
        let pair1 = parse_pairs(words[1], ',')?;
        let pair2 = parse_pairs(words[3], ',')?;
        Some(Operations::Toggle(pair1, pair2))
    } else {
        if words[1] == "off" {
            let pair1 = parse_pairs(words[2], ',')?;
            let pair2 = parse_pairs(words[4], ',')?;
            Some(Operations::TurnOff(pair1, pair2))
        } else {
            let pair1 = parse_pairs(words[2], ',')?;
            let pair2 = parse_pairs(words[4], ',')?;
            Some(Operations::TurnOn(pair1, pair2))
        }
    }
}

fn part1(input: &str) -> usize {
    let mut grid: Grid<bool> = Grid::new((1000, 1000));
    for line in input.lines() {
        if let Some(operation) = parse_line(line) {
            grid.do_operation(operation);
        }
    }
    grid.count_lights()
}

fn part2(input: &str) -> usize {
    let mut grid: Grid<usize> = Grid::new((1000, 1000));
    for line in input.lines() {
        if let Some(operation) = parse_line(line) {
            grid.do_operation(operation);
        }
    }
    grid.count_lights()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("the number of light on is: {:?}", part1(&input));
    println!(
        "the total brightness of all the lights is: {:?}",
        part2(&input)
    );
}
