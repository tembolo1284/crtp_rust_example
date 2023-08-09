use crate::interest_calculator::InterestCalculator;

pub struct SimpleInterestCalculator;

impl InterestCalculator for SimpleInterestCalculator {
    fn calculate(&self, principal: f64, rate: f64, time: f64) -> f64 {
        principal * rate * time / 100.0
    }
}
