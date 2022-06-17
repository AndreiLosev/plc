use std::time::Instant;

pub struct Pid {
    set_point: f32,
    kp: f32,
    tn: f32,
    tv: f32,
    y_offset: f32,
    y_min: f32,
    y_max: f32,
    err: f32,
    clock: Option<Instant>,
    i_accum: f32,
}

impl Pid {
    pub fn mew(
        set_point: f32,
        kp: f32,
        tn: f32,
        tv: f32,
        y_offset: f32,
        y_min: f32,
        y_max: f32,
    ) -> Self {
        Self {
            set_point, kp, tn, tv, y_offset, y_min, y_max,
            err: 0.0,
            clock: None,
            i_accum: 0.0,
        }
    }

    pub fn set_set_point(&mut self, set_point: f32) {
        self.set_point = set_point;
    }

    pub fn set_kp(&mut self, kp: f32) {
        self.kp = kp;
    }

    pub fn set_tn(&mut self, tn: f32) {
        self.tn = tn;
    }

    pub fn set_tv(&mut self, tv: f32) {
        self.tv = tv;
    }

    pub fn set_y_offset(&mut self, y_offset: f32) {
        self.y_offset = y_offset;
    }

    pub fn set_y_min(&mut self, y_min: f32) {
        self.y_min = y_min;
    }

    pub fn set_y_max(&mut self, y_max: f32) {
        self.y_max = y_max;
    }

    pub fn get_set_point(&self) -> f32 { self.set_point }

    pub fn get_kp(&self) -> f32 { self.kp }

    pub fn get_tn(&self) -> f32 { self.tn }

    pub fn get_tv(&self) -> f32 { self.tv }

    pub fn get_y_offset(&self) -> f32 { self.y_offset }

    pub fn get_y_min(&self) -> f32 { self.y_min }

    pub fn get_y_max(&self) -> f32 { self.y_max }

    pub fn run(&mut self, actual: f32, y_maual: f32, manual: bool, reset: bool ) -> f32 {
        
        if reset {
            self.i_accum = 0.0;
            self.clock = None;
            return 0.0;
        }

        if manual {
            self.clock = Some(Instant::now());
            self.i_accum = self.limits(y_maual);
            return self.i_accum;
        } 

        let mut period = 0.0;

        if self.clock.is_none() {
            self.i_accum = self.limits(self.y_offset);
        }

        if let Some(i) = self.clock {
            period = i.elapsed().as_secs_f32();
        }

        self.clock = Some(Instant::now());

        let err = self.set_point - actual;

        let i = self.calc_i(err, period);

        let d = self.calc_d(err, period);

        let result = self.kp * (err + i + d);

        self.limits(result)
    }


    fn calc_i(&mut self, err: f32, period: f32) -> f32 {

        if self.tn < 0.001 {
            return 0.0;
        }

        self.i_accum += err * period / self.tn;

        self.i_accum = self.limits(self.i_accum);

        self.i_accum
    }

    fn calc_d(&mut self, err: f32, period: f32) -> f32 {

        if self.tv < 0.001 {
            return 0.0;
        }

        let d = (err - self.err) * self.tv / period;

        self.err = err;

        self.limits(d)
    }

    fn limits(&self, value: f32) -> f32 {
        if value > self.y_max {
            self.y_max
        } else if value < self.y_min {
            self.y_min
        } else {
            value
        }
    }

}