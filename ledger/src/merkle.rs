use crypto::Blake3;

/// Computes the Merkle Root for a list of hashes.
/// If the list is empty, returns the hash of an empty string.
/// Uses a simple pairwise hashing strategy.
pub fn compute_merkle_root(hashes: &[String]) -> String {
    if hashes.is_empty() {
        return Blake3::hash(b"");
    }

    let mut current_layer = hashes.to_vec();

    while current_layer.len() > 1 {
        let mut next_layer = Vec::new();

        for chunk in current_layer.chunks(2) {
            let left = &chunk[0];
            // If the number of nodes is odd, duplicate the last node
            let right = if chunk.len() > 1 { &chunk[1] } else { left };

            let combined = format!("{}{}", left, right);
            next_layer.push(Blake3::hash(combined.as_bytes()));
        }

        current_layer = next_layer;
    }

    current_layer[0].clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merkle_root_empty() {
        let root = compute_merkle_root(&[]);
        assert_eq!(root, Blake3::hash(b""));
    }

    #[test]
    fn test_merkle_root_single() {
        let hash = Blake3::hash(b"test");
        let root = compute_merkle_root(&[hash.clone()]);
        assert_eq!(root, hash);
    }

    #[test]
    fn test_merkle_root_pair() {
        let h1 = Blake3::hash(b"test1");
        let h2 = Blake3::hash(b"test2");
        let root = compute_merkle_root(&[h1.clone(), h2.clone()]);

        let expected = Blake3::hash(format!("{}{}", h1, h2).as_bytes());
        assert_eq!(root, expected);
    }
}
