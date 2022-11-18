mod gui;
mod hypergraph;
mod tui;

// use gui::start; // imports gui
// use tui::start;

use crate::hypergraph::{Hypergraph};

fn main() {
    let hypergraph = Hypergraph::from_collection_form("{ { 1, 2, 3 }, { 1, 4 } }".to_string());
    dbg!(hypergraph.clone());
    println!("{}", hypergraph.to_collection_form());
}
