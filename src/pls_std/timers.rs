use std::time::{Duration, Instant};

pub struct Ton(Option<Instant>);
pub struct Tof(Option<Instant>);
pub struct Tp(Option<Instant>);

pub struct Timer<T> {
    in1: bool,
    pt: Duration,
    q: bool,
    et: Duration,
    timer_type: T,
}

impl<T> Timer<T> {
    pub fn get_q(&self) -> bool { self.q }
    pub fn get_et(&self) -> Duration { self.et }
    pub fn get_in1(&self) -> bool { self.in1 }
    pub fn get_pt(&self) -> Duration { self.pt }
    pub fn set_pt(&mut self, pt: Duration) { self.pt = pt; }

}

impl Timer<Ton> {
    pub fn new_ton(pt: Duration) -> Self {
        Self {
            in1: false,
            pt,
            q: false,
            et: Duration::ZERO,
            timer_type: Ton(None),
        }
    }

    pub fn run(&mut self, in1: bool) {

        let timer_run = in1 && !self.in1;
        self.in1 = in1;

        if timer_run {
            self.timer_type.0 = Some(Instant::now());
        }

        if let Some(i) = self.timer_type.0 {

            self.et = i.elapsed();

            if i.elapsed() >= self.pt {
                self.et = self.pt;
                self.timer_type.0 = None;
            }

            if !self.in1 {
                self.timer_type.0 = None;
            }
        }

        self.q = self.et == self.pt;

    }
    
}

impl Timer<Tof> {
    pub fn new_tof(pt: Duration) -> Self {
        Self {
            in1: false,
            pt,
            q: false,
            et: Duration::ZERO,
            timer_type: Tof(None),
        }
    }

    pub fn run(&mut self, in1: bool) {

        let timer_run = !in1 && self.in1;
        self.in1 = in1;

        if timer_run {
            self.timer_type.0 = Some(Instant::now());
        }

        if let Some(i) = self.timer_type.0 {

            self.et = i.elapsed();

            if i.elapsed() >= self.pt {
                self.et = self.pt;
                self.timer_type.0 = None;
            }
        }

        self.q = self.in1 || self.et < self.pt;
    }
}

impl Timer<Tp> {
    pub fn new_tp(pt: Duration) -> Self {
        Self {
            in1: false,
            pt,
            q: false,
            et: Duration::ZERO,
            timer_type: Tp(None),
        }
    }

    pub fn run(&mut self, in1: bool) {

        let timer_run = in1 && !self.in1 && self.timer_type.0.is_none();
        self.in1 = in1;

        if timer_run {
            self.timer_type.0 = Some(Instant::now());
        }

        if let Some(i) = self.timer_type.0 {

            self.et = i.elapsed();

            if i.elapsed() >= self.pt {
                self.et = self.pt;
                self.timer_type.0 = None;
            }
        }

        self.q = self.timer_type.0.is_some();

    }
}


#[test]
fn test_ton() {

    let duration = Duration::from_millis(20);
    let mut timer = Timer::new_ton(duration);

    let mut result = [Duration::ZERO, Duration::ZERO];

    let mut clock = Instant::now();

    let mut helper = true;

    loop {

        timer.run(true);

        if timer.get_q() && helper {
            result[0] = clock.elapsed();
            helper = false;
        }

        if clock.elapsed() > duration.checked_mul(3).unwrap() {
            timer.run(false);
            break;
        }
    }

    helper = true;

    clock = Instant::now();

    loop {
        timer.run(helper);


        if timer.get_q() && helper {
            result[1] = clock.elapsed();
        }

        if clock.elapsed() > Duration::from_millis(19) {
            helper = false;
        }

        if clock.elapsed() > duration.checked_mul(3).unwrap() {
            break;
        }
    }

    assert_eq!(
        [result[0].as_millis(), result[1].as_millis()],
        [duration.as_millis(), Duration::ZERO.as_millis()],
    );
}


#[test]
fn test_tof() {
    let duration = Duration::from_millis(20);
    let mut timer = Timer::new_tof(duration);

    let mut result = [Duration::ZERO, Duration::ZERO];

    let mut helper = true;

    let mut clock = Instant::now();

    loop {
        
        timer.run(helper);

        if clock.elapsed() > Duration::from_millis(10) {
            helper = false;
        }

        if !timer.get_q() {
            result[0] = clock.elapsed();
            break;
        }

    }

    clock = Instant::now();
    helper = false;

    loop {
        timer.run(helper);

        if clock.elapsed() > Duration::from_millis(5) && clock.elapsed() < Duration::from_millis(15) {
            helper = true;
        } else if clock.elapsed() > Duration::from_millis(30) && clock.elapsed() < Duration::from_millis(45) {
            helper = true;
        } else {
            helper = false;
        }

        if !timer.get_q() && clock.elapsed().as_millis() > 6 {
            result[1] = clock.elapsed();
            break;
        }
    }

    assert_eq!(
        [result[0].as_millis(), result[1].as_millis()],
        [30, 65]
    )

}

#[test]
fn test_tp() {
    let duration = Duration::from_millis(20);
    let mut timer = Timer::new_tp(duration);

    let mut result = [Duration::ZERO, Duration::ZERO];

    let mut helper = true;

    let mut clock = Instant::now();

    loop {
        timer.run(helper);

        if !timer.get_q() {
            result[0] = clock.elapsed();
            timer.run(false);
            break;
        }
    }

    std::thread::sleep(Duration::from_millis(21));

    clock = Instant::now();

    helper = true;

    loop {
        timer.run(helper);

        helper = clock.elapsed().as_millis() % 2 == 0;

        helper = !helper;


        if !timer.get_q() {
            result[1] = clock.elapsed();
            break;
        }
    }

    assert_eq!(
        [result[0].as_millis(), result[1].as_millis()],
        [20, 20],
    )

}