#[cfg(feature = "graph")]
use sequencer::graph::Roll;

use note::*;
use sequencer::*;

fn get_blocks() -> Vec<(Option<i32>, f32)> {
    Sequence::new(vec![
        note![A: C0, 1 / 4],
        pause![1 / 14],
        note![C: C1, 1 / 4 T],
        pause![1 / 14],
        note![A: C0, 1 / 8],
        pause![1 / 14],
        note![B: C0, 1 / 8 T],
    ])
    .transform(Humanize::note())
    .iter()
    .map(|n| {
        let y = n.midi();
        let beats = 1.0 / n.per_beat();
        let bars = beats * 4.0;
        (y, bars)
    })
    .collect()
}

#[cfg(feature = "graph")]
fn main() -> std::io::Result<()> {
    let blocks = get_blocks();

    let mut roll = Roll::new();
    roll.beats(4);
    roll.draw("foo.ppm", &blocks)?;

    eprintln!(
        "notes: {}",
        blocks
            .iter()
            .map(|x| {
                if x.0.is_some() {
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
                if x.0.is_some() {
                    x.1
                } else {
                    0.0
                }
            })
            .sum::<f32>()
    );
    eprintln!(
        "pauses: {}",
        blocks
            .iter()
            .map(|x| {
                if x.0.is_none() {
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
                if x.0.is_none() {
                    x.1
                } else {
                    0.0
                }
            })
            .sum::<f32>()
    );
    eprintln!("total: {}", blocks.iter().map(|x| x.1).sum::<f32>());

    Ok(())
}
