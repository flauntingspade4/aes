use crate::{
    key_expansion::{s_box, KeyExpander},
    Block,
};

#[must_use]
pub fn encrypt_block(mut plaintext: Block, key: Block) -> Block {
    let mut key_expander = KeyExpander::new(key);

    plaintext ^= key;

    for _ in 0..9 {
        full_round(&mut plaintext, key_expander.next_key());
    }

    final_round(&mut plaintext, key_expander.next_key());

    plaintext
}

#[must_use]
pub fn encrypt_blocks_multiple<'a>(
    plaintext: impl Iterator<Item = Block> + Clone + 'a,
    key: Block,
) -> impl Iterator<Item = Block> + Clone + 'a {
    let keys = KeyExpander::all_round_keys(key);

    plaintext.map(move |mut plaintext| {
        plaintext ^= key;

        for i in 0..9 {
            full_round(&mut plaintext, keys[i]);
        }

        final_round(&mut plaintext, keys[keys.len() - 1]);

        plaintext
    })
}

fn full_round(block: &mut Block, round_key: Block) {
    sub_bytes(block);

    shift_rows(block);

    mix_columns(block);

    *block ^= round_key;
}

fn final_round(block: &mut Block, round_key: Block) {
    sub_bytes(block);

    shift_rows(block);

    *block ^= round_key;
}

fn sub_bytes(block: &mut Block) {
    for row in &mut block.0 {
        for item in row {
            *item = s_box(*item);
        }
    }
}

fn shift_rows(block: &mut Block) {
    let max = block.0.len();

    for y in 1..max {
        for _ in 0..y {
            let temp = block.0[0][y];

            for i in 0..max - 1 {
                block.0[i][y] = block.0[i + 1][y];
            }

            block.0[3][y] = temp;
        }
    }
}

fn mix_columns(block: &mut Block) {
    fn g_mul(mut a: u8, mut b: u8) -> u8 {
        let mut p = 0;

        for _ in 0..8 {
            if b & 1 != 0 {
                p ^= a;
            }

            let high_bit_set = a & 0x80 != 0;
            a <<= 1;

            if high_bit_set {
                a ^= 0x1b;
            }

            b >>= 1;
        }

        p
    }

    let mut output = Block::new([0; 16]);

    for c in 0..4 {
        output.0[c][0] =
            g_mul(0x02, block.0[c][0]) ^ g_mul(0x03, block.0[c][1]) ^ block.0[c][2] ^ block.0[c][3];
        output.0[c][1] =
            block.0[c][0] ^ g_mul(0x02, block.0[c][1]) ^ g_mul(0x03, block.0[c][2]) ^ block.0[c][3];
        output.0[c][2] =
            block.0[c][0] ^ block.0[c][1] ^ g_mul(0x02, block.0[c][2]) ^ g_mul(0x03, block.0[c][3]);
        output.0[c][3] =
            g_mul(0x03, block.0[c][0]) ^ block.0[c][1] ^ block.0[c][2] ^ g_mul(0x02, block.0[c][3]);
    }

    *block = output;
}

#[test]
fn mix_columns_test() {
    let mut block = Block::new([
        0x63, 0x2f, 0xaf, 0xa2, 0xeb, 0x93, 0xc7, 0x20, 0x9f, 0x92, 0xab, 0xcb, 0xa0, 0xc0, 0x30,
        0x2b,
    ]);

    mix_columns(&mut block);

    assert_eq!(
        block,
        Block::new([
            0xba, 0x75, 0xf4, 0x7a, 0x84, 0xa4, 0x8d, 0x32, 0xe8, 0x8d, 0x06, 0x0e, 0x1b, 0x40,
            0x7d, 0x5d
        ])
    );

    block = Block::new([
        0xd4, 0xbf, 0x5d, 0x30, 0xe0, 0xb4, 0x52, 0xae, 0xb8, 0x41, 0x11, 0xf1, 0x1e, 0x27, 0x98,
        0xe5,
    ]);

    mix_columns(&mut block);

    assert_eq!(
        block,
        Block::new([
            0x04, 0x66, 0x81, 0xe5, 0xe0, 0xcb, 0x19, 0x9a, 0x48, 0xf8, 0xd3, 0x7a, 0x28, 0x06,
            0x26, 0x4c
        ])
    );
}
