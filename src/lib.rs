use std::str::FromStr;

#[derive(Debug)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
    pub cost: f64,
}

#[derive(Debug)]
pub struct SteinerInstance {
    pub num_nodes: usize,
    pub num_edges: usize,
    pub num_arcs: usize,
    pub num_obstacles: usize,
    pub num_terminals: usize,
    pub edges: Vec<Edge>,
    pub arcs: Vec<Edge>,
    pub terminals: Vec<usize>,
}

impl Default for SteinerInstance {
    fn default() -> Self {
        Self {
            num_nodes: 0,
            num_edges: 0,
            num_arcs: 0,
            num_obstacles: 0,
            num_terminals: 0,
            edges: Vec::new(),
            arcs: Vec::new(),
            terminals: Vec::new(),
        }
    }
}

#[derive(PartialEq)]
enum Section {
    Start,
    Comment,
    Graph,
    Terminals,
    Coordinates,
}

impl ToString for Section {
    fn to_string(&self) -> String {
        match self {
            Section::Start => "Start".to_string(),
            Section::Comment => "Comment".to_string(),
            Section::Graph => "Graph".to_string(),
            Section::Terminals => "Terminals".to_string(),
            Section::Coordinates => "Coordinates".to_string(),
        }
    }
}

impl FromStr for Section {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Start" => Ok(Section::Start),
            "Comment" => Ok(Section::Comment),
            "Graph" => Ok(Section::Graph),
            "Terminals" => Ok(Section::Terminals),
            "Coordinates" => Ok(Section::Coordinates),
            _ => Err(()),
        }
    }
}

pub struct Parser {
    current_section: Section,
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            current_section: Section::Start,
        }
    }
}

// TODO: implement maximum degrees
impl Parser {
    pub fn parse_stp(&mut self, stp: &str) -> SteinerInstance {
        let mut parsed_result = SteinerInstance::default();

        for line in stp.lines() {
            self.parse_stp_line(line.trim(), &mut parsed_result);
        }

        return parsed_result;
    }

    /*
     * Parse the current line and modify the resulting SteinerInstance in place.
     */
    pub fn parse_stp_line(&mut self, line: &str, current_result: &mut SteinerInstance) {
        match self.current_section {
            Section::Start => self.process_start_line(line, current_result),
            Section::Comment => self.process_comment_line(line, current_result),
            Section::Graph => self.process_graph_line(line, current_result),
            Section::Terminals => self.process_terminals_line(line, current_result),
            Section::Coordinates => self.process_coordinates_line(line, current_result),
        }
        self.move_section(line);
    }

    pub fn move_section(&mut self, line: &str) {
        if !line.starts_with("SECTION") {
            return;
        }

        let section_str = line.split(" ").nth(1);
        if section_str.is_none() {
            return;
        }

        match Section::from_str(section_str.unwrap()) {
            Ok(section) => self.current_section = section,
            Err(_) => return,
        }
    }

    pub fn process_start_line(&mut self, line: &str, current_result: &mut SteinerInstance) {
        // TODO: Do something with the information eventually. Skipped for now.
    }

    pub fn process_comment_line(&mut self, line: &str, current_result: &mut SteinerInstance) {
        // TODO: Do something with the information eventually. Skipped for now.
    }

    fn nth_arg<T: FromStr>(&self, line: &str, n: usize) -> Option<T> {
        let mut s = line.split(" ");
        let target = s.nth(n);
        if target.is_none() {
            return None;
        }
        let res = target.unwrap().parse::<T>();
        match res {
            Ok(parsed) => Some(parsed),
            Err(_) => None,
        }
    }

    fn parse_edge(&self, line: &str) -> Option<Edge> {
        let from = match self.nth_arg(line, 1) {
            Some(u) => u,
            None => return None,
        };
        let to = match self.nth_arg(line, 2) {
            Some(v) => v,
            None => return None,
        };
        // NOTE: This is not part of the official specification, but used by many in
        // practice.
        let cost = match self.nth_arg(line, 3) {
            Some(w) => w,
            None => 1.0,
        };
        Some(Edge { from, to, cost })
    }

    pub fn process_graph_line(&mut self, line: &str, current_result: &mut SteinerInstance) {
        let mut s = line.split(" ");
        match s.nth(0) {
            Some("Obstacles") => {
                let num: Option<usize> = self.nth_arg(line, 1);
                if num.is_none() {
                    return;
                }
                current_result.num_obstacles = num.unwrap();
            }
            Some("Nodes") => {
                let num: Option<usize> = self.nth_arg(line, 1);
                if num.is_none() {
                    return;
                }
                current_result.num_nodes = num.unwrap();
            }
            Some("Edges") => {
                let num: Option<usize> = self.nth_arg(line, 1);
                if num.is_none() {
                    return;
                }
                current_result.num_edges = num.unwrap();
            }
            Some("Arcs") => {
                let num: Option<usize> = self.nth_arg(line, 1);
                if num.is_none() {
                    return;
                }
                current_result.num_arcs = num.unwrap();
            }
            Some("E") => {
                let edge = match self.parse_edge(line) {
                    Some(e) => e,
                    None => return,
                };
                current_result.edges.push(edge);
            }
            Some("A") => {
                let arc = match self.parse_edge(line) {
                    Some(a) => a,
                    None => return,
                };
                current_result.arcs.push(arc);
            }
            Some(_) | None => return,
        }
    }

    // TODO: There are many more options in the specification, which are not all implemented
    pub fn process_terminals_line(&mut self, line: &str, current_result: &mut SteinerInstance) {
        let mut s = line.split(" ");
        match s.nth(0) {
            Some("Terminals") => {
                let num: Option<usize> = self.nth_arg(line, 1);
                if num.is_none() {
                    return;
                }
                current_result.num_terminals = num.unwrap();
            }
            Some("T") => {
                let terminal: usize = match self.nth_arg(line, 1) {
                    Some(t) => t,
                    None => return,
                };
                current_result.terminals.push(terminal);
            }
            Some(_) | None => return,
        }
    }

    pub fn process_coordinates_line(&mut self, line: &str, current_result: &mut SteinerInstance) {
        todo!()
    }
}
