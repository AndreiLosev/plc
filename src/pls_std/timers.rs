use std::time::{Duration, Instant};

struct Ton;
struct Tof;
struct Tp;

pub struct Timer<T> {
    in1: bool,
    pt: Duration,
    q: bool,
    et: Duration,
    time: Option<Instant>,
    timerType: T,
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
            time: None,
            timerType: Ton,
        }
    }

    pub fn run(&mut self, in1: bool) {

        let timer_run = in1 && !self.in1;
        self.in1 = in1;

        if timer_run {
            self.time = Some(Instant::now());
        }

        if let Some(i) = self.time {

            self.et = i.elapsed();

            if i.elapsed() >= self.pt {
                self.et = self.pt;
                self.time = None;
            }
        }

        self.q = self.et == self.pt;

    }
    
}

impl Timer<Tof> {
    pub fn new_ton(pt: Duration) -> Self {
        Self {
            in1: false,
            pt,
            q: false,
            et: Duration::ZERO,
            time: None,
            timerType: Tof,
        }
    }
}

impl Timer<Tp> {
    pub fn new_ton(pt: Duration) -> Self {
        Self {
            in1: false,
            pt,
            q: false,
            et: Duration::ZERO,
            time: None,
            timerType: Tp,
        }
    }
}
