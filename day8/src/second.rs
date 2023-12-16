use std::collections::HashMap;
use std::fmt::Display;

pub fn main(file_content: &String) -> u32 {
    let mut network = Network::new();
    let mut lines = file_content.lines();
    let mut sum = 0;
    let deplacements = lines.next().unwrap();
    lines.next();

    for line in lines {
        let mut line = line.split(" = ");
        let from = line.next().unwrap().to_string();
        let to = line.next().unwrap().split(", ");
        let (left, right) = (to.clone().nth(0).unwrap().replace("(", "").replace(")", ""), to.clone().nth(1).unwrap().replace("(", "").replace(")", ""));

        network.add_node(from.clone());
        network.add_node(right.clone());
        network.add_node(left.clone());
        network.add_edge(from.as_str(), left.clone());
        network.add_edge(from.as_str(), right.clone());

        // if from contains a "A" at the end, it's a starting node
        if from.chars().last().unwrap() == 'A' {
            network.add_starting_node(from.clone());
        }
    }

    let mut found = false;

    let deplacements = deplacements.split("").skip(1);

    while !found {
        for deplacement in deplacements.clone() {
            match deplacement {
                "L" => network.go_left(),
                "R" => network.go_right(),
                _ => break
            }

            sum += 1;

            if network.get_current_nodes().iter().all(|node| node.chars().last().unwrap() == 'Z') {
                found = true;
                break;
            }
        }
    }

    sum
}

pub struct Network {
    pub nodes: HashMap<String, Vec<String>>,
    current_nodes: Vec<String>,
}

impl Network {
    pub fn new() -> Network {
        Network {
            nodes: HashMap::new(),
            current_nodes: Vec::new(),
        }
    }

    pub fn add_starting_node(&mut self, node: String) {
        self.current_nodes.push(node)
    }

    pub fn add_node(&mut self, node: String) {
        if self.nodes.contains_key(&node) {
            return;
        }

        self.nodes.insert(node, Vec::new());
    }

    pub fn add_edge(&mut self, from: &str, to: String) {
        self.nodes.get_mut(from).unwrap().push(to);
    }

    pub fn get_node(&self, node: &str) -> Option<&Vec<String>> {
        self.nodes.get(node)
    }

    pub fn get_current_nodes(&self) -> &Vec<String> {
        &self.current_nodes
    }

    pub fn go_left(&mut self) {
        let mut new_current_nodes = Vec::new();

        for current_node in self.current_nodes.clone() {
            new_current_nodes.push(String::from(self.nodes.get(current_node.as_str()).unwrap()[0].to_string()));
        }

        self.current_nodes = new_current_nodes;
    }

    pub fn go_right(&mut self) {
        let mut new_current_nodes = Vec::new();

        for current_node in self.current_nodes.clone() {
            new_current_nodes.push(String::from(self.nodes.get(current_node.as_str()).unwrap()[1].to_string()));
        }

        self.current_nodes = new_current_nodes;
    }
}

impl Display for Network {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        for (node, edges) in self.nodes.iter() {
            result.push_str(format!("{}: ", node).as_str());

            for edge in edges {
                result.push_str(format!("{} ", edge).as_str());
            }

            result.push_str("\n");
        }

        write!(f, "{}", result)
    }
}