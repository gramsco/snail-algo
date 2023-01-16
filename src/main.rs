use std::{usize, vec};

fn snail(matrix: &[Vec<i32>]) -> Vec<i32> {
    if matrix[0].len() == 0 {
        return vec![];
    }
    let matrix = Vec::from(matrix);
    let mut s = Snail {
        coords: (0, 0),
        matrix: matrix.clone(),
        result: vec![],
        trace: vec![],
    };
    s.push(0, 0, matrix[0][0]);

    while s.result.len() < matrix.len().pow(2) {
        while let Some((y, x, v)) = s.go(Direction::Right) {
            s.push(y, x, v);
        }

        while let Some((y, x, v)) = s.go(Direction::Down) {
            s.push(y, x, v);
        }

        while let Some((y, x, v)) = s.go(Direction::Left) {
            s.push(y, x, v);
        }

        while let Some((y, x, v)) = s.go(Direction::Top) {
            s.push(y, x, v);
        }
    }
    return s.result;
}

struct Snail {
    matrix: Vec<Vec<i32>>,
    trace: Vec<(usize, usize)>,
    result: Vec<i32>,
    coords: (usize, usize),
}

enum Direction {
    Top,
    Down,
    Right,
    Left,
}

impl Snail {
    fn push(&mut self, y: usize, x: usize, value: i32) {
        let has_it = self.trace.iter().find(|(y1, x1)| y == *y1 && x == *x1);
        match has_it {
            Some(_) => (),
            _ => {
                self.coords = (y, x);
                self.trace.push((y, x));
                self.result.push(value);
            }
        };
    }

    fn read(&self, y: usize, x: usize) -> Option<(usize, usize, i32)> {
        let has_it = self.trace.iter().find(|(y1, x1)| y == *y1 && x == *x1);
        match has_it {
            Some(_) => None,
            None => {
                let arr = self.matrix.get(y);
                match arr {
                    Some(arr) => match arr.get(x) {
                        Some(value) => Some((y, x, *value)),
                        None => None,
                    },
                    None => None,
                }
            }
        }
    }

    fn go(&self, direction: Direction) -> Option<(usize, usize, i32)> {
        let (y, x) = self.coords;
        let new_position = match direction {
            Direction::Top => match y {
                0 => None,
                _ => Some((y - 1, x)),
            },
            Direction::Down => Some((y + 1, x)),
            Direction::Left => match x {
                0 => None,
                _ => Some((y, x - 1)),
            },

            Direction::Right => Some((y, x + 1)),
        };
        return match new_position {
            Some((y, x)) => self.read(y, x),
            None => None,
        };
    }
}

fn main() {
    println!("Hello, world!");
    let square = &[
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
        vec![13, 14, 15, 16],
    ];
    let r = snail(square);
    println!("{:?}", r);
}
