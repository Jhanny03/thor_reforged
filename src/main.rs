mod input;
mod network;
mod errors;
mod roll_up;
mod analyses;
mod util;

use std::collections::HashSet;
use std::error::Error;
use crate::analyses::{Analysis};
use crate::analyses::criticality::{Criticality};
use crate::network::Graph;
use crate::roll_up::OrRule;
use std::time::{Instant};
use rand::rngs::StdRng;
use rand::SeedableRng;
use crate::analyses::criticality::loop_condition::MaxLoopCondition;
use crate::analyses::criticality::vis_gen::visibility_states_gen::RandomGen;
use crate::input::{Input, STDCritConfigs, STDCritInput};

fn init(){
    env_logger::init();
}

fn main() -> Result<(), Box<dyn Error>>{
    init();
    //let args: Vec<String> = env::args().collect();
    //dbg!(args);

    let crit_config = STDCritConfigs { in_path: "./sample.csv".to_string()};
    let crit_input = STDCritInput {};
    let (mut graph, _crit_data) = crit_input.read(crit_config)?;

    let l_map = graph.links_map();
    let start_id = Graph::get_start_id(&l_map).unwrap();
    let end_id = Graph::get_end_id(&l_map).unwrap();
    graph.static_nodes.insert(start_id);
    graph.static_nodes.insert(end_id);
    let dynamic_ids: HashSet<u32> = graph.get_node_ids().into_iter()
        .filter(|id| !graph.static_nodes.contains(id))
        .collect();

    let crit = Criticality {
        threads: num_cpus::get() as u8,
        graph,
        dynamic_ids: dynamic_ids.clone(),
        vis_gen: Box::new(
            RandomGen {
                rng: StdRng::from_entropy(),
                ids: dynamic_ids,
                off_chances: Default::default(),
            }
        ),
        loop_condition: Box::new(
            MaxLoopCondition {
                max: 9,
                index: 0 }
        ),
        roll_up_rule: Box::new(
            OrRule {}
        ),
        l_map,
        start_id,
        end_id,
    };
    let start = Instant::now();
    crit.analyze();
    println!("Time elapsed: {:?}", start.elapsed());
    Ok(())
}
