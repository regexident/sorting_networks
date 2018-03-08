#[derive(Clone, Copy, Debug)]
pub struct Cursor {
    pub index: usize,
    pub count: usize,
}

impl Cursor {
    pub fn new(index: usize, count: usize) -> Self {
        Self { index, count }
    }
}

pub struct Network {
    order: usize,
    cursor: Cursor,
}

impl Network {
    pub fn new(order: usize) -> Self {
        let groups = order;
        let cursor = Cursor::new(0, groups);
        Self { order, cursor }
    }

    pub fn order(&self) -> usize {
        self.order
    }

    pub fn groups(&self) -> usize {
        self.cursor.count
    }
}

impl Iterator for Network {
    type Item = Group;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor.index >= self.cursor.count {
            return None;
        }
        let length = 1 << self.order;
        let index = self.cursor.index;
        let blocks = 1 << (self.cursor.count - self.cursor.index - 1);
        self.cursor.index += 1;
        Some(Group::new(length, index, blocks))
    }
}

#[derive(Clone, Debug)]
pub struct Group {
    length: usize,
    group_index: usize,
    cursor: Cursor,
}

impl Group {
    fn new(length: usize, group_index: usize, blocks: usize) -> Self {
        let cursor = Cursor::new(0, blocks);
        Self { length, group_index, cursor }
    }

    pub fn blocks(&self) -> usize {
        self.cursor.count
    }
}

impl Iterator for Group {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor.index >= self.cursor.count {
            return None;
        }

        let offset = self.cursor.index * (self.length / self.cursor.count);

        self.cursor.index += 1;
        Some(Block::new(offset, self.group_index))
    }
}

#[derive(Clone, Debug)]
pub struct Block {
    offset: usize,
    group_index: usize,
    cursor: Cursor,
}

impl Block {
    fn new(offset: usize, group_index: usize) -> Self {
        let stages = group_index + 1;
        let cursor = Cursor::new(0, stages);

        Self { offset, group_index, cursor }
    }

    pub fn stages(&self) -> usize {
        self.cursor.count
    }
}

impl Iterator for Block {
    type Item = Stage;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor.index >= self.cursor.count {
            return None;
        }
        let stage_index = self.cursor.index;
        let distance = 1 << (self.group_index - stage_index);
        let offset = if stage_index == 0 {
            self.offset
        } else {
            self.offset + distance
        };
        self.cursor.index += 1;
        Some(Stage::new(self.group_index, stage_index, offset))
    }
}

#[derive(Clone, Debug)]
pub struct Stage {
    distance: usize,
    offset: usize,
    cursor: Cursor,
}

impl Stage {
    fn new(group_index: usize, stage_index: usize, offset: usize) -> Self {
        let mut patterns = 0;
        let distance = 1 << (group_index - stage_index);

        let mut i = 1 << (group_index + 1);
        if stage_index > 0 {
            i -= distance;
        };

        while i > distance {
            patterns += 1;
            i -= 2 * distance;
        }

        let cursor = Cursor::new(0, patterns);
        Self { distance, offset, cursor }
    }

    pub fn patterns(&self) -> usize {
        self.cursor.count
    }

    pub fn meta_pattern(&self) -> MetaPattern {
        let start = self.offset;
        let count = self.cursor.count;
        let length = self.distance;
        MetaPattern {
            start,
            count,
            length,
        }
    }

    pub fn distance(&self) -> usize {
        self.distance * 2
    }
}

impl Iterator for Stage {
    type Item = Pattern;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor.index >= self.cursor.count {
            return None;
        }

        let offset = self.cursor.index * (2 * self.distance);
        self.cursor.index += 1;

        let min = self.offset + offset;
        let max = min + self.distance;
        let count = self.distance;

        Some(Pattern::new(min, max, count))
    }
}

#[derive(Clone, Debug)]
pub struct MetaPattern {
    /// The first sub-pattern's lower index
    pub start: usize,
    /// The number of sub-patterns
    pub count: usize,
    /// The length of sub-patterns
    pub length: usize,
}

#[derive(Clone, Debug)]
pub struct Pattern {
    min: usize,
    max: usize,
    cursor: Cursor,
}

impl Pattern {
    fn new(min: usize, max: usize, count: usize) -> Self {
        let cursor = Cursor::new(0, count);
        Self { min, max, cursor }
    }

    pub fn pairs(&self) -> usize {
        self.cursor.count
    }
}

impl Iterator for Pattern {
    type Item = Pair;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor.index >= self.cursor.count {
            return None;
        }

        let offset = self.cursor.index;
        self.cursor.index += 1;

        let min = self.min + offset;
        let max = self.max + offset;

        Some(Pair::new(min, max))
    }
}

#[derive(Clone, Debug)]
pub struct Pair {
    /// Index of min element
    pub min: usize,
    /// Index of max element
    pub max: usize,
}

impl Pair {
    fn new(min: usize, max: usize) -> Self {
        Self { min, max }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//
//     }
// }
