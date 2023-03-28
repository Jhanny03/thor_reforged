#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub from: u32,
    pub to: u32,
}

#[derive(Debug, Clone)]
pub struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: vec![],
            edges: vec![],
        }
    }

    pub fn add_node(&mut self, name: String, id: u32) {
        if !self.contains_node(id) {
            self.nodes.push(Node { name, id });
        }
    }

    pub fn add_edge(&mut self, from: u32, to: u32) {
        self.edges.push(Edge { from, to });
    }

    pub fn get_node(&self, id: u32) -> Option<&Node> {
        return self.nodes.iter().find(|&n| n.id == id);
    }

    pub fn contains_node(&self, id: u32) -> bool {
        self.nodes.iter().any(|node| node.id == id)
    }
}