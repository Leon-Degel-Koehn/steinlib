// Add capabilities to export instances to files again
// For now, only the most important artifacts are exported
// such as Graph and Terminals.

use crate::{Edge, Section, SteinerInstance};
use std::fmt::Write;

impl ToString for SteinerInstance {
    fn to_string(&self) -> String {
        let mut output = String::new();

        // Export Graph section
        let _ = writeln!(&mut output, "SECTION Graph");
        //  write num nodes
        let _ = writeln!(&mut output, "Nodes {}", self.num_nodes);
        //  write num edges
        let _ = writeln!(&mut output, "Edges {}", self.num_edges);
        //  write every edge
        for edge in &self.edges {
            let _ = writeln!(&mut output, "E {} {} {}", edge.from, edge.to, edge.cost);
        }
        let _ = writeln!(&mut output, "END");
        let _ = writeln!(&mut output, "");

        // Export Terminals Section
        let _ = writeln!(&mut output, "SECTION Terminals");
        let _ = writeln!(&mut output, "Terminals {}", self.num_terminals);
        for terminal in &self.terminals {
            let _ = writeln!(&mut output, "T {}", terminal);
        }
        let _ = writeln!(&mut output, "END");
        let _ = writeln!(&mut output, "");
        let _ = writeln!(&mut output, "EOF");

        return output;
    }
}
