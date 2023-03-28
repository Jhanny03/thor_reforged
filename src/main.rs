mod input;
mod network;


use crate::input::*;


fn main() {
    env_logger::init();
    let csv = read_csv("./sample.csv");
    println!("{:?}", csv);
    let graph = csv_to_graph(&csv);
    match graph {
        Ok(graph) => {
            println!("{:?}", graph);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
