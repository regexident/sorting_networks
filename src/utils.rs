pub fn log2ceil(value: usize) -> usize {
    let mut i = 0;
    let mut cmp = 1;
    while cmp < value {
        i += 1;
        cmp <<= 1;
    }
    i
}

pub fn is_power_of_two(value: usize) -> bool {
    (value & (value - 1)) == 0
}
