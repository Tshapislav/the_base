//! ba04

const U8_MIN_BOUND: u8 = 0b0000_0000;
const U8_MAX_BOUND: u8 = 0b1111_1111;

fn main() {
    todo!();
}

fn add_u8_checked(a: u8, b: u8) -> Option<u8> {
    let inverted = U8_MAX_BOUND ^ a;
    if b > inverted {
        return None;
    } else {
        return Some(a + b);
    }
}

fn add_u8_wrapping(a: u8, b: u8) -> u8 {
    let inverted = U8_MAX_BOUND ^ a;
    if b > inverted {
        return U8_MIN_BOUND + (b - inverted - 1);
    } else {
        return a + b;
    }
}

fn add_u8_saturating(a: u8, b: u8) -> u8 {
    let inverted = U8_MAX_BOUND ^ a;
    if b > inverted {
        return U8_MAX_BOUND;
    } else {
        return a + b;
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn unsigned_overflow_modes() {
        assert_eq!(add_u8_checked(255, 1), None);
        assert_eq!(add_u8_wrapping(255, 1), 0);
        assert_eq!(add_u8_saturating(255, 1), 255);
        assert_eq!(add_u8_checked(10, 20), Some(30));
        assert_eq!(add_u8_wrapping(10, 20), 30);
        assert_eq!(add_u8_saturating(10, 20), 30);
    }
}
