use std;

pub struct BitIterator {
    value : usize,
    low_bit_index : usize,
    high_bit_index : usize,
    bit_count : usize,
}

impl BitIterator {
    pub fn new<T>(value : T) -> BitIterator
    where T : Into<usize> {
        BitIterator {
            value : value.into(),
            low_bit_index : 0,
            high_bit_index : 0,
            bit_count : std::mem::size_of::<T>() * 8,
        }
    }
}

impl Iterator for BitIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.low_bit_index < self.bit_count {
            let ret = if self.value & (1 << self.low_bit_index) == 0 {
                0
            } else {
                1
            };

            self.low_bit_index += 1;

            Some(ret)
        } else {
            None
        }
    }
}

impl DoubleEndedIterator for BitIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.high_bit_index < self.bit_count {
            let ret = if self.value & (1 << (self.bit_count - self.high_bit_index - 1)) == 0 {
                0
            } else {
                1
            };

            self.high_bit_index += 1;

            Some(ret)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn low_order() {
        assert_eq!(BitIterator::new(0x0fu8).collect::<Vec<u8>>(), vec![1, 1, 1, 1, 0, 0, 0, 0]);
    }

    #[test]
    fn high_order() {
        assert_eq!(BitIterator::new(0x0fu8).rev().collect::<Vec<u8>>(), vec![0, 0, 0, 0, 1, 1, 1, 1]);
    }
}
