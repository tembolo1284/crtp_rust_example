pub trait InterestCalculator {
    fn calculate_interest(&self, principal: f64, rate: f64, time: f64) -> f64 {
        self.calculate(principal, rate, time)
    }

    fn calculate(&self, principal: f64, rate: f64, time: f64) -> f64;
}
