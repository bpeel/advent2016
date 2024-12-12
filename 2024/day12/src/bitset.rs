type Elem = u32;

pub struct BitSet {
    bits: Vec<Elem>,
}

impl BitSet {
    pub fn new() -> BitSet {
	BitSet {
	    bits: Vec::new(),
	}
    }

    pub fn set(&mut self, bit: usize) {
	let index = bit / Elem::BITS as usize;

	if self.bits.len() <= index {
	    self.bits.resize_with(index + 1, Default::default);
	}

	self.bits[index] |= 1 << (bit % Elem::BITS as usize);
    }

    pub fn contains(&self, bit: usize) -> bool {
	let index = bit / Elem::BITS as usize;

	self.bits.get(index).map(|b| {
	    b & (1 << (bit % Elem::BITS as usize)) != 0
	}).unwrap_or(false)
    }

    pub fn merge(&mut self, other: &BitSet) {
	for i in 0..self.bits.len().min(other.bits.len()) {
	    self.bits[i] |= other.bits[i];
	}

	if self.bits.len() < other.bits.len() {
	    self.bits.extend_from_slice(&other.bits[self.bits.len()..]);
	}
    }

    pub fn len(&self) -> usize {
	self.bits.iter().map(|bits| {
	    bits.count_ones() as usize
	}).sum::<usize>()
    }

    pub fn bits(&self) -> Bits {
	Bits {
	    s: &self.bits,
	    b: 0,
	    offset: usize::MAX - Elem::BITS as usize + 1,
	}
    }
}

pub struct Bits<'a> {
    s: &'a [Elem],
    b: Elem,
    offset: usize,
}

impl<'a> Iterator for Bits<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
	while self.b == 0 {
	    let Some(&next_b) = self.s.get(0)
	    else {
		return None;
	    };

	    self.s = &self.s[1..];
	    self.offset = self.offset.wrapping_add(Elem::BITS as usize);

	    self.b = next_b;
	}

	let bit_index = self.b.trailing_zeros();
	self.b &= !(1 << bit_index);

	Some(bit_index as usize + self.offset)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bit_set() {
	let mut bit_set = BitSet::new();

	assert_eq!(
	    &bit_set.bits().collect::<Vec<_>>(),
	    &[],
	);

	bit_set.set(0);
	bit_set.set(31);
	bit_set.set(68);

	assert_eq!(
	    &bit_set.bits().collect::<Vec<_>>(),
	    &[0, 31, 68],
	);

	let mut other = BitSet::new();

	other.set(1);
	other.set(96);

	bit_set.merge(&other);

	assert_eq!(
	    &bit_set.bits().collect::<Vec<_>>(),
	    &[0, 1, 31, 68, 96],
	);

	assert!(bit_set.contains(0));
	assert!(bit_set.contains(1));
	assert!(!bit_set.contains(2));
	assert!(bit_set.contains(96));
	assert!(!bit_set.contains(128));
    }
}
