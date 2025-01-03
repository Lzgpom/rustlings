// Calculates the power of 2 using a bit shift.
// `1 << n` is equivalent to "2 to the power of n".
fn power_of_2(n: u8) -> u64 {
    1 << n
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert_eq() {
        assert_eq!(2, power_of_2(1));
        assert_eq!(16, power_of_2(4));
        assert_eq!(1024, power_of_2(10));
        assert_eq!(32, power_of_2(5));
    }
}
