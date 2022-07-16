#[cfg(feature = "graph")]
use sequencer::graph::{Hits, Roll};

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

    let values: Vec<(f64, f64)> = Sequence::new(volumes)
        .transform(Spaceout::amount(spaceout))
        .transform(Humanize::amount())
        .iter()
        .map(|x| {
            let beats = 1.0 / x.duration().per_beat();
            let bars = beats * 4.0;
            (x.intensity(), bars as f64)
        })
        .collect();
    let mut graph = Hits::new();
    graph.beats(4);
    graph.draw("foo.ppm", &values)?;

    let notes: Vec<(Option<i32>, f32)> = Sequence::new(melody)
        .transform(Spaceout::note(spaceout))
        .transform(Humanize::note())
        .iter()
        .map(|n| {
            let y = n.midi();
            let beats = 1.0 / n.per_beat();
            let bars = beats * 4.0;
            (y, bars)
        })
        .collect();
    let mut roll = Roll::new();
    roll.beats(4);
    roll.draw("foo1.ppm", &notes)?;

    Ok(())
}
