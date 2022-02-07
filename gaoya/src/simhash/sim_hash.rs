use crate::simhash::sim_hasher::SimHasher;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::{BitOr, BitXor, Shl, Shr};

use crate::simhash::SimHashBits;

pub struct SimHash<H, S, const L: usize>
where
    H: SimHasher<T = S>,
    S: SimHashBits,
{
    hasher: H,
    marker: PhantomData<(S, H)>,
}

impl<H, S, const L: usize> SimHash<H, S, L>
where
    H: SimHasher<T = S>,
    S: SimHashBits,
{
    pub fn new(hasher: H) -> Self {
        SimHash {
            hasher: hasher,
            marker: PhantomData,
        }
    }

    pub fn create_signature<T, U>(&self, iter: T) -> S
    where
        T: Iterator<Item = U>,
        U: Hash,
    {
        let mut counts = [0i64; L];

        for mut hash in iter.map(|item| self.hasher.hash(&item)) {
            for (i, count) in counts.iter_mut().enumerate() {
                if hash & S::one() == S::zero() {
                    *count += 1;
                } else {
                    *count -= 1;
                }
                hash >>= 1;
            }
        }

        let mut result = S::zero();
        for i in 0..L {
            if counts[i] > 0 {
                result |= (S::one() << i);
            }
        }
        result
    }

    pub fn create_centroid<T>(&self, signatures: T) -> S
    where
        T: Iterator<Item = S>,
    {
        let mut counts = [0u64; L];
        let mut len = 0;
        for signature in signatures {
            for i in 0..L {
                if signature >> i & S::one() == S::one() {
                    counts[i] += 1;
                }
            }
            len += 1;
        }
        let mut centroid = S::zero();
        for i in 0..L {
            if counts[i] > len / 2 {
                centroid |= S::one() << i;
            }
        }
        centroid
    }


}

#[cfg(test)]
mod tests {
    use num_traits::real::Real;
    use super::SimHash;
    use crate::simhash::sim_hasher::SimSipHasher128;
    use crate::simhash::sim_hasher::{ShaHasher64, SimSipHasher64};
    use crate::simhash::SimHashBits;
    use crate::text::whitespace_split;

    static S1: &'static str = "Returns the number of bits necessary to represent an integer in binary, excluding the sign and leading zero";
    static S2: &'static str = "Returns the number of bits necessary to represent an integer in binary, excluding the sign and leading zero bit";
    static S3: &'static str = "local sensitive hashing is awesome";

    static S5: &'static str = "At the time of writing this article, the named tensor functionality is in experimental mode";
    static S6: &'static str = "At the time of writing this article, the named tensor functionality is in experimental mode";

    #[test]
    pub fn test_sim_hash_basics() {
        let sim_hash = SimHash::<ShaHasher64, u64, 64>::new(ShaHasher64::new());
        let s1 = sim_hash.create_signature(whitespace_split(S1));
        let s2 = sim_hash.create_signature(whitespace_split(S2));
        println!("{:b}", s1);
        println!("{:b}", s2);

        println!("s1 {} s2  {} distance {}", s1, s2, s1.hamming_distance(&s2));
    }

    #[test]
    pub fn test_sim_hash_basics128() {
        let sim_hash = SimHash::<SimSipHasher128, u128, 128>::new(SimSipHasher128::new(1, 2));
        let s1 = sim_hash.create_signature(whitespace_split(S1));
        let s2 = sim_hash.create_signature(whitespace_split(S2));
        println!("{:b}", s1);
        println!("{:b}", s1);
        println!("s1 {} s2  {} distance {}", s1, s2, s1.hamming_distance(&s2));
    }

}
