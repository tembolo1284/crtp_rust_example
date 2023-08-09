use crate::interest_calculator::InterestCalculator;

pub struct CompoundInterestCalculator;

impl InterestCalculator for CompoundInterestCalculator {
    fn calculate(&self, principal: f64, rate: f64, time: f64) -> f64 {
        principal * (1.0 + rate / 100.0).powf(time) - principal
    }
}
