#![no_std]
#![feature(test)]
#[macro_use]
extern crate crypto_tests;
extern crate sha1_asm as sha1;

bench_digest!(sha1::Sha1);
