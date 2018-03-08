extern crate sorting_networks;

use sorting_networks::*;

fn main() {
    println!("{:?}", SortingNetwork2::new());
    println!("{:?}", SortingNetwork4::new());
    println!("{:?}", SortingNetwork8::new());
    println!("{:?}", SortingNetwork16::new());
    println!("{:?}", SortingNetwork32::new());
}
