use crate::{Edge, SteinerInstance};
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::visit::Bfs;
use rand::{Rng, rng, seq::index::sample};
use std::collections::HashSet;

/*
* Generate a random Steiner Tree problem instance on `num_vertices` vertices
* with vertex cover of size at most `vc`.
* Returns a tuple of the SteinerInstance and the vertex cover.
*/

pub fn generate_random_with_fixed_vc(
    num_vertices: usize,
    num_terminals: usize,
    vc: usize,
) -> (SteinerInstance, Vec<usize>) {
    let cover = generate_vertex_subset(num_vertices, vc);
    let terminals = generate_vertex_subset(num_vertices, num_terminals);

    let mut is_in_cover = vec![false; num_vertices];
    for v in &cover {
        is_in_cover[v - 1] = true;
    }

    let mut rand_generator = rng();
    let mut edges = Vec::new();

    loop {
        // 1. Add edges using the existing logic
        for i in 1..=num_vertices {
            for j in (i + 1)..=num_vertices {
                if is_in_cover[i - 1] || is_in_cover[j - 1] {
                    let new_edge = Edge {
                        from: i,
                        to: j,
                        cost: 1.0,
                    };
                    if rand_generator.random_bool(0.5) && !edges.contains(&new_edge) {
                        edges.push(new_edge);
                    }
                }
            }
        }

        // 2. Build temporary petgraph to check connectivity
        // We map our 1-indexed edges to 0-indexed NodeIndices
        let pet_edges: Vec<(u32, u32)> = edges
            .iter()
            .map(|e| ((e.from - 1) as u32, (e.to - 1) as u32))
            .collect();

        let g = UnGraph::<(), ()>::from_edges(&pet_edges);

        // 3. Connectivity Check: Can every terminal reach the first terminal?
        if terminals.is_empty() {
            break;
        }

        let mut visited_terminals = HashSet::new();
        let start_node = NodeIndex::new(terminals[0] - 1);
        let mut bfs = Bfs::new(&g, start_node);

        // Traverse all reachable nodes from the first terminal
        while let Some(nx) = bfs.next(&g) {
            let actual_val = nx.index() + 1;
            if terminals.contains(&actual_val) {
                visited_terminals.insert(actual_val);
            }
        }

        // 4. Terminate outer loop only if all terminals were reached
        if visited_terminals.len() == terminals.len() {
            break;
        }

        // If not connected, the loop continues and adds MORE edges to the existing 'edges' Vec
    }

    (SteinerInstance::new(num_vertices, edges, terminals), cover)
}

pub fn generate_vertex_subset(num_vertices: usize, size: usize) -> Vec<usize> {
    sample(&mut rng(), num_vertices, size)
        .into_iter()
        .map(|x| x + 1) // Shift range from 0..n to 1..=n
        .collect()
}
