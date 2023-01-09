verify-beacon
=============

This is for computing and verifying the [randomness beacon][beacon] used in the
MASP MPC ceremony, using hardware acceleration if available. Based on https://github.com/plutomonkey/verify-beacon

The beacon is computed using 2^42 iterations of SHA-256 on the SHA256 hash of the three block hashes.

The file `masp.txt` contains 1028 hashes (3 block hashes and 1024
sequential pairs), allowing the beacon to be verified more quickly in parallel.

Two hardware-accelerated implementations are available, along with a
non-accelerated fallback.  Currently, [Intel SHA extensions][intel] (e.g. AMD
Ryzen) and ARMv8 cryptographic extensions are supported.

Usage
-----

*Important:* binaries _must_ be compiled with `RUSTFLAGS='-C
target-cpu=native'` to enable hardware-acceleration.

* `cargo run --release --bin compute > masp.txt`
* `cargo run --release --bin verify < masp.txt`

Benchmarks
----------

The time taken is around 130 cycles per iteration on AMD Ryzen, which is ~1h45m
to verify on on 24 cores running at 3.8GHz.

[beacon]: https://twitter.com/namadanetwork/status/1605538847378395137
[ceremony]: https://ceremony.namada.net/
[intel]: https://en.wikipedia.org/wiki/Intel_SHA_extensions
