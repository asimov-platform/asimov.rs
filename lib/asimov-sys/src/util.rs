// This is free and unencumbered software released into the public domain.

#![allow(dead_code)]

extern crate alloc;

use alloc::vec::Vec;
use core::{cmp::min, ffi::c_char, iter::repeat, mem::size_of, slice};

pub fn size_of_result<F, T, U>(_f: F) -> usize
where
    F: FnOnce(T) -> U,
{
    size_of::<U>()
}

pub fn string_to_static_array<const N: usize>(input: &str) -> [c_char; N] {
    let input_bytes: &[c_char] = unsafe { slice::from_raw_parts(input.as_ptr() as _, input.len()) };
    let input_len = min(input_bytes.len(), N - 1); // decrement NUL
    let mut output = [0 as c_char; N];
    output[..input_len].copy_from_slice(&input_bytes[..input_len]);
    output
}

pub fn string_to_dynamic_array(input: &str, capacity: usize) -> Vec<c_char> {
    let input_bytes = input.as_bytes();
    let input_len = min(input_bytes.len(), capacity - 1); // decrement NUL
    input_bytes[..input_len]
        .iter()
        .map(|b| *b as c_char)
        .chain(repeat(0 as c_char))
        .take(capacity)
        .collect::<Vec<c_char>>()
}
