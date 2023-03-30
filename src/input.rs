use std::error::Error;
use std::fs::File;
use csv;
use crate::network::{Graph};
use crate::{errors};
use crate::analyses::criticality::CriticalityData;

use crate::errors::input::{CellNotNumericError, GraphCreationError};

/// Module containing all necessary structures for reading inputs / data necessary for analyses.
///
/// Reading from any file is done using a struct that implements the ['Input'] trait.
/// All input structures must provide some way to build a graph, as well as some type of
/// additional information for some analysis. Note that each analysis requires it's own input
/// implementation. Most input structures will likely share similar code.

/// List of rows (list of strings) obtained from reading a file.
pub type StringMatrix = Vec<Vec<String>>;

/// Node name used if the input fails to provide a proper name
const DEFAULT_NODE_NAME: &str = "DEFAULT";
/// Node id used if the input fails to provide a proper id
const DEFAULT_NODE_ID: u32 = 999999;

/// A trait which provides a method for creating a graph from some configurations as well some
/// data that is used to analyze the graph.
 pub trait Input {
    /// The type of struct that contains information on what is to be read and how to read it
    type Configs;
    /// The type of additional data that is required for the target analysis.
    type AnalysisData;
    /// Function that uses the input configuration to create a graph and additional data which
    /// will be passed to some analysis
    ///
    /// # Errors
    ///
    /// May return a ['Error'] if the input configuration is invalid.
    fn read(&self, configs: Self::Configs) -> Result<(Graph, Self::AnalysisData), Box<dyn Error>>;
 }

/// Reads a csv file from a 'path' and converts it into a ['StringMatrix'].
/// The first row of the csv file will be ignored
///
/// # Errors
///
/// Will return an io error if the file at the given path cannot be opened
/// Will return an error if any rows in the csv file cannot be parsed
pub fn read_csv(path: &str) -> Result<StringMatrix, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut reader = csv::ReaderBuilder::new()
        .from_reader(file);
    let mut rows = Vec::new();
    for result in reader.records() {
        // read the csv
        let record = result?;
        // convert the record to a vector
        let record = record.into_iter().map(|x| x.to_string()).collect();
        rows.push(record);
    }
    Ok(rows)
}

/// Creates a graph from a standard links 'string_matrix'.
/// A standard link ['StringMatrix'] is a matrix where each row is an edge composed of 4
/// components: from node name, from node id, to node name, to node id. The components should be
/// exactly in the order listed.
///
/// # Errors
///
/// Will return a ['GraphCreationError'] if the 'string_matrix' is somehow invalid.
/// The 'string_matrix' can be invalid if:
/// * Each row has less or more than 4 components
/// * The from node id and to node id values cannot casted into u32
pub fn create_graph(string_matrix: &StringMatrix) -> Result<Graph, GraphCreationError> {
    let mut graph = Graph::new();
    let mut errors: Vec<String> = vec![];

    for (y, row) in string_matrix.into_iter().enumerate() {
        // Get the name and ID of the child and parent nodes
        let c_name = get_string_cell(&row, (0, y), &mut errors).unwrap_or(DEFAULT_NODE_NAME.to_string());
        let p_name = get_string_cell(&row, (2, y), &mut errors).unwrap_or(DEFAULT_NODE_NAME.to_string());
        let c_id = get_u32_cell(&row, (1, y), &mut errors).unwrap_or(DEFAULT_NODE_ID);
        let p_id = get_u32_cell(&row, (3, y), &mut errors).unwrap_or(DEFAULT_NODE_ID);
        // Add both nodes and an edge connecting the two
        graph.add_node(c_name, c_id);
        graph.add_node(p_name, p_id);
        graph.add_edge(c_id, p_id);
    }

    if errors.is_empty() {
        Ok(graph)
    } else {
        Err(GraphCreationError {
            errors
        })
    }
}

/// Get a string value from a 'row' in a ['StringMatrix'] and the 'pos' of the value.
/// The 'pos' is (x, y) where x is the index of value within the row, and y is the index of the
/// row within the string matrix.
///
/// # Errors
///
/// All errors are added to the 'errors' list which is meant to be passed to a ['GraphCreationError']
/// Returns none if the index of the value is not in the row
fn get_string_cell(row: &Vec<String>, pos: (usize, usize), errors: &mut Vec<String>) -> Option<String> {
    match row.get(pos.0) {
        Some(x) => {
            Some(x.trim().to_string())
        }
        None => {
            errors.push(errors::input::CellNotInRowError {
                row_i: pos.1,
                cell_i: pos.0,
            }.to_string());
            None
        }
    }
}

/// Call ['get_string_cell'] and then attempt to convert the result into u32
///
/// # Errors
///
/// * Returns None if ['get_string_cell'] return None
/// * Return None if the value at the given 'pos' cannot be converted to a u32
fn get_u32_cell(row: &Vec<String>, pos: (usize, usize), errors: &mut Vec<String>) -> Option<u32> {
    let string_val = get_string_cell(row, pos, errors).unwrap_or(DEFAULT_NODE_NAME.to_string());
    match string_val.parse::<u32>() {
        Ok(x) => {
            Some(x)
        }
        Err(e) => {
            errors.push(CellNotNumericError {
                    row_i: pos.1,
                    cell_i: pos.0,
                    cell_val: &string_val,
                    val: 0u32
                }.to_string()
            );
            errors.push(format!("\t{}", e));
            None
        }
    }

}

/// Configurations which holds information necessary to read values for the critically analysis
/// using the STD (standard) input
pub struct STDCritConfigs {
    /// The path to the input file
    pub in_path: String,
}

/// Structure used to read all the values necessary for a criticality analysis
pub struct STDCritInput {}
impl Input for STDCritInput {
    type Configs = STDCritConfigs;
    type AnalysisData = CriticalityData;

    fn read(&self, configs: STDCritConfigs) -> Result<(Graph, CriticalityData), Box<dyn Error>> {
        let string_matrix = read_csv(&configs.in_path)?;
        let graph =  create_graph(&string_matrix)?;
        return Ok((graph, CriticalityData {}))
    }
}