type Byte = u8;
use anyhow::{Result, bail};
use itertools::Itertools;

#[allow(unused)]
pub fn pack_bytes(source: &[Byte]) -> Result<Vec<Byte>> {
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

#[cfg(test)]
mod test {
    use crate::mcpaint::pack_bytes;

    #[test]
    fn test_pack() {
        let out = pack_bytes(&[
            255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 0, 0, 0, 0, 0, 0, 255, 255,
        ])
        .unwrap();
        println!("{out:?}");

        assert_eq!(out, vec![1, 0, 255, 3, 3])
    }
}
