use crate::controller::Controller;
use crate::simulator::Simulator;

mod controller;
mod coffee_machine;
mod simulator;

fn main() {
    let mut simulator = Simulator::new();
    simulator.simulate()
}
