#[cfg(test)]
mod tests {
    use steinlib::{Edge, Parser, export, generate_random::generate_random_with_fixed_vc};

    const SAMPLE_STP: &str = r#"
    SECTION Graph
    Nodes 3
    Edges 3
    E 1 2 1
    E 2 3 2
    E 1 3 3
    END

    SECTION Terminals
    Terminals 2
    T 1
    T 3
    END

    EOF  
    "#;

    #[test]
    fn parsed_instance_correctness() {
        let mut parser = Parser::default();
        let parsed = parser.parse_stp(SAMPLE_STP);

        // ✅ Check node count
        assert_eq!(parsed.num_nodes, 3, "Unexpected number of nodes");
        assert_eq!(parsed.num_edges, 3, "Unexpected number of edges");
        assert_eq!(parsed.num_terminals, 2, "Unexpected number of terminals");

        // ✅ Expected edges
        let expected_edges = vec![
            Edge {
                from: 1,
                to: 2,
                cost: 1.0,
            },
            Edge {
                from: 2,
                to: 3,
                cost: 2.0,
            },
            Edge {
                from: 1,
                to: 3,
                cost: 3.0,
            },
        ];

        // Check that every expected edge exists in parsed, ignoring order and using tolerance for f64
        for exp in &expected_edges {
            assert!(
                parsed.edges.iter().any(|e| edge_eq(e, exp)),
                "Missing expected edge: {:?}",
                exp
            );
        }

        // Check that no extra edges exist
        assert_eq!(
            parsed.edges.len(),
            expected_edges.len(),
            "Parsed edges have unexpected extra entries"
        );

        // ✅ Expected terminals (unordered)
        let mut expected_terminals = vec![1, 3];
        expected_terminals.sort();

        let mut parsed_terminals = parsed.terminals.clone();
        parsed_terminals.sort();

        assert_eq!(
            parsed_terminals, expected_terminals,
            "Parsed terminals do not match expected"
        );

        // Test also that the parsed instance is exported correctly
        let expected = SAMPLE_STP
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n");
        let actual = parsed
            .to_string()
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n");
        assert_eq!(actual, expected);
    }

    #[test]
    fn generatore_test() {
        let (steiner, vc) = generate_random_with_fixed_vc(8, 3, 3);
        println!("{}", steiner.to_string());
        println!("{:?}", vc);
        assert!(true);
    }

    /// Helper for fuzzy float comparison in edges
    fn edge_eq(a: &Edge, b: &Edge) -> bool {
        a.from == b.from && a.to == b.to && (a.cost - b.cost).abs() < 1e-9
    }
}
