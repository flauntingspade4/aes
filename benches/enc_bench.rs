use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use rand::RngCore;

use aes::{encrypt_block, encrypt_blocks_multiple, Block};

fn criterion_benchmark(c: &mut Criterion) {
    let plaintext = Block::from_str_single("Two One Nine Two");
    let key = Block::from_str_single("Thats my Kung Fu");

    c.bench_function("1 block", |b| {
        b.iter(|| {
            assert_eq!(
                black_box(encrypt_block(plaintext, key)),
                Block::new([
                    0x29, 0xc3, 0x50, 0x5f, 0x57, 0x14, 0x20, 0xf6, 0x40, 0x22, 0x99, 0xb3, 0x1a,
                    0x2, 0xd7, 0x3a
                ])
            )
        })
    });

    let mut data = vec![0; 16 * 20];
    rand::thread_rng().fill_bytes(&mut data);

    let data = data.as_slice();

    let plaintext = Block::from_slice_multiple(&data);
    let key = Block::from_str_single("Thats my Kung Fu");

    let encrypted = encrypt_blocks_multiple(plaintext, key);

    c.bench_with_input(
        BenchmarkId::new("many blocks", "random data"),
        &encrypted,
        |b, i| {
            b.iter(|| {
                let mut i = black_box(encrypt_blocks_multiple(i.clone(), key));
                assert!(i.next().is_some())
            })
        },
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
