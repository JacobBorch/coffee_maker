use std::collections::VecDeque;
use time::Time;
use crate::coffee_machine::CoffeeMachine;

pub struct Controller {
    data: Vec<Log>,
    machine: CoffeeMachine,
    time: Time,
    queue: VecDeque<Time>
}

impl Controller {
    pub fn new() -> Self {
        Controller {
            data: vec![],
            machine: CoffeeMachine::new(),
            time: Time::MIDNIGHT,
            queue: VecDeque::new()
        }
    }

    pub fn tick(&mut self) {
        if self.machine.is_done {
            self.queue.pop_front();
            if !self.queue.is_empty() {
                self.machine.brew()
            }
        }
        self.machine.tick()
    }

    pub fn request(&mut self, time: Time) {
        self.queue.push_back(time);
        println!("Requested coffee at {}", time)
    }
}

struct Log {
    times: Vec<Time>
}