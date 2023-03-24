use time::Time;

const COFFE_BREW_TIME: usize = 5; //time in minutes

pub struct CoffeeMachine {
    timer: usize,
    pub is_available: bool,
}

impl CoffeeMachine {

    pub fn new() -> Self {
        CoffeeMachine {
            timer: 0,
            is_available: true,
        }
    }

    pub fn tick(&mut self) -> {

    }

    pub fn brew(&self) {
        timer = COFFE_BREW_TIME;
        todo!()
    }
}