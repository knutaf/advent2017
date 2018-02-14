use super::*;

const KNOT_HASH_RING_SIZE : u32 = 256;
const KNOT_HASH_BLOCK_SIZE : usize = 16;

pub struct Ring {
    ring : Vec<u8>,
    pos : usize,
    skip_size : usize,
}

impl Ring {
    pub fn new(length : u32) -> Ring {
        Ring {
            ring : (0 .. length).map(|n| n as u8).collect(),
            pos : 0,
            skip_size : 0,
        }
    }

    pub fn advance(&mut self, length : usize) -> u32 {
        reverse_circular_vec_segment(&mut self.ring, self.pos, length);

        self.pos += length + self.skip_size;
        while self.pos >= self.ring.len() {
            self.pos -= self.ring.len();
        }

        self.skip_size += 1;

        (self.ring[0] as u32) * (self.ring[1] as u32)
    }

    fn reduce(&self, block_size : usize) -> Vec<u8> {
        let mut result : Vec<u8> = vec![];

        for (i, num) in self.ring.iter().enumerate() {
            if (i % block_size) == 0 {
                result.push(*num);
            } else {
                let last = *result.last().unwrap();
                *result.last_mut().unwrap() = last ^ num;
            }
        }

        result
    }
}

pub fn knot_hash(input : &str) -> Vec<u8> {
    const SUFFIX_LENGTHS : [u8 ; 5] = [17u8, 31u8, 73u8, 47u8, 23u8];
    const NUM_ROUNDS : u32 = 64;

    let mut ring = Ring::new(KNOT_HASH_RING_SIZE);

    for _ in 0 .. NUM_ROUNDS {
        for b in input.bytes() {
            let _ = ring.advance(b as usize);
        }

        for b in SUFFIX_LENGTHS.iter() {
            let _ = ring.advance(*b as usize);
        }
    }

    ring.reduce(KNOT_HASH_BLOCK_SIZE)
}

pub fn knot_hash_as_hex(input : &str) -> String {
    numbers_to_hex_string(knot_hash(input).iter().map(|n| *n as u8))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn reduce_default_2() {
        let ring = Ring::new(10);
        assert_eq!(ring.reduce(2), vec![1, 1, 1, 1, 1]);
    }

    #[test]
    fn reduce_default_3() {
        let ring = Ring::new(9);
        assert_eq!(ring.reduce(3), vec![3, 2, 9]);
    }

    #[test]
    fn reduce_size_1() {
        let ring = Ring::new(5);
        assert_eq!(ring.reduce(1), vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn knot_hash() {
        assert_eq!(knot_hash_as_hex(""), "a2582a3a0e66e6e86e3812dcb672a272");
        assert_eq!(knot_hash_as_hex("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
        assert_eq!(knot_hash_as_hex("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
        assert_eq!(knot_hash_as_hex("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
    }
}
