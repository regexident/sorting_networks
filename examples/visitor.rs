extern crate sorting_networks;

use sorting_networks::generate::*;

trait Visitor<T> {
    type Payload;

    fn visit(&self, visitee: T, payload: Self::Payload);
}

struct NetworkVisitor;

impl Visitor<Network> for NetworkVisitor {
    type Payload = ();

    fn visit(&self, network: Network, _payload: ()) {
        println!("network {:?}", 1 << network.order());
        let groups = network.groups();
        for (index, group) in network.enumerate() {
            self.visit(group, Cursor::new(index, groups));
        }
    }
}

impl Visitor<Group> for NetworkVisitor {
    type Payload = Cursor;

    fn visit(&self, group: Group, cursor: Cursor) {
        println!("→ group {:?}/{:?}", cursor.index, cursor.count);
        let blocks = group.blocks();
        for (index, block) in group.enumerate() {
            self.visit(block, Cursor::new(index, blocks));
        }
    }
}

impl Visitor<Block> for NetworkVisitor {
    type Payload = Cursor;

    fn visit(&self, block: Block, cursor: Cursor) {
        println!("  → block {:?}/{:?}", cursor.index, cursor.count);
        let stages = block.stages();
        for (index, stage) in block.enumerate() {
            self.visit(stage, Cursor::new(index, stages));
        }
    }
}

impl Visitor<Stage> for NetworkVisitor {
    type Payload = Cursor;

    fn visit(&self, stage: Stage, cursor: Cursor) {
        println!("    → stage {:?}/{:?}", cursor.index, cursor.count);
        let patterns = stage.patterns();
        for (index, pattern) in stage.enumerate() {
            self.visit(pattern, Cursor::new(index, patterns));
        }
    }
}

impl Visitor<Pattern> for NetworkVisitor {
    type Payload = Cursor;

    fn visit(&self, pattern: Pattern, cursor: Cursor) {
        println!("      → pattern {:?}/{:?}", cursor.index, cursor.count);
        let pairs = pattern.pairs();
        for (index, pair) in pattern.enumerate() {
            self.visit(pair, Cursor::new(index, pairs));
        }
    }
}

impl Visitor<Pair> for NetworkVisitor {
    type Payload = Cursor;

    fn visit(&self, pair: Pair, cursor: Cursor) {
        println!(
            "        → pair {:?}/{:?}: {:?}-{:?}",
            cursor.index, cursor.count, pair.min, pair.max
        );
    }
}

fn main() {
    let order = 4;
    let network = Network::new(order);
    let visitor = NetworkVisitor;

    visitor.visit(network, ());
}
