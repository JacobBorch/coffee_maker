use time::{Duration, Time};
use time::ext::NumericalStdDuration;
use time_macros::time;
use std::cmp;

const COFFEE_BREW_TIME: Time = time!(00:05); //time in minutes

pub struct CoffeeMachine {
    pub timer: Time,
    pub coffee_ready: usize,
}

impl CoffeeMachine {
    pub fn new() -> Self {
        CoffeeMachine {
            timer: time!(00:00),
            coffee_ready: 0,
        }
    }

    pub fn tick(&mut self) {
        if self.timer.minute() == 1 {
            self.coffee_ready += 1
        }
        if self.timer.minute() > 0 {
            self.timer -= 1.std_minutes();

        }
    }

    pub fn brew(&mut self) {
        self.timer = COFFEE_BREW_TIME;
        println!("Brewing coffee uwu 😋")
    }
}