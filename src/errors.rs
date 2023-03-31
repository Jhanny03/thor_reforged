pub mod input {
    use std::{error::Error, fmt};
    use std::any::type_name;
    use std::fmt::{Debug, Display, Formatter};
    use std::marker::PhantomData;

    pub struct CellNotFoundError {
        pub cell_pos: (usize, usize),
    }
    impl Error for CellNotFoundError {}
    impl Debug for CellNotFoundError {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "The cell with pos ({}, {}) is out of bounds", self.cell_pos.0, self.cell_pos.1)
        }
    }
    impl Display for CellNotFoundError {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "The cell with pos ({}, {}) is out of bounds", self.cell_pos.0, self.cell_pos.1)
        }
    }

    pub struct CellNotNumericError<T> {
        pub cell_pos: (usize, usize),
        pub cell_val: String,
        pub phantom: PhantomData<T>
    }
    impl<T> Default for CellNotNumericError<T> {
        fn default() -> Self {
            CellNotNumericError {
                cell_pos: (0, 0),
                cell_val: "".to_string(),
                phantom: Default::default(),
            }
        }
    }
    impl<T> Error for CellNotNumericError<T> {
    }
    impl<T> Debug for CellNotNumericError<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "The cell at ({}, {}), with value: {}, should be a {} value", self.cell_pos.0, self.cell_pos.1, self.cell_val, type_name::<T>())
        }
    }
    impl<T> Display for CellNotNumericError<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "The cell at ({}, {}), with value: {}, should be a {} value", self.cell_pos.0, self.cell_pos.1, self.cell_val, type_name::<T>())
        }
    }

    pub struct CreateError<T>
        where T: Debug{
        pub task: String,
        pub errors: Vec<String>,
        pub input: T,
    }
    impl<T: Debug> CreateError<T> {
        fn get_string_error(&self) -> String {
            let mut string_errors = "".to_string();
            for error in self.errors.iter() {
                string_errors += error;
                string_errors += "\n";
            }
            string_errors
        }
    }
    impl<T: Debug> Error for CreateError<T> {}
    impl<T: Debug> Debug for CreateError<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "The program encountered the following errors when {}: \n{} The input was: {:?}"
                   , self.task, self.get_string_error(), self.input)
        }
    }
    impl<T: Debug> Display for CreateError<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "The program encountered the following errors when {}: \n{} The input was: {:?}"
                   , self.task, self.get_string_error(), self.input)
        }
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