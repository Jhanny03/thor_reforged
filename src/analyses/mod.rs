pub mod criticality;

pub const VISIBLE_VAL: u8 = 1;

pub trait Analysis {
    fn analyze(self);
}