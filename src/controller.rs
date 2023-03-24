use std::collections::VecDeque;
use time::ext::NumericalStdDuration;
use time::Time;
use time_macros::time;
use crate::coffee_machine::CoffeeMachine;

pub struct Controller {
    data: Vec<Log>,
    machine: CoffeeMachine,
    time: Time,
    queue: VecDeque<Time>,
    people: usize,
}

impl Controller {
    pub fn new() -> Self {
        Controller {
            data: vec![],
            machine: CoffeeMachine::new(),
            time: Time::MIDNIGHT,
            queue: VecDeque::new(),
            people: 0,
        }
    }

    pub fn tick(&mut self) {
        if self.machine.coffee_ready > 0 {
            self.machine.coffee_ready -= 1;
            self.queue.pop_front();
            println!("Delivered coffee at: {}", self.time);
        }
        if self.machine.timer.minute() == 0 && !self.queue.is_empty() {
            self.machine.brew();
        }
        self.time += 1.std_minutes();
        self.machine.tick();
    }

    pub fn request(&mut self, time: Time) {
        self.queue.push_back(time);
        println!("Requested coffee at {}", time);
    }

    pub fn delta_people(&mut self, amount: usize) {
        self.people += amount;
    }
}

struct Log {
    times: Vec<Time>
}