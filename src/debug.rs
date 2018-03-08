use std::fmt;
use std::prelude::v1::*;

use generate::*;

pub fn debug_fmt(order: usize, f: &mut fmt::Formatter) -> fmt::Result {
    let length = 1 << order;

    let group_widths: Vec<usize> = (0..order)
        .map(|group| {
            let stage_count = group + 1;
            let group_width = (0..stage_count).fold(0, |width, stage| {
                let stage_width = 1 << stage;
                width + stage_width
            });
            group_width
        })
        .collect();
    let width: usize = group_widths.iter().sum();

    let mut wires: Vec<Vec<char>> = vec![vec!['─'; width * 2 + 1]; length];

    let network = Network::new(order);
    for (index, group) in network.enumerate() {
        let x = (0..index).fold(0, |sum, group| sum + group_widths[group]);
        for block in group {
            let mut x = x;
            let stages = block.stages();
            for (index, stage) in block.enumerate() {
                for pattern in stage {
                    for (index, pair) in pattern.enumerate() {
                        let x = x + index;
                        for y in (pair.min)..(pair.max + 1) {
                            let character = if y == pair.min {
                                '┰'
                            } else if y == pair.max {
                                '┸'
                            } else {
                                '╂'
                            };
                            wires[y][x * 2 + 1] = character;
                        }
                    }
                }
                let stage_width = 1 << (stages - index - 1);
                x += stage_width;
            }
        }
    }

    let mut result = Ok(());
    for (index, wire) in wires.into_iter().enumerate() {
        let string: String = wire.into_iter().collect();
        result = writeln!(f, "{:3}: {}", index, string);
    }
    result
}
