use crate::{Edge, SteinerInstance};
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::visit::Bfs;
use rand::distr::Distribution;
use rand::distr::weighted::WeightedIndex;
use rand::random_bool;
use rand::seq::IndexedRandom;
use rand::{Rng, rng, seq::index::sample};
use std::collections::HashSet;
use std::fmt::Write;
use std::fs;
use std::io::BufWriter;
use std::path::PathBuf;

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

#[derive(Debug)]
pub struct UpdateProbabilities {
    pub edge_insertion: f32,
    pub edge_deletion: f32,
    pub terminal_activation: f32,
    pub terminal_deactivation: f32,
}

#[derive(Debug)]
pub enum UpdateOperation {
    EdgeInsertion(Edge),
    EdgeDeletion(Edge),
    TerminalActivation(usize),
    TerminalDeactivation(usize),
    Query(SteinerInstance),
}

impl ToString for UpdateOperation {
    fn to_string(&self) -> String {
        match self {
            UpdateOperation::Query(steiner_instance) => steiner_instance.to_string(),
            Self::EdgeInsertion(edge) => format!("E I {} {} {}", edge.from, edge.to, edge.cost),
            Self::EdgeDeletion(edge) => format!("E D {} {} {}", edge.from, edge.to, edge.cost),
            Self::TerminalActivation(vertex) => format!("T A {}", vertex),
            Self::TerminalDeactivation(vertex) => format!("T D {}", vertex),
        }
    }
}

pub fn generate_update_sequence(
    instance: &SteinerInstance,
    update_probs: UpdateProbabilities,
    query_prob: f64,
) -> Vec<UpdateOperation> {
    let mut updates = Vec::new();
    let mut rng = rng();

    let mut current_edges: Vec<Edge> = Vec::new();
    let mut current_terminals: Vec<usize> = Vec::new();

    let weights = [
        update_probs.edge_insertion,
        update_probs.edge_deletion,
        update_probs.terminal_activation,
        update_probs.terminal_deactivation,
    ];

    let dist = WeightedIndex::new(&weights).expect("Invalid probabilities");

    loop {
        let edges_full = current_edges.len() == instance.edges.len();
        let terminals_full = current_terminals.len() == instance.terminals.len();

        if edges_full && terminals_full {
            break;
        }

        // 1. Choose either terminal or edge update
        // 2. choose between insert/activate or delete/deactivate
        // 3. choose legal target of operation (edge or vertex)

        // 1 = terminal update, 0 = edge update
        let choice = dist.sample(&mut rng);
        if choice == 0 && edges_full
            || choice == 1 && current_edges.len() == 0
            || choice == 2 && terminals_full
            || choice == 3 && current_terminals.len() == 0
        {
            continue;
        }

        // terminal update
        if choice == 2 || choice == 3 {
            let is_activation = choice == 2;
            let available_vertices: Vec<usize> = instance
                .terminals
                .clone()
                .into_iter()
                .filter(|i| is_activation ^ current_terminals.contains(i))
                .collect();
            if available_vertices.len() == 0 {
                continue;
            }
            if is_activation {
                let target = *available_vertices.choose(&mut rng).unwrap();
                updates.push(UpdateOperation::TerminalActivation(target));
                current_terminals.push(target);
            } else {
                let target = *available_vertices.choose(&mut rng).unwrap();
                updates.push(UpdateOperation::TerminalDeactivation(target));
                current_terminals.retain(|&x| x != target);
            }
        }

        // edge update
        if choice == 0 || choice == 1 {
            let is_insertion = choice == 0;
            let available_edges: Vec<Edge> = instance
                .edges
                .clone()
                .into_iter()
                .filter(|i| is_insertion ^ current_edges.contains(i))
                .collect();
            if available_edges.len() == 0 {
                continue;
            }
            if is_insertion {
                let target = available_edges.choose(&mut rng).unwrap().clone();
                updates.push(UpdateOperation::EdgeInsertion(target.clone()));
                current_edges.push(target.clone());
            } else {
                let target = available_edges.choose(&mut rng).unwrap().clone();
                updates.push(UpdateOperation::EdgeDeletion(target.clone()));
                current_edges.retain(|x| *x != target);
            }
        }

        let do_query = random_bool(query_prob);
        if do_query {
            updates.push(UpdateOperation::Query(SteinerInstance::new(
                instance.num_nodes,
                current_edges.clone(),
                current_terminals.clone(),
            )));
        }
    }

    // Ensure that any full sequence ends with a query
    if !matches!(updates.last().unwrap(), UpdateOperation::Query(_)) {
        updates.push(UpdateOperation::Query(SteinerInstance::new(
            instance.num_nodes,
            current_edges.clone(),
            current_terminals.clone(),
        )));
    }

    return updates;
}

pub fn generate_vertex_subset(num_vertices: usize, size: usize) -> Vec<usize> {
    sample(&mut rng(), num_vertices, size)
        .into_iter()
        .map(|x| x + 1) // Shift range from 0..n to 1..=n
        .collect()
}

pub fn export_update_sequence(updates: Vec<UpdateOperation>) -> (String, Vec<String>) {
    let mut main_output = String::new();
    let mut query_instances = Vec::new();
    let _ = writeln!(main_output, "SECTION UPDATES");
    let mut query_no = 1;

    for update in updates {
        match update {
            UpdateOperation::Query(_) => {
                let _ = writeln!(main_output, "Q {}", query_no);
                let _ = query_instances.push(update.to_string());
                query_no += 1;
            }
            _ => {
                let _ = writeln!(main_output, "{}", update.to_string());
            }
        }
    }

    return (main_output, query_instances);
}

pub fn output_update_sequence(
    updates: Vec<UpdateOperation>,
    directory: String,
) -> std::io::Result<()> {
    let (main_output, query_instances) = export_update_sequence(updates);
    let path = PathBuf::from(directory);
    let mut main_path = path.clone();
    main_path.push("updates.dus");
    fs::write(main_path, main_output)?;

    let mut query_no = 1;
    for query_instance in query_instances {
        let mut query_path = path.clone();
        query_path.push(format!("instance_{}.gr", query_no));
        fs::write(query_path, query_instance)?;
        query_no += 1;
    }

    Ok(())
}
