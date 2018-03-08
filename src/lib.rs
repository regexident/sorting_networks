#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(test), feature(lang_items))]

#[cfg(not(any(feature = "std", test)))]
extern crate core as std;

#[cfg(not(any(feature = "std", test)))]
#[macro_use]
extern crate std;

/// Iterators for enumerating a sorting network's structure.
pub mod generate {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/generate.rs"));
}

#[cfg(any(feature = "std", test))]
mod debug;

// Branchless max(x, y)/min(x, y) for unsigned integers:
//
// let x: usize = 4;
// let y: usize = 10;
//
// let min = y ^ ((x ^ y) & (!(x < y) as usize - 1));
// println!("min({}, {}) = {}", x, y, min);
//
// let max = y ^ ((x ^ y) & (!(y < x) as usize - 1));
// println!("max({}, {}) = {}", x, y, max);

#[inline]
unsafe fn swap_unchecked<T, F>(slice: &mut [T], lhs: usize, rhs: usize, compare: F)
where
    F: Fn(&T, &T) -> Ordering,
{
    use std::ptr;

    let ptr = slice.as_mut_ptr();

    let lhs_ptr = ptr.offset(lhs as isize);
    let rhs_ptr = ptr.offset(rhs as isize);

    let lhs_ref = &*lhs_ptr;
    let rhs_ref = &*rhs_ptr;

    let is_ordered = compare(&lhs_ref, &rhs_ref) == Ordering::Less;

    let min_ptr = (if is_ordered { lhs_ref } else { rhs_ref }) as *const T;
    let max_ptr = (if is_ordered { rhs_ref } else { lhs_ref }) as *const T;

    let min_val = ptr::read(min_ptr);
    let max_val = ptr::read(max_ptr);

    ptr::write(lhs_ptr, min_val);
    ptr::write(rhs_ptr, max_val);
}

/// Trait for sorting networks
pub trait SortingNetworkTrait {
    /// Sorts the passed slice
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        self.sort_by(slice, |lhs, rhs| lhs.cmp(rhs))
    }

    fn sort_by<T, F>(&self, slice: &mut [T], compare: F)
    where
        F: Fn(&T, &T) -> Ordering;
}

pub trait FixedSizeSortingNetwork {
    fn order() -> usize;

    fn width() -> usize {
        1 << Self::order()
    }
}

// http://www.iti.fh-flensburg.de/lang/algorithmen/sortieren/networks/oemen.htm
pub struct SortingNetwork;

impl SortingNetwork {
    pub fn new() -> Self {
        SortingNetwork
    }

    pub fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        let len = slice.len();
        let is_power_of_two = (len & (len - 1)) == 0;
        assert!(is_power_of_two);
        self.sort_internal(slice, 0, len);
    }

    fn sort_internal<T>(&self, slice: &mut [T], i: usize, n: usize)
    where
        T: Ord,
    {
        if n <= 1 {
            return;
        }
        let m = n / 2;
        self.sort_internal(slice, i, m);
        self.sort_internal(slice, i + m, m);
        self.merge(slice, i, n, 1);
    }

    fn merge<T>(&self, slice: &mut [T], i: usize, n: usize, interval: usize)
    where
        T: Ord,
    {
        let m = interval * 2;
        if m >= n {
            let i = i;
            let j = i + interval;
            if slice[i] > slice[j] {
                slice.swap(i, j);
                // println!("{:?}, {:?}", i, j);
            }
            return;
        }
        self.merge(slice, i, n, m);
        self.merge(slice, i + interval, n, m);
        let mut i = i + interval;
        while i + interval < n {
            let j = i + interval;
            if slice[i] > slice[j] {
                slice.swap(i, j);
                // println!("{:?}, {:?}", i, j);
            }
            i += m;
        }
    }
}

#[derive(Clone, Copy)]
struct SortingNetwork1;

impl SortingNetwork1 {
    #[inline]
    pub fn new() -> Self {
        SortingNetwork1
    }
    // pub fn width() -> usize { 1 }
}

impl SortingNetworkTrait for SortingNetwork1 {
    #[inline]
    fn sort_by<T, F>(&self, _slice: &mut [T], _compare: F)
    where
        F: Fn(&T, &T) -> Ordering,
    {
        // intentionally left blank
    }
}

include!(concat!(env!("OUT_DIR"), "/generated.rs"));
