#![feature(test)]

extern crate byteorder;
extern crate speedy;
extern crate test;
#[macro_use]
extern crate criterion;

use byteorder::{NativeEndian, ReadBytesExt};
use criterion::Criterion;
use speedy::{Endianness, Readable, Writable};
use std::io::{Read, Write};
use test::black_box;

fn deserialization_manual_bytes(c: &mut Criterion) {
    let original: &[u8] = black_box(&[1, 2, 3, 4]);
    let data = original.write_to_vec(Endianness::NATIVE).unwrap();
    c.bench_function("Manual byte deserialization", move |b| {
        b.iter(|| {
            let mut buffer = &data[..];

            let len = buffer.read_u32::<NativeEndian>().unwrap() as usize;
            let mut vec = Vec::with_capacity(len);
            unsafe {
                vec.set_len(len);
            }
            buffer.read_exact(&mut vec[..]).unwrap();

            vec
        })
    });
}

fn deserialization_speedy_bytes(c: &mut Criterion) {
    let original: &[u8] = black_box(&[1, 2, 3, 4]);
    let data = original.write_to_vec(Endianness::NATIVE).unwrap();
    c.bench_function("Speedy byte deserialization", move |b| {
        b.iter(|| {
            let deserialized: Vec<u8> =
                Readable::read_from_buffer(Endianness::NATIVE, &data).unwrap();
            deserialized
        })
    });
}

fn deserialization_manual_string(c: &mut Criterion) {
    let original: &str = black_box("Hello world!");
    let data = original.write_to_vec(Endianness::NATIVE).unwrap();
    c.bench_function("Manual string deserialization", move |b| {
        b.iter(|| {
            let mut buffer = &data[..];

            let len = buffer.read_u32::<NativeEndian>().unwrap() as usize;
            let mut vec = Vec::with_capacity(len);
            unsafe {
                vec.set_len(len);
            }
            buffer.read_exact(&mut vec[..]).unwrap();
            String::from_utf8(vec)
        })
    });
}

fn deserialization_speedy_string(c: &mut Criterion) {
    let original: &str = black_box("Hello world!");
    let data = original.write_to_vec(Endianness::NATIVE).unwrap();
    c.bench_function("Speedy string deserialization", move |b| {
        b.iter(|| {
            let deserialized: String =
                Readable::read_from_buffer(Endianness::NATIVE, &data).unwrap();
            deserialized
        })
    });
}

fn deserialization_manual_u8(c: &mut Criterion) {
    let original: u8 = black_box(12);
    let data = original.write_to_vec(Endianness::NATIVE).unwrap();
    c.bench_function("Manual u8 deserialization", move |b| {
        b.iter(|| {
            let mut buffer = &data[..];
            buffer.read_u8().unwrap()
        })
    });
}

fn deserialization_speedy_u8(c: &mut Criterion) {
    let original: u8 = black_box(12);
    let data = original.write_to_vec(Endianness::NATIVE).unwrap();
    c.bench_function("Speedy u8 deserialization", move |b| {
        b.iter(|| {
            let deserialized: u8 = Readable::read_from_buffer(Endianness::NATIVE, &data).unwrap();
            deserialized
        })
    });
}



fn deserialization_manual_u64(c: &mut Criterion) {
    let original: u64 = black_box(1234);
    let data = original.write_to_vec(Endianness::NATIVE).unwrap();
    c.bench_function("Manual u64 deserialization", move |b| {
        b.iter(|| {
            let mut buffer = &data[..];
            buffer.read_u64::<NativeEndian>().unwrap()
        })
    });
}

fn deserialization_speedy_u64(c: &mut Criterion) {
    let original: u64 = black_box(1234);
    let data = original.write_to_vec(Endianness::NATIVE).unwrap();
    c.bench_function("Speedy u64 deserialization", move |b| {
        b.iter(|| {
            let deserialized: u64 = Readable::read_from_buffer(Endianness::NATIVE, &data).unwrap();
            deserialized
        })
    });
}

fn serialization_manual_megabyte_buffer(c: &mut Criterion) {
    let mut buffer: Vec<u8> = Vec::new();
    buffer.resize(1024 * 1024, 1);
    buffer = black_box(buffer);
    c.bench_function("Manual megabyte buffer serialization", move |b| {
        b.iter(|| {
            let mut output = Vec::new();
            Write::write_all(&mut output, &buffer).unwrap();
            output
        })
    });
}

// These two benchmarks should have exactly the same speeds.

fn serialization_speedy_megabyte_buffer_little_endianness(c: &mut Criterion) {
    let mut buffer: Vec<u8> = Vec::new();
    buffer.resize(1024 * 1024, 1);
    buffer = black_box(buffer);
    c.bench_function("Speedy megabyte buffer serialization LE", move |b| {
        b.iter(|| {
            let mut output = Vec::new();
            buffer
                .write_to_stream(Endianness::LittleEndian, &mut output)
                .unwrap();
            output
        })
    });
}

fn serialization_speedy_megabyte_buffer_big_endianness(c: &mut Criterion) {
    let mut buffer: Vec<u8> = Vec::new();
    buffer.resize(1024 * 1024, 1);
    buffer = black_box(buffer);
    c.bench_function("Speedy megabyte buffer serialization LE", move |b| {
        b.iter(|| {
            let mut output = Vec::new();
            buffer
                .write_to_stream(Endianness::BigEndian, &mut output)
                .unwrap();
            output
        })
    });
}
criterion_group!(benches, 
deserialization_manual_bytes,
deserialization_speedy_bytes,
deserialization_manual_string,
deserialization_speedy_string,
deserialization_manual_u8,
deserialization_speedy_u8,
deserialization_manual_u64,
deserialization_speedy_u64,
serialization_manual_megabyte_buffer,
serialization_manual_megabyte_buffer,
serialization_speedy_megabyte_buffer_little_endianness, 
serialization_speedy_megabyte_buffer_big_endianness
);


criterion_main!(benches);
