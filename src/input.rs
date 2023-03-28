use std::fs::File;
use csv;
use crate::network::Graph;

pub fn read_csv(path: &str) -> Vec<Vec<String>> {
    let file = File::open(path).unwrap();
    // open the file
    let mut reader = csv::ReaderBuilder::new()
        // create the reader
        .has_headers(false)
        // no need to read the header
        .from_reader(file);
    let mut rows = Vec::new();

    for result in reader.records() {
        // read the csv
        let record = result.unwrap();

        // convert the record to a vector
        let record = record.into_iter().map(|x| x.to_string()).collect();

        rows.push(record);

    }
    rows
}

pub fn csv_to_graph(csv: &Vec<Vec<String>>) -> Result<Graph, Vec<String>> {
    let mut graph = Graph::new();
    let mut errors = vec![];

    for (y, row) in csv.into_iter().enumerate() {
        let c_name = get_string_cell(&row, 0, y, &mut errors);
        let p_name = get_string_cell(&row, 2, y, &mut errors);
        let c_id = get_u32_cell(&row, 1, y, &mut errors);
        let p_id = get_u32_cell(&row, 3, y, &mut errors);
        graph.add_node(c_name, c_id);
        graph.add_node(p_name, p_id);
        graph.add_edge(c_id, p_id);
    }

    if errors.len() > 0 {
        Err(errors)
    } else {
        Ok(graph)
    }
}

fn get_string_cell(row: &Vec<String>, x: usize, y: usize, errors: &mut Vec<String>) -> String {
    match row.get(x) {
        Some(x) => {
            x.trim().to_string()
        }
        None => {
            errors.push(String::from(format!("Row {} does not have index {}", y, x)));
            "name_not_set".to_string()
        }
    }
}

fn get_u32_cell(row: &Vec<String>, x: usize, y: usize, errors: &mut Vec<String>) -> u32 {
    let string_val = get_string_cell(row, x, y, errors);
    string_val.parse::<u32>().unwrap_or_else(|e| {
        println!("{}", e);
        errors.push(String::from(format!("Cell ({}, {}) should be u32", y, x)));
        0
    })
}