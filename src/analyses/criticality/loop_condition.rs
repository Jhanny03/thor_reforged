use dyn_clone::DynClone;
use crate::util;

pub trait CritLoopCondition : DynClone + Send{
    fn stop(&mut self) -> bool;
    fn split_to_threads(&self, threads: u64) -> Vec<Box<dyn CritLoopCondition>>;
}

#[derive(Clone)]
pub struct MaxLoopCondition {
    pub max: u64,
    pub index: u64
}

impl CritLoopCondition for MaxLoopCondition {
    fn stop(&mut self) -> bool {
        if self.index == self.max {
            return true
        }
        self.index += 1;
        return false
    }

    fn split_to_threads(&self, threads: u64) -> Vec<Box<(dyn CritLoopCondition)>> {
        let mut out: Vec<Box<(dyn CritLoopCondition)>> = vec![];
        let splits = util::split_range(self.index, self.max, threads);
        for split in splits {
            out.push(Box::new(
                MaxLoopCondition { max: split.1, index: split.0 }
            ))
        }
        out
    }
}