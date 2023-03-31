pub mod visibility_states_gen {
    use std::collections::{HashSet};
    use dyn_clone::DynClone;
    use rand::{Rng, SeedableRng};
    use rand::rngs::{StdRng};
    use crate::network::{NodeValueMap};

    pub trait VisGen: DynClone + Send {
        fn next_states(&mut self) -> NodeValueMap<u8>;
        fn split_to_threads(&self, threads: u64) -> Vec<Box<dyn VisGen>>;
    }

    const DEFAULT_OFF_CHANCE: f32 = 0.5;

    #[derive(Clone)]
    pub struct RandomGen {
        pub rng: StdRng,
        pub ids: HashSet<u32>,
        pub off_chances: NodeValueMap<f32>
    }

    impl VisGen for RandomGen {
        fn next_states(&mut self) -> NodeValueMap<u8> {
            let mut new_states = NodeValueMap::new();
            for id in &self.ids {
                let rand: f32 = self.rng.gen();
                let off_chance = self.off_chances.get(id).unwrap_or(&DEFAULT_OFF_CHANCE);
                if *off_chance < rand{
                    new_states.insert(*id, 0);
                } else {
                    new_states.insert(*id, 1);
                }
            }
            new_states
        }

        fn split_to_threads(&self, threads: u64) -> Vec<Box<(dyn VisGen)>> {
            let mut out: Vec<Box<(dyn VisGen)>> = vec![];
            for _ in 0..threads {
                out.push(Box::new(
                    RandomGen {
                        rng: StdRng::from_entropy(),
                        ids: self.ids.clone(),
                        off_chances: self.off_chances.clone(),
                    }
                ))
            }
            out
        }
    }
}