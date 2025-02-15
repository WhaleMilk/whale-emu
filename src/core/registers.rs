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
    SP
}

pub struct registers {
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
    pub fn new() -> registers {
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

    fn read16(&self, reg: r16) -> u16 {
        use self::r16::*;
        match reg {
            SP => self.sp,
            AF => ((self.a as u16) << 8) | (self.f.bits() as u16),
            BC => ((self.b as u16) << 8) | (self.c as u16),
            DE => ((self.d as u16) << 8) | (self.e as u16),
            HL => ((self.h as u16) << 8) | (self.l as u16),
        }
    }

    fn write16(&mut self, reg: r16, val: u16) {
        use self::r16::*;
        match reg{
            SP => self.sp = val,
            AF => {
                self.a = (val >> 8) as u8;
                self.f = Flags::from_bits_truncate(val as u8)
            },
            BC => {
                self.b = (val >> 8) as u8;
                self.c = (val) as u8;
            },
            DE => {
                self.d = (val >> 8) as u8;
                self.e = val as u8;
            },
            HL => {
                self.h = (val >> 8) as u8;
                self.l = val as u8;
            }
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
