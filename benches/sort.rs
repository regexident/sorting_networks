#![feature(test)]

extern crate test;

extern crate sorting_networks;

use sorting_networks::*;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    fn shuffled(length: usize) -> Vec<usize> {
        let prime = 313373;
        let sorted: Vec<_> = (0..length).collect();
        (0..length).map(|i| sorted[(i * prime) % length]).collect()
    }

    macro_rules! def_tests {
        ($length:expr => mod $module:ident { $type:ident }) => {
            mod $module {
                use super::*;

                #[bench]
                fn sorting_network(bencher: &mut Bencher) {
                    let items = test::black_box(shuffled($length));
                    bencher.iter(|| {
                        let mut items = items.clone();
                        let sorter = $type::new();
                        sorter.sort(&mut items[..]);
                        let _ = test::black_box(items);
                    });
                }

                #[bench]
                fn stdlib(bencher: &mut Bencher) {
                    let items = test::black_box(shuffled($length));
                    bencher.iter(|| {
                        let mut items = items.clone();
                        items.sort();
                        let _ = test::black_box(items);
                    });
                }
            }
        }
    }

    def_tests!(2 => mod length_2 { SortingNetwork2 });
    def_tests!(4 => mod length_4 { SortingNetwork4 });
    def_tests!(8 => mod length_8 { SortingNetwork8 });
    def_tests!(16 => mod length_16 { SortingNetwork16 });
    def_tests!(32 => mod length_32 { SortingNetwork32 });
    def_tests!(64 => mod length_64 { SortingNetwork64 });
    def_tests!(128 => mod length_128 { SortingNetwork128 });
    def_tests!(256 => mod length_256 { SortingNetwork256 });
}
