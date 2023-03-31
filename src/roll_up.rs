use dyn_clone::DynClone;
use crate::analyses::VISIBLE_VAL;
use crate::network::{GraphNodeState};

const MAX_OPERABILITY: f32 = 1.0;
const MIN_OPERABILITY: f32 = 0.0;

pub trait RollUp : DynClone + Send {
    fn get_value(&self, t_id: &u32, children: &Vec<u32>, visibilities: &GraphNodeState<u8>, values: &GraphNodeState<f32>) -> f32 {
        if children.is_empty() {
            return MAX_OPERABILITY;
        }
        let t_visible = visibilities.get(t_id);
        return match t_visible {
            None => { self.compute_val(t_id, children, values) }
            Some(x) => {
                if *x == VISIBLE_VAL { self.compute_val(t_id, children, values) } else { MIN_OPERABILITY }
            }
        }
    }
    fn compute_val(&self, t_id: &u32, children: &Vec<u32>, values: &GraphNodeState<f32>) -> f32;
}

#[derive(Clone)]
pub struct OrRule {}

impl RollUp for OrRule {
    fn compute_val(&self, _t_id: &u32, children: &Vec<u32>, values: &GraphNodeState<f32>) -> f32 {
        let mut max = MIN_OPERABILITY;
        for child in children {
            match values.get(child) {
                None => {
                    return MAX_OPERABILITY
                }
                Some(val) => {
                    if *val > max {
                        max = *val;
                    }
                }
            }
        }
        return max
    }
}