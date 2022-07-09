use std::fs::File;
use std::io::{BufWriter, Write};

mod roll;
pub use roll::*;

const MIN_WIDTH: i32 = 800;
const MAX_HEIGHT: i32 = 600;

fn color(raw: u32) -> (u8, u8, u8) {
    (
        ((raw >> 16) & 255) as u8,
        ((raw >> 8) & 255) as u8,
        (raw & 255) as u8,
    )
}

#[allow(clippy::unused_io_amount)]
pub fn save(blocks: &[(i32, f32)]) -> std::io::Result<()> {
    let ppm_width: i32 = MIN_WIDTH;
    let ppm_height: i32 = MAX_HEIGHT;
    let max_size = 3 * ppm_width * ppm_height;

    let bheight: i32 = 10;
    let bwidth: i32 = 5;

    let mut buffer = vec![0; max_size as usize];

    // frame
    let bg_color = color(0x001133);
    let fg_color = color(0x102143);
    for y in 0..ppm_height {
        for x in 0..ppm_width {
            let offset = ((y * ppm_width * 3) + (x * 3)) as usize;
            if x % bwidth == 0 || y % bheight == 0 {
                buffer[offset] = fg_color.0;
                buffer[offset + 1] = fg_color.1;
                buffer[offset + 2] = fg_color.2;
            } else {
                buffer[offset] = bg_color.0;
                buffer[offset + 1] = bg_color.1;
                buffer[offset + 2] = bg_color.2;
            }
        }
    }

    // Blocks
    let block_color = color(0x33AA33);
    let mut relx = 0;
    for block in blocks {
        let posy = block.0 as usize * bheight as usize;
        let posx = relx + (block.1 as usize * bwidth as usize) as usize;
        println!("{}: {} {}", posy, relx, block.1);
        for y in posy..posy + bheight as usize {
            for x in relx..posx {
                let offset = ((y * ppm_width as usize * 3) + (x * 3)) as usize;
                buffer[offset] = block_color.0;
                buffer[offset + 1] = block_color.1;
                buffer[offset + 2] = block_color.2;
            }
        }
        relx = posx;
    }

    let mut p = BufWriter::new(File::create("foo.ppm")?);
    p.write(format!("P6 {} {} 255\n", ppm_width, ppm_height).as_bytes())?;
    p.write(&buffer)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color() {
        assert_eq!(color(0xFFFF00), (255, 255, 0));
        assert_eq!(color(0xFF0000), (255, 0, 0));
        assert_eq!(color(0x00FF00), (0, 255, 0));
    }
}
