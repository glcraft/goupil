use std::{mem::MaybeUninit, sync::LockResult};

///! sha256 implementation
///! https://fr.wikipedia.org/wiki/SHA-2

const CONSTANTS: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

const INITIALIZATION: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

#[inline]
fn ch(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (!x & z)
}
#[inline]
fn maj(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (x & z) ^ (y & z)
}
#[inline]
fn sum0(x: u32) -> u32 {
    u32::rotate_right(x, 2) ^ u32::rotate_right(x, 13) ^ u32::rotate_right(x, 22)
}
#[inline]
fn sum1(x: u32) -> u32 {
    u32::rotate_right(x, 6) ^ u32::rotate_right(x, 11) ^ u32::rotate_right(x, 25)
}
#[inline]
fn theta0(x: u32) -> u32 {
    u32::rotate_right(x, 7) ^ u32::rotate_right(x, 18) ^ (x >> 3)
}
#[inline]
fn theta1(x: u32) -> u32 {
    u32::rotate_right(x, 17) ^ u32::rotate_right(x, 19) ^ (x >> 10)
}
pub fn fill_bits<const N: u32>(buffer: &[u8]) -> Vec<u8> {
    let nbytes: usize = N as usize / 8;

    let max_bytes = N - 64;
    let k = (max_bytes as i64 - (buffer.len() * 8 + 1) as i64).rem_euclid(N as i64) as u64;
    let mut result = Vec::with_capacity(((buffer.len() / nbytes) + 1) * nbytes);
    result.extend_from_slice(buffer);
    result.push(0x80);
    for _ in 0..(k / 8) {
        result.push(0);
    }
    for c in (buffer.len() * 8).to_be_bytes() {
        result.push(c);
    }
    result
}
pub fn sha256(buffer: &[u8]) -> [u8; 32] {
    const BLOCK_BYTES: usize = 512 / 8;
    let result = INITIALIZATION.clone();
    let msg = fill_bits::<512>(buffer);
    for ibuf in 0..=(buffer.len() / BLOCK_BYTES) {
        let part = &msg[(ibuf * BLOCK_BYTES)..((ibuf + 1) * BLOCK_BYTES)];
        let w: [u32; 64] = unsafe {
            let mut w = MaybeUninit::uninit();
            for i in 0..15 {
                w[i] = u32::from_be_bytes(part[(i * 4)..((i + 1) * 4)]);
            }

            w
        };
    }
    todo!()
}
