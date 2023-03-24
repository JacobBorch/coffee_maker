use std::ops::Add;
use time::ext::NumericalStdDuration;
use time::Time;
use crate::controller::Controller;
use time_macros::time;
use rand::Rng;

pub struct Simulator {
    controller: Controller,
    requests: Vec<Time>,
    time: Time
}

impl Simulator {
    pub fn new() -> Self {
        Simulator {
            controller: Controller::new(),
            requests: vec![time!(08:00), time!(08:02), time!(13:00), time!(18:00)],
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

    pub fn get_test_intervals() -> Vec<Self> {
        todo!()
    }

    pub fn get_random_times(intervals: Vec<Interval>) -> Vec<Time>{
        let mut rng = rand::thread_rng();
        let mut result = Vec::new();
        for i in intervals {
            let time = rng.gen_range(i.start,i.end);
            result.append(time);
        }
        result
    }

    pub fn generate_random_intervals(amount: usize) -> Vec<Interval> {
        let mut intervals = Vec::new();
        for i in 0..amount {
            let interval = Self::get_random_interval();
            intervals.push(interval);
        }
        intervals
    }

    fn get_random_interval() -> Interval {
        let mut rng = rand::thread_rng();
        let first_minutes = rng.gen_range(0..60);
        let first_hours = rng.gen_range(0..24);
        let second_minutes = rng.gen_range(first_minutes..60);
        let second_hours = rng.gen_range(first_hours..24);
        let first_time = Time::from_hms(first_hours, first_minutes, 0).unwrap();
        let second_time = Time::from_hms(second_hours, second_minutes, 0).unwrap();
        Interval::new(first_time, second_time)
    }

    pub fn new(start: Time, end: Time) -> Self {
        Interval {
            start,
            end
        }
    }
}