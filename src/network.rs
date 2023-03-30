use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::ops::Index;
use crate::errors::network::{EndNodeError, NoEndConnectionError, StartNodeError};
use crate::roll_up::RollUp;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Node {
    pub name: String,
    pub id: u32,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Edge {
    pub from: u32,
    pub to: u32,
}

pub type NodeStates<D> = BTreeMap<u32, D>;

#[derive(Debug, Clone)]
pub struct Graph {
    nodes: HashMap<u32, Node>,
    edges: HashSet<Edge>,
    pub static_nodes: HashSet<u32>
}

pub type LinkMap = HashMap<u32, (Vec<u32>, Vec<u32>)>;

#[allow(dead_code)]
impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: HashMap::new(),
            edges: HashSet::new(),
            static_nodes: HashSet::new()
        }
    }

    pub fn add_node(&mut self, name: String, id: u32) -> Option<Node> {
        self.nodes.insert(id, Node { name, id })
    }

    pub fn get_node(&self, id: &u32) -> Option<&Node> {
        return self.nodes.get(id)
    }

    pub fn remove_node(&mut self, id: &u32) -> Option<Node> {
        return self.nodes.remove(id);
    }

    pub fn get_node_ids(&self) -> HashSet<u32> {
        let mut ids: HashSet<u32> = HashSet::new();
        for node in &self.nodes {
            ids.insert(*node.0);
        }
        ids
    }

    pub fn add_edge(&mut self, from: u32, to: u32) -> bool {
        self.edges.insert(Edge { from, to })
    }

    pub fn get_edge(&self, from: u32, to: u32) -> Option<&Edge> {
        return self.edges.get( &Edge { from, to})
    }

    pub fn remove_edge(&mut self, from: u32, to: u32) -> bool {
        return self.edges.remove( &Edge { from, to})
    }

    pub fn links_map(&self) -> LinkMap {
         let mut map: LinkMap = HashMap::new();
        for edge in self.edges.iter() {
            if !map.contains_key(&edge.from) {
                map.insert(edge.from, (vec![], vec![]));
            }
            if !map.contains_key(&edge.to) {
                map.insert(edge.to, (vec![], vec![]));
            }
            map.get_mut(&edge.from).unwrap().1.push(edge.to);
            map.get_mut(&edge.to).unwrap().0.push(edge.from);
        }
        map
    }

    pub fn roll_up_state(&self,
                         graph_path: &Vec<u32>,
                         l_map: &LinkMap,
                         roll_up_rule: &Box<dyn RollUp>,
                         visibilities: &NodeStates<u8>)
        -> NodeStates<f32>
    {
        let mut new_state = NodeStates::new();
        for node in graph_path {
            let children = &l_map.get(node).unwrap().0;
            new_state.insert(*node, roll_up_rule.get_value(node, children, visibilities, &new_state));
        }
        new_state
    }

    pub fn deep_clone(&self) -> Self {
        let mut clone = Graph::new();
        for node in &self.nodes {
            clone.add_node(node.1.name.to_string(), node.1.id);
        }
        for edge in &self.edges {
            clone.add_edge(edge.from, edge.to);
        }
        clone
    }

    pub fn get_start_id(map: &LinkMap) -> Result<u32, StartNodeError>{
        let mut starts: Vec<u32> = vec![];
        for node in map {
            if node.1.0.is_empty() {
                starts.push(*node.0);
            }
        }
        if starts.len() == 1 {
            return Ok(*starts.index(0))
        }
        Err(StartNodeError { starts })
    }

    pub fn get_end_id(map: &LinkMap) -> Result<u32, EndNodeError>{
        let mut ends: Vec<u32> = vec![];
        for node in map {
            if node.1.1.is_empty() {
                ends.push(*node.0);
            }
        }
        if ends.len() == 1 {
            return Ok(*ends.index(0))
        }
        Err(EndNodeError { ends })
    }

    pub fn get_bfs_path(map: &LinkMap, start_id: u32) -> Vec<u32>{
        let mut path: Vec<u32> = vec![];
        let mut visited: HashSet<u32> = HashSet::from([start_id]);
        let mut agenda: VecDeque<u32> = VecDeque::from([start_id]);
        while !agenda.is_empty() {
            let current = agenda.pop_front().unwrap();
            path.push(current);
            if !map.contains_key(&current){ continue; }
            for parent in &map.index(&current).1 {
                if !visited.contains(parent) {
                    agenda.push_back(*parent);
                    visited.insert(*parent);
                }
            }
        }
        path
    }

    pub fn validate_end_connection(map: &LinkMap, start_id: u32, end_id: u32)
        -> Result<Vec<u32>, NoEndConnectionError> {
        if start_id == end_id {
            return Ok(Vec::from([start_id]))
        }
        let path = Graph::get_bfs_path(map, start_id);
        if path.contains(&end_id){
            return Ok(path)
        }
        Err(NoEndConnectionError { start_id, end_id })
    }
}