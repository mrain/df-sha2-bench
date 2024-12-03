use std::io::Write;

use sha2::{Digest, Sha256};

pub fn hash_chain(num_iters: u64) -> [u8; 32] {
    let mut beacon = [0u8; 32];
    for _ in 0..num_iters {
        beacon = Sha256::digest(beacon).into();
    }
    beacon
}

pub fn hash_chain2(num_iters: u64) -> [u8; 32] {
    let mut beacon = [0u8; 32];
    let mut hasher = Sha256::new();
    for _ in 0..num_iters {
        hasher.write_all(&beacon).unwrap();
        hasher.finalize_into_reset((&mut beacon).into());
    }
    beacon
}

#[cfg(test)]
mod tests {
    use crate::{hash_chain, hash_chain2};

    #[test]
    fn integrity_test() {
        let num_iters = 100;
        assert_eq!(hash_chain(num_iters), hash_chain2(num_iters));
    }
}
