// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// See documentation: http://doc.crates.io/build-script.html

#![recursion_limit = "128"]

extern crate core;
extern crate proc_macro;

#[macro_use]
extern crate quote;
extern crate syn;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/generate.rs"));

#[allow(unused_macros)]
macro_rules! scaffold_swap_checked {
    () => (quote! {
        #[inline]
        fn swap_checked<T, F>(slice: &mut [T], lhs: usize, rhs: usize, compare: F)
        where
            F: Fn(&T, &T) -> Ordering
        {
            let is_not_ordered = compare(slice[lhs], slice[rhs]) == Ordering::Greater;
            if is_not_ordered {
                slice.swap(lhs, rhs);
            }
        }
    })
}

macro_rules! scaffold_sorting_network_n {
    (order: $order:expr) => ({
        let last_group = Network::new($order).last().unwrap();
        let patterns: Vec<_> = last_group
            .flat_map(|block| block.flat_map(|stage| {
                let MetaPattern {
                    start,
                    count,
                    length,
                } = stage.meta_pattern();
                let (start, count, length) = (
                    start as u8, count as u8, length as u8
                );
                quote! { (#start, #count, #length) }
            })).collect();

        let count = patterns.len();

        let operations = quote! {
            let patterns: [(u8, u8, u8); #count] = [#(#patterns),*];
            for (start, count, length) in patterns.iter().cloned() {
                let (start, count, length) = (
                    start as usize, count as usize, length as usize
                );
                let gap = 2 * length;
                for i in 0..(count as usize) {
                    let offset = i * gap;
                    for j in 0..(length as usize) {
                        let min = start + offset + j;
                        let max = min + length;
                        unsafe {
                            swap_unchecked(slice, min, max, &compare);
                        }
                    }
                }
                println!();
            }
        };

        let order = $order;
        let width = (1 << $order) as usize;
        let name = syn::Ident::from(format!("SortingNetwork{}", width));

        let sub_width = (1 << ($order - 1)) as usize;
        let sub_name = syn::Ident::from(format!("SortingNetwork{}", sub_width));

        quote! {
            /// Optimized sorting network for slices of specific length.
            #[derive(Clone, Copy)]
            pub struct #name;

            impl #name {
                /// Creates a sorting network for slices of specific length.
                #[inline]
                pub fn new() -> Self {
                    #name
                }
            }

            impl SortingNetworkTrait for #name {
                fn sort_by<T, F>(&self, slice: &mut [T], compare: F)
                where
                    F: Fn(&T, &T) -> Ordering
                {
                    let len = slice.len();
                    assert!(len == #width, "Expected slice of length {}", #width);
                    {
                        let (lhs, rhs) = slice.split_at_mut(len / 2);
                        let sub_sort = #sub_name::new();
                        sub_sort.sort_by(lhs, &compare);
                        sub_sort.sort_by(rhs, &compare);
                    }
                    #(#operations)*
                }
            }

            impl FixedSizeSortingNetwork for #name {
                #[inline]
                fn order() -> usize {
                    #order
                }
            }

            impl ::std::fmt::Debug for #name {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    debug::debug_fmt(#order, f)
                }
            }
        }
    });
}

macro_rules! scaffold_tests {
    (max_order: $max_order:expr) => ({
        let tests: Vec<_> = (1..($max_order + 1)).map(|order| {
            scaffold_test!(order: order)
        }).collect();

        quote! {
            #[cfg(test)]
            mod sorting_network {
                use std::prelude::v1::*;
                use super::*;

                fn shuffled(length: usize) -> Vec<usize> {
                    let prime = 313373;
                    let sorted: Vec<_> = (0..length).collect();
                    (0..length).map(|i| sorted[(i * prime) % length]).collect()
                }

                #(#tests)*
            }
        }
    })
}

macro_rules! scaffold_test {
    (order: $order:expr) => ({
        let width: usize = 1 << $order;
        let mod_name = syn::Ident::from(format!("length_{}", width));
        let name = syn::Ident::from(format!("SortingNetwork{}", width));

        quote! {
            mod #mod_name {
                use super::*;

                #[test]
                fn fixed_size() {
                    let mut items = shuffled(#width);
                    println!("\ninput: {:?}\n", items);
                    let sorter = #name::new();
                    sorter.sort(&mut items[..]);
                    let expected: Vec<_> = (0..#width).collect();
                    assert_eq!(items, expected);
                }
            }
        }
    })
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let file_path = Path::new(&out_dir).join("generated.rs");
    let mut f = File::create(&file_path).unwrap();

    let mut tokens = vec![];

    tokens.push(quote! {
        use std::cmp::Ordering;
    });

    let max_order = 8;
    for order in 1..(max_order + 1) {
        tokens.push(scaffold_sorting_network_n!(order: order));
    }

    tokens.push(scaffold_tests!(max_order: max_order));

    let tokens = quote! {
        #(#tokens)*
    };

    f.write_all(tokens.to_string().as_bytes()).unwrap();

    let fmt_result = Command::new("rustfmt")
        .arg("--write-mode")
        .arg("overwrite")
        .arg(file_path.to_str().unwrap())
        .output();

    let _ = fmt_result;

    // if let Err(error) = fmt_result {
    //     if error.kind() == NotFound {
    //         println!("cargo:warning=Could not run rustfmt, please make sure it is in your PATH.");
    //     } else {
    //         println!("cargo:warning=Error while running rustfmt: {:?}", error.message());
    //     }
    // }
}
