use encoding::canonical::{
    canonicalize_utf8, encode_id, encode_string, encode_varint_u64, validate_id,
};
use encoding::hash::{hash_bytes, hash_with_domain};
use encoding::merkle::{compute_root_with_tags, empty_payload_root};

fn to_hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        out.push_str(&format!("{:02x}", b));
    }
    out
}

#[test]
fn string_encoding_vectors() {
    let empty = encode_string("");
    assert_eq!(to_hex(&empty), "00");

    let hello = encode_string("hello");
    assert_eq!(to_hex(&hello), "0568656c6c6f");
}

#[test]
fn id_encoding_vector() {
    let id = "018f5c2a-7c4d-7b2e-8f9a-9d2b5a0c3f1d";
    let encoded = encode_id(id).expect("id should be valid");
    assert_eq!(
        to_hex(&encoded),
        "0000002430313866356332612d376334642d376232652d386639612d396432623561306333663164"
    );
}

#[test]
fn id_validation_rejects_uppercase() {
    let id = "018F5c2a-7c4d-7b2e-8f9a-9d2b5a0c3f1d";
    assert!(validate_id(id).is_err());
}

#[test]
fn payload_canonicalization_lf() {
    let input = b"Hello\r\nWorld";
    let canonical = canonicalize_utf8(input).expect("canonicalization should succeed");
    assert_eq!(canonical, b"Hello\nWorld");
}

#[test]
fn varint_encoding_matches_examples() {
    let len = encode_varint_u64(5);
    assert_eq!(to_hex(&len), "05");
    let len = encode_varint_u64(0);
    assert_eq!(to_hex(&len), "00");
}

#[test]
fn domain_separation_changes_hash() {
    let a = hash_with_domain("seed-a", b"");
    let b = hash_with_domain("seed-b", b"");
    assert_ne!(a, b);
}

#[test]
fn merkle_root_matches_manual_construction() {
    let tag_leaf = "seed-merkle-leaf";
    let tag_node = "seed-merkle-node";
    let leaves = vec![b"c".to_vec(), b"a".to_vec(), b"b".to_vec()];

    let mut leaf_bytes = leaves.clone();
    leaf_bytes.sort();
    let mut level = leaf_bytes
        .iter()
        .map(|leaf| {
            let mut buf = Vec::new();
            buf.extend_from_slice(tag_leaf.as_bytes());
            buf.extend_from_slice(leaf);
            blake3::hash(&buf).as_bytes().to_vec()
        })
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
            let mut node_bytes = Vec::new();
            node_bytes.extend_from_slice(left);
            node_bytes.extend_from_slice(right);
            let mut buf = Vec::new();
            buf.extend_from_slice(tag_node.as_bytes());
            buf.extend_from_slice(&node_bytes);
            next.push(blake3::hash(&buf).as_bytes().to_vec());
            idx += 2;
        }
        level = next;
    }

    let expected = level[0].clone();
    let computed = compute_root_with_tags(
        &leaves,
        tag_leaf,
        tag_node,
        true,
        Some(empty_payload_root()),
    );
    assert_eq!(computed.0, expected);
}

#[test]
fn empty_payload_root_matches_spec_constant() {
    let expected = hash_bytes(b"seed-empty-payload");
    let computed = compute_root_with_tags(
        &[],
        "seed-merkle-leaf",
        "seed-merkle-node",
        true,
        Some(empty_payload_root()),
    );
    assert_eq!(computed.0, expected);
}
