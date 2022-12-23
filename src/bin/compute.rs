/// Computes 2**42 iterations of SHA-256 for a given 256-bit input, hex-encoded (64 hex
/// characters).
///
/// Usage: `cargo run --release --bin compute < hex.txt`
extern crate hex;
extern crate verify_beacon;

use std::io::{self, BufRead};
use verify_beacon::sha256;
use sha2::Digest;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let count = 1024;
    let iterations = (1 << 42) / count;
    let mut lines = handle.lines().peekable();
    let btc_hash = lines.next().unwrap();
    let mut btc_hash = decode_hex(btc_hash);
    let eth_hash = lines.next().unwrap();
    let mut eth_hash = decode_hex(eth_hash);
    let zec_hash = lines.next().unwrap();
    let mut zec_hash = decode_hex(zec_hash);
    
    println!("{}", hex::encode(&btc_hash));
    println!("{}", hex::encode(&eth_hash));
    println!("{}", hex::encode(&zec_hash));
    
    let hash_result = {
        let mut h = sha2::Sha256::new();

        for beacon_hash in [btc_hash, eth_hash, zec_hash] {
            h.update(&beacon_hash);
        }
        h.finalize().to_vec()
    };
    let mut seed = [0u8;32];
    seed.copy_from_slice(&hash_result);
    println!("{}", hex::encode(&seed));
    
    let mut start = 0;
    while let Some(hash) = lines.next() {
        if lines.peek().is_none() { break; }
        seed = decode_hex(hash);
        start += 1;
    }
    for _ in start..count {
        let next = unsafe { sha256::iterated_sha256(&seed, iterations) };
        println!("{}", hex::encode(&next));
        seed = next;
    }
}

fn decode_hex(s: Result<String, io::Error>) -> [u8; 32] {
    let mut buffer = [0u8; 32];
    buffer.copy_from_slice(&hex::decode(s.unwrap()).unwrap());
    buffer
}
