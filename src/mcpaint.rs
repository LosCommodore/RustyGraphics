type Byte = u8;
use anyhow::{Result, bail};
use itertools::Itertools;

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

#[cfg(test)]
mod test {
    use crate::mcpaint::{pack_bytes, unpack_bytes};

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
}
