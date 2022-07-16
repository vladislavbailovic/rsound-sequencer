#[cfg(feature = "graph")]
use sequencer::graph::Hits;

use note::*;
use sequencer::{Sequence, Trigger, Volume};

#[cfg(feature = "graph")]
fn main() -> std::io::Result<()> {
    let values: Vec<(f64, f64)> = Sequence::new(vec![
        Volume::at(0.75, val![1 / 4]),
        Volume::zero(val![1 / 14]),
        Volume::new(val![1/4 T]),
        Volume::zero(val![1 / 14]),
        Volume::at(0.5, val![1 / 8]),
        Volume::zero(val![1 / 14]),
        Volume::new(val![1/8 T]),
    ])
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

    Ok(())
}
