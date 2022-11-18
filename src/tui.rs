use cursive::Cursive;
use cursive::views::{TextView};
use cursive::traits::*;

use crate::hypergraph::{Hypergraph};

pub fn display_hypergraph(hypergraph: Hypergraph) {
    let mut siv = cursive::default();

    siv.add_global_callback('q', |s| s.quit());

    // let hypergraphText = TextView::new(content)
}