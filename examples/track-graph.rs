#[cfg(feature = "graph")]
use graph::svg::Renderer;
#[cfg(feature = "graph")]
use graph::writer::{FileWriter, Writer};
#[cfg(feature = "graph")]
use graph::{Block, Graph, Track};

use note::*;
use sequencer::{Amount, Humanize, Sequence, Spaceout};

#[cfg(feature = "graph")]
fn main() -> std::io::Result<()> {
    let spaceout = val![1 / 4];

    let mut volumes = Vec::new();
    let melody = vec![
        note![A: C0, 1 / 4],
        note![C: C1, 1 / 4 T],
        note![A: C0, 1 / 8],
        note![B: C0, 1 / 8 T],
    ];
    for note in &melody {
        let value = match note {
            Note::Tone(_, _, v) => v,
            Note::Rest(v) => v,
        };
        volumes.push(Amount::at(1.0, *value));
    }

    let values: Vec<Block> = Sequence::new(volumes)
        .transform(Spaceout::amount(spaceout))
        .transform(Humanize::amount())
        .iter()
        .map(|x| {
            let beats = 1.0 / x.duration().per_beat();
            let bars = beats * 4.0;
            Block::new(bars as f64, x.intensity())
        })
        .collect();

    let notes: Vec<Block> = Sequence::new(melody)
        .transform(Spaceout::note(spaceout))
        .transform(Humanize::note())
        .iter()
        .map(|n| {
            let y = n.midi();
            let beats = 1.0 / n.per_beat();
            let bars = beats * 4.0;
            if let Some(y) = y {
                return Block::new(bars.into(), y.into());
            } else {
                return Block::new(bars.into(), 00.0);
            }
        })
        .collect();

    let track = Track::new(&values, &notes);
    let w = FileWriter::new("foo.svg");
    let renderer = Renderer::new(&track.size());
    w.write(renderer, track)?;

    Ok(())
}
