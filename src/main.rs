mod interest_calculator;
mod compound_interest_calculator;
mod simple_interest_calculator;

use compound_interest_calculator::CompoundInterestCalculator;
use simple_interest_calculator::SimpleInterestCalculator;
use crate::interest_calculator::InterestCalculator; 

fn main() {
    let simple = SimpleInterestCalculator;
    let compound = CompoundInterestCalculator;

    let principal = 1000.0;
    let rate = 5.0;
    let time = 2.0;

    println!(
        "Simple Interest: {}",
        simple.calculate(principal, rate, time)
    );
    println!(
        "Compound Interest: {}",
        compound.calculate(principal, rate, time)
    );
}
