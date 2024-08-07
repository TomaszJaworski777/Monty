use super::MctsParams;

pub struct MathTables {
    explore_scaling: Vec<f32>
}

impl MathTables {
    pub const EXPLORE_SCALING_TABLE_SIZE: i32 = 10_000_000;

    pub fn new(params: &MctsParams) -> Self {
        let mut explore_scaling = Vec::with_capacity(Self::EXPLORE_SCALING_TABLE_SIZE as usize);
        for index in 0..Self::EXPLORE_SCALING_TABLE_SIZE {
            explore_scaling.push((params.expl_tau() * ((index + 1) as f32).ln()).exp())
        }

        Self { 
            explore_scaling 
        }
    }

    #[inline]
    pub fn get_explore_scaling_value(&self, index: usize) -> f32 {
        self.explore_scaling[index]
    }
}