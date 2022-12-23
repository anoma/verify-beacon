/// Parallel verification of a 2**42 SHA-256 random beacon.
///
/// Usage: `cargo run --release --bin verify < 1024.txt`
///
/// Input should be 1025 lines, i.e. 1024 overlapping pairs, each line being 256 bits, hex-encoded
/// (64 hex characters).
extern crate hex;
extern crate itertools;
extern crate rayon;
extern crate verify_beacon;

use sha2::Digest;
use itertools::Itertools;
use rayon::prelude::*;
use std::io::{self, BufRead};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use verify_beacon::sha256;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let btc_hash = lines.next().unwrap();
    let btc_hash = decode_hex(btc_hash);
    let eth_hash = lines.next().unwrap();
    let eth_hash = decode_hex(eth_hash);
    let zec_hash = lines.next().unwrap();
    let zec_hash = decode_hex(zec_hash);

    assert_eq!(btc_hash, decode_hex(Ok("00000000000000000006ccea7a2c42ff8cc2b1b9bb98e159cd922fa30dfe770f".to_string())));
    assert_eq!(eth_hash, decode_hex(Ok("232c260a02b07e754c6b685452439b5b99d91f0f87f46899ad3ba1aab54d364e".to_string())));
    assert_eq!(zec_hash, decode_hex(Ok("0000000000a276663e3d2acab5e571d815fca496e6a84f87841783a68ba779bb".to_string())));
    
    println!("BTC block hash: {}", hex::encode(&btc_hash));
    println!("ETH block hash: {}", hex::encode(&eth_hash));
    println!("ZEC block hash: {}", hex::encode(&zec_hash));

    let block_hashes = {
        let mut h = sha2::Sha256::new();

        for beacon_hash in [btc_hash, eth_hash, zec_hash] {
            h.update(&beacon_hash);
        }
        
        let mut seed = [0u8;32];
        seed.copy_from_slice(&h.finalize().to_vec());
        seed
    };
    
    let pairs = lines
        .map(decode_hex)
        .tuple_windows()
        .collect::<Vec<([u8; 32], [u8; 32])>>();
    
    assert_eq!(pairs[0].0, block_hashes);
    let count = pairs.len();
    let iterations = (1 << 42) / count;
    let remaining = Arc::new(AtomicUsize::new(count));
    pairs.par_iter().for_each(|(a, b)| {
        verify(&a, &b, iterations);
        println!(
            "remaining={}/{}",
            remaining.fetch_sub(1, Ordering::Relaxed) - 1,
            count
        );
    });
}

fn verify(a: &[u8; 32], b: &[u8; 32], iterations: usize) {
    unsafe {
        let result = sha256::iterated_sha256(a, iterations);
        assert_eq!(b, &result);
    }
}

fn decode_hex(s: Result<String, io::Error>) -> [u8; 32] {
    let mut buffer = [0u8; 32];
    buffer.copy_from_slice(&hex::decode(s.unwrap()).unwrap());
    buffer
}
