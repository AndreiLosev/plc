pub struct RTrig {
    clk: bool,
    q: bool,
}

impl RTrig {
    pub fn new() -> Self { Self { clk: false, q: false } }
    pub fn get_q(&self) -> bool { self.q }

    pub fn run(&mut self, clk: bool) {
        self.q = clk && !self.clk;
        self.clk = clk;
    }
}

pub struct FTrig {
    clk: bool,
    q: bool,
}

impl FTrig {
    pub fn new() -> Self { Self { clk: false, q: false } }
    pub fn get_q(&self) -> bool { self.q }

    pub fn run(&mut self, clk: bool) {
        self.q = !clk && self.clk;
        self.clk = clk;
    }
}

pub struct Rs {
    set: bool,
    reset: bool,
    q: bool,
}

impl  Rs {
    pub fn new() -> Self { Self { set: false, reset: false, q: false } }
    pub fn get_q(&self) -> bool { self.q }

    pub fn run(&mut self, set: bool, reset: bool) {
        self.q = (set || self.q) && !reset;
        self.set = set;
        self.reset = reset;
    }
}


#[test]
fn test_r_trig() {

    let mut resutl: Vec<bool> = vec![];

    let mut trig = RTrig::new();

    for i in [false, true, true, true, false, false, true, true] {
        trig.run(i);
        resutl.push(trig.get_q());
    }

    let expect = [false, true, false, false, false, false, true, false];

    assert_eq!(resutl, expect);

}

#[test]
fn test_f_trig() {

    let mut resutl: Vec<bool> = vec![];

    let mut trig = FTrig::new();

    for i in [false, true, false, true, false, false, true, true] {
        trig.run(i);
        resutl.push(trig.get_q());
    }

    let expect = [false, false, true, false, true, false, false, false];

    assert_eq!(resutl, expect);

}


#[test]
fn test_rs_trig() {
    let mut resutl: Vec<bool> = vec![];

    let mut trig = Rs::new();

    for i in 0..16 {
        let set = i % 2 == 0;
        let reset = i % 3 == 0;
        trig.run(set, reset);
        resutl.push(trig.get_q());
    }

    let expect = [
        false, false, true, false,
        true, true, false, false,
        true, false, true, true,
        false, false, true, false,
    ];

    assert_eq!(resutl, expect);
    
}