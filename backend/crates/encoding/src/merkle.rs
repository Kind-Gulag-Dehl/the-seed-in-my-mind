#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MerkleRoot(pub Vec<u8>);

pub fn empty_payload_root() -> Vec<u8> {
    crate::hash::hash_bytes(b"seed-empty-payload")
}

pub fn compute_root_with_tags(
    leaves: &[Vec<u8>],
    domain_tag_leaf: &str,
    domain_tag_node: &str,
    sort_leaves: bool,
    empty_root: Option<Vec<u8>>,
) -> MerkleRoot {
    if leaves.is_empty() {
        let root =
            empty_root.unwrap_or_else(|| crate::hash::hash_with_domain(domain_tag_node, &[]));
        return MerkleRoot(root);
    }

    let mut leaf_bytes = leaves.to_vec();
    if sort_leaves {
        leaf_bytes.sort();
    }

    let mut level = leaf_bytes
        .iter()
        .map(|leaf| crate::hash::hash_with_domain(domain_tag_leaf, leaf))
        .collect::<Vec<_>>();

    while level.len() > 1 {
        let mut next = Vec::with_capacity(level.len().div_ceil(2));
        let mut idx = 0;
        while idx < level.len() {
            let left = &level[idx];
            let right = if idx + 1 < level.len() {
                &level[idx + 1]
            } else {
                &level[idx]
            };
            let mut node_bytes = Vec::with_capacity(left.len() + right.len());
            node_bytes.extend_from_slice(left);
            node_bytes.extend_from_slice(right);
            next.push(crate::hash::hash_with_domain(domain_tag_node, &node_bytes));
            idx += 2;
        }
        level = next;
    }

    MerkleRoot(level[0].clone())
}

pub fn compute_root(leaves: &[Vec<u8>]) -> MerkleRoot {
    compute_root_with_tags(
        leaves,
        "seed-merkle-leaf",
        "seed-merkle-node",
        true,
        Some(empty_payload_root()),
    )
}
