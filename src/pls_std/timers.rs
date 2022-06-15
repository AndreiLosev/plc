use std::time::{Duration, Instant};

struct Ton(Option<Instant>);
struct Tof(Option<Instant>);
struct Tp(Option<Instant>);

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
        }

        self.q = self.timer_type.0.is_some();

    }
}


// #[test]
// fn test_ton() {
//     let timer = Timer::new_ton(Duration::from_secs(2));
// }