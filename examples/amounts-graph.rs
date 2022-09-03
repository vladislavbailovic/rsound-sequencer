#[cfg(feature = "graph")]
use graph::ppm::Renderer;
#[cfg(feature = "graph")]
use graph::writer::{FileWriter, Writer};
#[cfg(feature = "graph")]
use graph::{Block, Graph, Hits};

use note::*;
use sequencer::{Amount, Humanize, Sequence};

#[cfg(feature = "graph")]
fn main() -> std::io::Result<()> {
    let values: Vec<Block> = Sequence::new(vec![
        Amount::at(0.75, val![1 / 4]),
        Amount::zero(val![1 / 14]),
        Amount::new(val![1/4 T]),
        Amount::zero(val![1 / 14]),
        Amount::at(0.5, val![1 / 8]),
        Amount::zero(val![1 / 14]),
        Amount::new(val![1/8 T]),
    ])
    .transform(Humanize::amount())
    .iter()
    .map(|x| {
        let beats = 1.0 / x.duration().per_beat();
        let bars = beats * 4.0;
        return Block::new(bars as f64, x.intensity() * 10.0)
    })
    .collect();

    let hits = Hits::new(&values);
    let w = FileWriter::new("foo.ppm");
    let renderer = Renderer::new(&hits.size());
    w.write(renderer, hits)?;

    Ok(())
}
