#[cfg(feature = "graph")]
use graph::ppm::Renderer; 
#[cfg(feature = "graph")]
use graph::writer::{FileWriter, Writer};
#[cfg(feature = "graph")]
use graph::{Block, Graph, Roll};

use note::*;
use sequencer::*;

fn get_blocks() -> Vec<Block> {
    Sequence::new(vec![
        note![A: C0, 1 / 4],
        // pause![1 / 14],
        note![C: C1, 1 / 4 T],
        // pause![1 / 14],
        note![A: C0, 1 / 8],
        // pause![1 / 14],
        note![B: C0, 1 / 8 T],
    ])
    .transform(Humanize::note())
    .iter()
    .map(|n| {
        let y = n.midi();
        let beats = 1.0 / n.per_beat();
        let bars = beats * 4.0;
        // println!("{:?}: pitch: {y:?}, duration: {bars}", n);
        if let Some(y) = y {
            return Block::new(bars.into(), y.into());
        } else {
            return Block::new(bars.into(), 20.0);
        }
    })
    .collect()
}

#[cfg(feature = "graph")]
fn main() -> std::io::Result<()> {
    let blocks = get_blocks();

    let roll = Roll::new(&blocks);
    let w = FileWriter::new("foo.ppm");
    let renderer = Renderer::new(&roll.size());
    w.write(renderer, roll)?;

    eprintln!(
        "notes: {}",
        blocks
            .iter()
            .map(|x| {
                if x.intensity().is_some() {
                    1
                } else {
                    0
                }
            })
            .sum::<i32>()
    );
    eprintln!(
        "play time: {}",
        blocks
            .iter()
            .map(|x| {
                if x.intensity().is_some() {
                    *x.duration()
                } else {
                    0.0
                }
            })
            .sum::<f64>()
    );
    eprintln!(
        "pauses: {}",
        blocks
            .iter()
            .map(|x| {
                if x.intensity().is_none() {
                    1
                } else {
                    0
                }
            })
            .sum::<i32>()
    );
    eprintln!(
        "pause time: {}",
        blocks
            .iter()
            .map(|x| {
                if x.intensity().is_none() {
                    *x.duration()
                } else {
                    0.0
                }
            })
            .sum::<f64>()
    );
    eprintln!("total: {}", blocks.iter().map(|x| x.duration()).sum::<f64>());

    Ok(())
}
