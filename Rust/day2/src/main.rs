/// day2
use std::str::FromStr;

#[derive(Debug)]
struct GiftBox {
    l: usize,
    w: usize,
    h: usize,
}

impl GiftBox {
    fn new(l: usize, w: usize, h: usize) -> Self {
        Self { l, w, h }
    }

    fn get_smallest_side(&self) -> usize {
        let side1 = self.l * self.w;
        let side2 = self.w * self.h;
        let side3 = self.h * self.l;
        let sides = [side1, side2, side3];
        *sides.iter().min().unwrap()
    }

    fn get_two_smallest_sides(&self) -> (usize, usize) {
        let mut sides = [self.l, self.w, self.h];
        sides.sort();
        (sides[0], sides[1])
    }

    fn calculate_surface_area(&self) -> usize {
        2 * self.l * self.w + 2 * self.w * self.h + 2 * self.h * self.l + self.get_smallest_side()
    }

    fn calculate_volumen(&self) -> usize {
        self.l * self.w * self.h
    }

    fn calculate_ribbon_lenght(&self) -> usize {
        let (side1, side2) = self.get_two_smallest_sides();
        2 * side1 + 2 * side2 + self.calculate_volumen()
    }
}

fn parse_line(line: &str) -> GiftBox {
    let numbers: Vec<_> = line
        .split('x')
        .filter_map(|number| usize::from_str(number).ok())
        .collect();
    GiftBox::new(numbers[0], numbers[1], numbers[2])
}

fn part1(boxes: &[GiftBox]) -> usize {
    boxes.iter().map(|b| b.calculate_surface_area()).sum()
}

fn part2(boxes: &[GiftBox]) -> usize {
    boxes.iter().map(|b| b.calculate_ribbon_lenght()).sum()
}

fn generate_boxes(input: &str) -> Vec<GiftBox> {
    let mut boxes: Vec<GiftBox> = Vec::new();
    for line in input.split_whitespace() {
        boxes.push(parse_line(line));
    }
    boxes
}

fn main() {
    let input = include_str!("../input.txt");
    let boxes = generate_boxes(&input);
    let part1_response = part1(&boxes);
    println!("part1_response: {}", part1_response);
    let part2_response = part2(&boxes);
    println!("part2_response: {}", part2_response);
}
