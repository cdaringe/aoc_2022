use std::ops::Range;

use aoc;
use grid;

#[derive(Default)]
struct Tree {
    height: u8,
    is_visible: bool,
    is_boundary: bool,
}

impl Tree {
    pub fn new(height: u8, is_boundary: bool) -> Self {
        Tree {
            height,
            is_visible: false,
            is_boundary,
        }
    }
}

struct Forest {
    data: grid::Grid<Tree>,
}

impl Forest {
    pub fn addr_of_pos(&self, idx: usize) -> (usize, usize) {
        let num_cols = self.data.cols();
        let col = idx % num_cols;
        let row = idx / num_cols;
        println!("idx {}, ({}, {}", idx, col, row);
        (col, row)
    }
    pub fn get_mut_at(&mut self, idx: usize) -> &mut Tree {
        let addr = self.addr_of_pos(idx);
        self.data.get_mut(addr.1, addr.0).expect("missing tree")
    }
    pub fn size(&self) -> usize {
        let (x, y) = self.data.size();
        x * y
    }
    pub fn neighbors(&self, (x, y): (usize, usize)) -> Vec<Option<&Tree>> {
        let dirs: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        dirs.map(|(dx, dy)| {
            let x_ = (x as i32) + dx;
            let y_ = (y as i32) + dy;
            if x_ >= 0 && y_ >= 0 {
                self.data.get(y_ as usize, x_ as usize)
            } else {
                None
            }
        })
        .into_iter()
        .filter(|x| x.is_some())
        .collect()
    }
    pub fn trace_visible(&self) -> Vec<usize> {
        let size = self.size();
        (0..size)
            .map(|i| {
                let addr = { self.addr_of_pos(i) };
                let (x, y) = addr;
                let tree = self
                    .data
                    .get(x, y)
                    .expect(&format!("missing tree @ {},{}", x, y));
                let neighbors: Vec<&Tree> =
                    self.neighbors((x, y)).iter().filter_map(|x| *x).collect();
                if neighbors.len() < 4 {
                    Some(i) // boundary
                } else {
                    let y1 = || (0..y).all(|yi| self.data.get(x, yi).unwrap().height < tree.height);
                    let y2 = || {
                        (Range {
                            start: y + 1,
                            end: self.data.rows(),
                        })
                        .all(|yi| self.data.get(x, yi).unwrap().height < tree.height)
                    };
                    let x1 = || (0..x).all(|xi| self.data.get(xi, y).unwrap().height < tree.height);
                    let x2 = || {
                        (Range {
                            start: x + 1,
                            end: self.data.cols(),
                        })
                        .all(|xi| self.data.get(xi, y).unwrap().height < tree.height)
                    };
                    if y1() || y2() || x1() || x2() {
                        Some(i)
                    } else {
                        None
                    }
                }
            })
            .filter_map(|x| x)
            .collect::<Vec<usize>>()
    }

    pub fn visible(&self) -> Vec<&Tree> {
        self.data.iter().filter(|x| x.is_visible).collect()
    }
}

impl From<Vec<String>> for Forest {
    fn from(lines: Vec<String>) -> Self {
        let num_rows = lines.len();
        let num_cols = lines.first().unwrap().len();
        let mut data = grid::Grid::new(0, num_cols);
        lines.iter().enumerate().for_each(|(row_idx, line)| {
            let row = line
                .split("")
                .filter(|x| !x.is_empty())
                .enumerate()
                .map(|(col_idx, c)| {
                    let is_boundary: bool = col_idx == 0
                        || row_idx == 0
                        || row_idx == num_rows - 1
                        || col_idx == num_cols - 1;
                    Tree::new(
                        c.parse::<u8>().expect("tree height must be u8"),
                        is_boundary,
                    )
                })
                .collect();
            data.insert_row(row_idx, row);
        });
        let mut f = Forest { data };
        for i in f.trace_visible() {
            f.get_mut_at(i).is_visible = true;
        }
        f
    }
}

fn main() {
    // let lines = aoc::lines("./input_p1.txt");
    let lines = aoc::lines("packages/day_08/input_p1.txt");
    let forest: Forest = lines.into();
    println!("p1: {}", forest.visible().iter().count());
}

#[cfg(test)]
mod test_day_08 {
    use crate::*;

    #[test]
    fn test_demo_input_p1() {
        let lines = aoc::lines("packages/day_08/input_p1_demo.txt");
        let forest: Forest = lines.into();
        assert_eq!(forest.visible().iter().count(), 21);
    }

    #[test]
    fn _test_demo_input_p1() {
        let lines = aoc::lines("packages/day_08/input_p1_demo.txt");
        // let lines = aoc::lines("input_p1_demo.txt");
        assert_eq!(0, 0);
    }
}
