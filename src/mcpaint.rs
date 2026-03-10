type Byte = u8;
use anyhow::{Result, bail};
use itertools::Itertools;

#[allow(unused)]
pub fn compress(source: &[Byte]) -> Result<Vec<Byte>> {
    if source.len() > 128 {
        bail!("Conpression only valid for Slices of length <=255")
    }

    let mut dest = Vec::new();

    if source.len() == 0 {
        return Ok(dest);
    }

    enum State {
        Same(u8), // (repeating value, index of value)
        Diff(u8), // (last_value, index of first change)
        Init,     // initialize
    }
    impl State {
        fn flush(&self, slice: &[Byte], dest: &mut Vec<u8>) {
            let len = slice.len();
            match self {
                State::Diff(_) => {
                    dest.push((len - 1) as u8);
                    dest.extend_from_slice(slice);
                }
                State::Same(same) => {
                    dest.push((257 - len) as u8); // encoding of number of repetitions
                    dest.push(*same);
                }
                State::Init => (),
            }
        }
    }
    let mut last_i = 0usize;
    let mut state = State::Init;

    for (i, current) in source.iter().enumerate() {
        state = match state {
            State::Same(same) => {
                if same == *current {
                    state
                } else {
                    state.flush(&source[last_i..i], &mut dest);
                    last_i = i;
                    State::Diff(*current)
                }
            }
            State::Diff(last) => {
                if last != *current {
                    State::Diff(*current)
                } else {
                    if (i - last_i) > 1 {
                        state.flush(&source[last_i..i - 1], &mut dest);
                    }
                    last_i = i - 1;
                    State::Same(*current)
                }
            }
            State::Init => State::Diff(*current),
        }
    }
    state.flush(&source[last_i..], &mut dest);
    Ok(dest)
}

#[allow(unused)]
pub fn pack_bytes(source: &[Byte]) -> Result<Vec<Byte>> {
    if source.len() % 8 != 0 {
        bail!("Number of Pixels must be divisible by 8")
    }

    let capacity = source.len() / 8 + 1;
    let mut dest = Vec::with_capacity(capacity);

    for chunk in &source.into_iter().chunks(8) {
        let mut out_byte = 0u8;
        for (i, byte) in chunk.enumerate() {
            match byte {
                255 => out_byte += 1 << i,
                0 => (),
                _ => bail!("Only 0 or 255 allowed"),
            }
        }
        dest.push(out_byte);
    }
    Ok(dest)
}

#[allow(unused)]
pub fn unpack_bytes(source: &[Byte]) -> Vec<Byte> {
    let capacity = source.len() * 8;
    let mut dest = Vec::with_capacity(capacity);

    for byte in source {
        for i in 0..8 {
            let out_byte = ((byte >> i) & 1) * 255;
            dest.push(out_byte);
        }
    }
    dest
}

#[allow(unused)]
pub fn pack_compress(data: &[u8]) -> Result<Vec<u8>> {
    let p = pack_bytes(data)?;
    compress(&p)
}

#[cfg(test)]
mod test {
    use crate::mcpaint::{compress, pack_bytes, unpack_bytes};

    #[test]
    fn test_pack() {
        let out = pack_bytes(&[
            255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 0, 0, 0, 0, 0, 0,
        ])
        .unwrap();
        println!("{out:?}");

        assert_eq!(out, vec![1, 0, 255, 3])
    }

    #[test]
    fn test_unpack() {
        let out = unpack_bytes(&[1, 2, 3, 4]);
        for o in out.chunks(8) {
            println!("{o:?}")
        }

        //assert_eq!(out, vec![1, 0, 255, 3, 3])
    }

    #[test]
    fn test_pack_unpack() {
        let data_in = &[
            255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 0, 0, 0, 0, 0, 0,
        ];

        let step_1 = pack_bytes(data_in).unwrap();
        let out = unpack_bytes(&step_1);
        let dat_in_vec = Vec::from(data_in);
        assert_eq!(dat_in_vec, out, "not equal");
    }

    #[test]
    fn test_compress_different() {
        let data_in = [1, 2, 3, 4, 5];
        let out = compress(&data_in).unwrap();
        println!("{:?}", out);
        assert_eq!(out, [4, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_compress_same() {
        let data_in = [5, 5, 5, 5, 5];
        let out = compress(&data_in).unwrap();
        println!("{:?}", out);
        assert_eq!(out, [252, 5]);
    }

    #[test]
    fn test_compress_example() {
        let data_in = [
            170, 170, 170, 128, 0, 42, 170, 170, 170, 170, 128, 0, 42, 34, 170, 170, 170, 170, 170,
            170, 170, 170, 170, 170, 99,
        ];
        let out = compress(&data_in).unwrap();
        println!("{:?}", out);
        assert_eq!(
            out,
            [
                254, 170, 2, 128, 0, 42, 253, 170, 3, 128, 0, 42, 34, 247, 170, 0, 99
            ]
        );
    }
}
