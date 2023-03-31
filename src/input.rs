use std::error::Error;
use std::fs::File;

use std::str::FromStr;
use csv;
use crate::network::{Graph, EdgeValueMap};
use crate::{errors};
use crate::analyses::criticality::CriticalityData;

use crate::errors::input::{CellNotNumericError, CreateError};

/// Module containing all necessary structures for reading inputs / data necessary for analyses.
///
/// Reading from any file is done using a struct that implements the ['Input'] trait.
/// All input structures must provide some way to build a graph, as well as some type of
/// additional information for some analysis. Note that each analysis requires it's own input
/// implementation. Most input structures will likely share similar code.

/// A row of a strings
type StringRow = Vec<String>;

/// A row of a strings belonging to a matrix
type StringCol = Vec<String>;

/// List of rows (list of strings) obtained from reading a file.
type RowStringMatrix = Vec<StringRow>;

/// List of rows (list of strings) obtained from reading a file.
type ColStringMatrix = Vec<StringCol>;

// TODO: return error if all the rows are not the same length
fn row_to_col_matrix(row_matrix: &RowStringMatrix) -> ColStringMatrix {
    let mut col = Vec::new();
    for _ in 0..row_matrix.len() {
        col.push(Vec::new());
    }
    for row in row_matrix {
        for (i, cell) in row.iter().enumerate() {
            col[i].push(cell.to_string());
        }
    }
    return col;
}

// TODO: return error if all the col are not the same length
fn col_to_row_matrix(col_matrix: &ColStringMatrix) -> RowStringMatrix {
    let mut row = Vec::new();
    for _ in 0..col_matrix[0].len() {
        row.push(Vec::new());
    }
    for col in col_matrix {
        for (i, cell) in col.iter().enumerate() {
            row[i].push(cell.to_string());
        }
    }
    row
}

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
fn read_csv_matrix(path: &str) -> Result<RowStringMatrix, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
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
/// This function also return a list of edges which can be passed to the ['read_edge_state_map']
/// function to map any column of values to each edge. The edges returned are in the order that
/// they appear in the input matrix.
///
/// # Errors
///
/// Will return a ['GraphCreationError'] if the 'string_matrix' is somehow invalid.
/// The 'string_matrix' can be invalid if:
/// * Each row has less or more than 4 components
/// * The from node id and to node id values cannot casted into u32
fn create_graph(edges_matrix: &RowStringMatrix) -> Result<(Graph, Vec<(u32, u32)>), CreateError<RowStringMatrix>> {
    let mut graph = Graph::new();
    let mut errors: Vec<String> = vec![];
    let mut edges = vec![];

    for (y, row) in edges_matrix.into_iter().enumerate() {
        // Get the name and ID of the child and parent nodes
        let c_name = get_string_cell(&row, (0, y), 0,&mut errors).unwrap_or(DEFAULT_NODE_NAME.to_string());
        let p_name = get_string_cell(&row, (2, y), 2, &mut errors).unwrap_or(DEFAULT_NODE_NAME.to_string());
        let c_id = get_from_str_cell(&row, (1, y), 1, &mut errors).unwrap_or(DEFAULT_NODE_ID);
        let p_id = get_from_str_cell(&row, (3, y), 3, &mut errors).unwrap_or(DEFAULT_NODE_ID);

        // Add both nodes and an edge connecting the two
        graph.add_node(c_name, c_id);
        graph.add_node(p_name, p_id);
        graph.add_edge(c_id, p_id);
        edges.push((c_id, p_id));
    }

    if errors.is_empty() {
        Ok((graph, edges))
    } else {
        Err(CreateError {
            task: "creating a graph".to_string(),
            errors,
            input: edges_matrix.clone(),
        })
    }
}


fn create_edge_value_map<T: Clone + FromStr>(edges: &Vec<(u32, u32)>, col: &StringCol, defaults: T) -> Result<EdgeValueMap<T>, CreateError<StringCol>>{
    let mut map = EdgeValueMap::new();
    let mut errors: Vec<String> = vec![];
    for (y, edge) in edges.into_iter().enumerate() {
        let value = get_from_str_cell(&col, (0, y), y, &mut errors).unwrap_or(defaults.clone());
        map.insert((edge.0, edge.1), value);
    }

    if errors.is_empty() {
        Ok(map)
    } else {
        Err(CreateError {
            task: "creating an edge value map".to_string(),
            errors,
            input: col.clone(),
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
fn get_string_cell(list: &Vec<String>, pos: (usize, usize), cell_i: usize, errors: &mut Vec<String>) -> Option<String> {
    match list.get(cell_i) {
        Some(x) => {
            Some(x.trim().to_string())
        }
        None => {
            errors.push(errors::input::CellNotFoundError {
                cell_pos: pos,
            }.to_string());
            None
        }
    }
}

/// Call ['get_string_cell'] and then attempt to convert the result to type T
///
/// # Errors
///
/// * Returns None if ['get_string_cell'] return None
/// * Return None if the value at the given 'pos' cannot be converted to type T
fn get_from_str_cell<T>(list: &Vec<String>, pos: (usize, usize), cell_i: usize, errors: &mut Vec<String>) -> Option<T>
where T: FromStr
{
    let string_val = get_string_cell(list, pos, cell_i, errors).unwrap_or(DEFAULT_NODE_NAME.to_string());
    match string_val.parse::<T>() {
        Ok(x) => {
            Some(x)
        }
        Err(_e) => {
            errors.push(
                CellNotNumericError::<T> { cell_pos: pos, cell_val: string_val, ..Default::default() }.to_string()
            );
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
        let links_map = read_csv_matrix(&configs.in_path)?;
        println!("row map: {:?}", links_map);
        let col = row_to_col_matrix(&links_map);
        println!("col map: {:?}", col);
        println!("back to row map: {:?}", col_to_row_matrix(&col));
        let alpha_matrix = read_csv_matrix("alpha.csv")?;
        let alpha_col = &alpha_matrix[0];
        let (graph, edges) =  create_graph(&links_map)?;
        let alpha_map = create_edge_value_map(&edges, &alpha_col, -1.0f32)?;
        return Ok((graph, CriticalityData {}))
    }
}