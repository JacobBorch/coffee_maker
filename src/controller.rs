use time::Time;
use crate::coffee_machine::CoffeeMachine;

pub struct Controller {
    data: Vec<Log>,
    machine: CoffeeMachine,
    time: Time
}

impl Controller {
    pub fn new() -> Self {
        Controller {
            data: vec![],
            machine: CoffeeMachine::new(),
            time: Time::MIDNIGHT
        }
    }

    pub fn request(&self, time: Time) {
        println!("Requested coffee at {}", time)
    }
}

struct Log {
    times: Vec<Time>
}