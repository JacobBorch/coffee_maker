use std::ops::Add;
use time::ext::NumericalStdDuration;
use time::Time;
use crate::controller::Controller;
use time_macros::time;

pub struct Simulator {
    controller: Controller,
    requests: Vec<Time>,
    time: Time
}

impl Simulator {
    pub fn new() -> Self {
        Simulator {
            controller: Controller::new(),
            requests: vec![time!(08:00), time!(13:00), time!(18:00)],
            time: Time::MIDNIGHT
        }
    }

    fn tick(&mut self) {
        let next_request = self.requests.first();
        match next_request {
            None => {return;}
            Some(request) => {
                if self.time == *request {
                    self.requests.remove(0);
                    self.controller.request(self.time)
                }
            }
        }
        self.controller.tick()
    }

    pub fn simulate(&mut self) {
        while self.time != time!(23:59) {
            self.tick();
            self.time += 1.std_minutes();
        }
    }
}

struct Interval {
    start: Time,
    end: Time
}

impl Interval {

    pub fn get_intervals() -> Vec<Self> {
        todo!()
    }

    pub fn new(start: Time, end: Time) -> Self {
        Interval {
            start,
            end
        }
    }
}