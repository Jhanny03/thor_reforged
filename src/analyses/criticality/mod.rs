use std::collections::{HashMap, HashSet};
use std::ops::Index;
use log::info;
use crate::analyses::{Analysis, VISIBLE_VAL};
use crate::network::{Graph, LinkMap, GraphNodeState};
use crate::roll_up::RollUp;
use std::sync::mpsc;
use std::thread;
use crate::analyses::criticality::loop_condition::CritLoopCondition;
use crate::analyses::criticality::vis_gen::visibility_states_gen::VisGen;

pub mod loop_condition;
pub mod vis_gen;

pub struct CriticalityData {
}

pub struct Criticality {
    pub threads: u8,
    pub graph: Graph,
    pub dynamic_ids: HashSet<u32>,
    pub vis_gen: Box<dyn VisGen>,
    pub loop_condition: Box<dyn CritLoopCondition>,
    pub roll_up_rule: Box<dyn RollUp>,
    pub l_map: LinkMap,
    pub start_id: u32,
    pub end_id: u32
}

impl Analysis for Criticality {
    fn analyze(self) {
        info!("Starting Criticality Analysis");
        let path = Graph::get_bfs_path(&self.l_map, self.start_id);

        let (tx1, rx) = mpsc::channel();

        let mut loop_conditions = self.loop_condition.split_to_threads(self.threads as u64);
        let mut vis_gens = self.vis_gen.split_to_threads(self.threads as u64);
        let mut senders = vec![];
        for _ in 0..self.threads -1 {
            senders.push(tx1.clone());
        }
        senders.push(tx1);
        for _ in 0..self.threads {
            let tx = senders.pop().unwrap();

            let graph = self.graph.clone();
            let loop_condition = loop_conditions.pop().unwrap();
            let vis_gen = vis_gens.pop().unwrap();
            let roll_up_rule = dyn_clone::clone_box(&*self.roll_up_rule);
            let l_map = self.l_map.clone();
            let new_path = path.clone();
            let dynamic_ids = self.dynamic_ids.clone();
            let end_id = self.end_id;

            thread::spawn(move || {
                let data = Criticality::calculate_data(
                    graph,
                    vis_gen,
                    loop_condition,
                    roll_up_rule,
                    l_map,
                    new_path,
                    dynamic_ids,
                    end_id
                );
                tx.send(data).unwrap();
            });
        }

        for received in rx {
            println!("Got {:?}", received);
        }
    }
}

impl Criticality {
    fn calculate_data(graph: Graph,
                      mut states_generator: Box<dyn VisGen>,
                      mut loop_condition: Box<dyn CritLoopCondition>,
                      roll_up_rule: Box<dyn RollUp>,
                      l_map: LinkMap,
                      path: Vec<u32>,
                      dynamic_ids: HashSet<u32>,
                      end_id: u32
    ) -> GraphCritData
    {
        let mut data = GraphCritData {
            row_count: 0,
            end_op_sum: 0.0,
            node_data: HashMap::new(),
        };

        for n_id in dynamic_ids.iter() {
            data.node_data.insert(*n_id, NodeCritData {
                sum_end_on: 0.0,
                sum_end_off: 0.0,
            });
        }

        let mut visited: HashSet<GraphNodeState<u8>> = HashSet::new();

        while !loop_condition.stop() {
            let visibility_state = states_generator.next_states();
            if visited.contains(&visibility_state) {
                continue
            }
            let result = graph.roll_up_state(&path, &l_map, &roll_up_rule, &visibility_state);
            let end_val = result.get(&end_id).unwrap();
            let mut node_data = HashMap::new();
            for id in dynamic_ids.iter() {
                let visible = match visibility_state.get(&id) {
                    None => { true }
                    Some(x) => { *x == VISIBLE_VAL }
                };
                let state_val = result.get(&id).unwrap();
                let mut sum_end_on = 0.0;
                let mut sum_end_off = 0.0;
                match visible {
                    true => {
                        sum_end_on += state_val;
                    }
                    false => {
                        sum_end_off += state_val;
                    }
                }
                node_data.insert(*id, NodeCritData {
                    sum_end_on: sum_end_on as f64,
                    sum_end_off: sum_end_off as f64,
                });
            }
            let new_data = GraphCritData {
                row_count: 1,
                end_op_sum: *end_val as f64,
                node_data,
            };
            data.add(&new_data);
            visited.insert(visibility_state);
        }
        data
    }
}

#[derive(Debug)]
struct GraphCritData {
    row_count: u64,
    end_op_sum: f64,
    node_data: HashMap<u32, NodeCritData>
}

impl GraphCritData {
    pub fn add(&mut self, d2: &GraphCritData){
        self.row_count += d2.row_count;
        self.end_op_sum += d2.end_op_sum;
        for (id, crit_data) in self.node_data.iter_mut() {
            crit_data.add(d2.node_data.index(id));
        }
    }
}

#[derive(Debug)]
struct NodeCritData {
    sum_end_on: f64,
    sum_end_off: f64,
}

impl NodeCritData {
    pub fn add(&mut self, d2: &NodeCritData){
        self.sum_end_on += d2.sum_end_on;
        self.sum_end_off += d2.sum_end_off;
    }
}