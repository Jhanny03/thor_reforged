pub mod input {
    use std::{error::Error, fmt};
    use std::any::type_name;
    use std::fmt::{Debug, Display, Formatter};
    use std::marker::PhantomData;

    pub struct CellNotInRowError {
        pub row_i : usize,
        pub cell_i: usize
    }
    impl Error for CellNotInRowError {}
    impl Debug for CellNotInRowError {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "Row {} does not contain cell {}. Cell is out of bounds.", self.row_i, self.cell_i)
        }
    }
    impl Display for CellNotInRowError {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "Row {} does not contain cell {}. Cell is out of bounds.", self.row_i, self.cell_i)
        }
    }

    pub struct CellNotNumericError<T> {
        pub row_i : usize,
        pub cell_i: usize,
        pub cell_val: String,
        pub phantom: PhantomData<T>
    }
    impl<T> Default for CellNotNumericError<T> {
        fn default() -> Self {
            CellNotNumericError {
                row_i: 0,
                cell_i: 0,
                cell_val: "".to_string(),
                phantom: Default::default(),
            }
        }
    }
    impl<T> Error for CellNotNumericError<T> {
    }
    impl<T> Debug for CellNotNumericError<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "The cell at row {}, index {}, with value: {}, should be a {} value", self.row_i, self.cell_i, self.cell_val, type_name::<T>())
        }
    }
    impl<T> Display for CellNotNumericError<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "The cell at row {}, index {}, with value: {}, should be a {} value", self.row_i, self.cell_i, self.cell_val, type_name::<T>())
        }
    }

    pub struct GraphCreationError {
        pub errors: Vec<String>
    }
    impl GraphCreationError {
        fn get_string_error(&self) -> String {
            let mut string_errors = "".to_string();
            for error in self.errors.iter() {
                string_errors += error;
                string_errors += "\n";
            }
            string_errors
        }
    }
    impl Error for GraphCreationError {}
    impl Debug for GraphCreationError {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "The program encountered the following errors when creating a graph: {} ", self.get_string_error())
        }
    }
    impl Display for GraphCreationError {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "The program encountered the following errors when creating a graph: {} ", self.get_string_error())        }
    }
}

pub mod network {
    use std::{error::Error, fmt};
    use std::fmt::{Debug, Display, Formatter};

    fn multiple_nodes_error(node_type: &str, node_dependent: &str, nodes: &Vec<u32>) -> String {
        return if nodes.is_empty() {
            format!("Couldn't determine a {} node. \
                All nodes have {}", node_type, node_dependent)
        } else {
            format!("Could not determine a single start node. The options are:\n{:?}", nodes)
        }
    }

    pub struct StartNodeError {
        pub starts: Vec<u32>
    }
    impl Error for StartNodeError {}
    impl Debug for StartNodeError {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "{}", multiple_nodes_error("start", "children", &self.starts))
        }
    }
    impl Display for StartNodeError {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "{}", multiple_nodes_error("start", "children", &self.starts))
        }
    }

    pub struct EndNodeError {
        pub ends: Vec<u32>
    }
    impl Error for EndNodeError {}
    impl Debug for EndNodeError {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "{}", multiple_nodes_error("end", "parents", &self.ends))
        }
    }
    impl Display for EndNodeError {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "{}", multiple_nodes_error("end", "parents", &self.ends))
        }
    }

    pub struct NoEndConnectionError {
        pub start_id: u32,
        pub end_id: u32
    }
    impl Error for NoEndConnectionError {}
    impl Debug for NoEndConnectionError {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "The start node with id: {} does not connect to the end node with id: {}", self.start_id, self.end_id)
        }
    }
    impl Display for NoEndConnectionError {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "The start node with id: {} does not connect to the end node with id: {}", self.start_id, self.end_id)
        }
    }
}