use bitflags::bitflags;

bitflags! (
    pub struct Flags: u8 {
        const Z = 0b_1000_0000;
        const N = 0b_0100_0000;
        const H = 0b_0010_0000;
        const C = 0b_0001_0000;
    }
);

pub enum r8 {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L
}

pub enum r16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC
}

struct registers {
    a: u8,
    f: Flags,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16
}

impl registers {
    pub fn new(&mut self) -> registers {
        registers {
            a: 0,
            f: Flags::empty(),
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0
        }
    }

    pub fn check_z(&self) -> bool{
        return self.f.contains(Flags::Z);
    }
    pub fn check_n(&self) -> bool {
        return self.f.contains(Flags::N);
    }
    pub fn check_h(&self) -> bool {
        return self.f.contains(Flags::H);
    }
    pub fn check_c(&self) -> bool{
        return self.f.contains(Flags::C);
    }

    pub fn set_z(&mut self, z: bool) {
        self.f.set(Flags::Z, z);
    }
    pub fn set_n(&mut self, n: bool) {
        self.f.set(Flags::N, n);
    }
    pub fn set_h(&mut self, h: bool) {
        self.f.set(Flags::H, h);
    }
    pub fn set_c(&mut self, c: bool) {
        self.f.set(Flags::C, c);
    }
}
