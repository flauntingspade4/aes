#![warn(clippy::nursery, clippy::pedantic)]
#![no_std]

mod block;
pub mod dec;
pub mod enc;
mod key_expansion;

pub use block::Block;
pub use enc::{encrypt_block, encrypt_blocks_multiple};

#[test]
fn encrypt_test() {
    let plaintext = Block::from_str_single("Two One Nine Two");
    let key = Block::from_str_single("Thats my Kung Fu");

    let block = encrypt_block(plaintext, key);

    assert_eq!(
        block,
        Block::new([
            0x29, 0xc3, 0x50, 0x5f, 0x57, 0x14, 0x20, 0xf6, 0x40, 0x22, 0x99, 0xb3, 0x1a, 0x2,
            0xd7, 0x3a
        ])
    );

    let plaintext = Block::new([
        0x32, 0x43, 0xf6, 0xa8, 0x88, 0x5a, 0x30, 0x8d, 0x31, 0x31, 0x98, 0xa2, 0xe0, 0x37, 0x07,
        0x34,
    ]);
    let key = Block::new([
        0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f,
        0x3c,
    ]);

    let block = encrypt_block(plaintext, key);

    assert_eq!(
        block,
        Block::new([
            0x39, 0x25, 0x84, 0x1d, 0x02, 0xdc, 0x09, 0xfb, 0xdc, 0x11, 0x85, 0x97, 0x19, 0x6a,
            0x0b, 0x32
        ])
    );

    let test = [
        Block::new([
            0x80, 0xc1, 0x99, 0x5a, 0x44, 0xe2, 0xfa, 0xae, 0x3f, 0x8d, 0x6, 0x4c, 0x15, 0x82,
            0x60, 0xea,
        ]),
        Block::new([
            0x5a, 0x66, 0xb0, 0x48, 0x0d, 0x4f, 0x5c, 0xa3, 0x5d, 0xb2, 0xe5, 0x1e, 0x3a, 0xed,
            0x36, 0x25,
        ]),
    ];

    let blocks = Block::from_str_multiple("This is a long message 123456789");

    for (encrypted_block, test) in encrypt_blocks_multiple(blocks, key).zip(test) {
        assert_eq!(encrypted_block, test);
    }
}
