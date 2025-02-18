use crate::cpu::Cpu;
use crate::cpu::register_page::r16;

use super::register_page;

impl Cpu {
    pub fn add(&mut self, val: u8) {
        let (new_value, overflow) = self.get_regPage().a.overflowing_add(val);
        let hc = ((self.get_regPage().a & 0xF) + (val & 0xF)) > 0xF;
        self.get_regPage().set_z(new_value == 0);
        self.get_regPage().set_n(false);
        self.get_regPage().set_c(overflow);
        self.get_regPage().set_h(hc);
        self.get_regPage().a = new_value;
    }

    pub fn add16(&mut self, from: r16, to: r16) {
        let page= self.get_regPage();
        let v1 = page.read16(to.clone());
        let v2 = page.read16(from);
        let new_value = v1.wrapping_add(v2); 
        page.set_z(new_value == 0);
        page.set_n(false);
        page.set_c(v1 > 0xffff - v2);
        //page.set_h(); figure this out later
        page.write16(to, new_value);
    }

    //pub fn LOAD()

    pub fn execute(&mut self, byte: u8) {
        match byte {
            0x00 => (),
            0x01 => (),
            0x02 => (), 
            0x03 => (),
            0x04 => (),
            0x05 => (),
            0x06 => (),
            0x07 => (),
            0x08 => (),
            0x09 => self.add16(r16::BC, r16::HL),
            0x0A..0x19 => (),
            0x19 => self.add16(r16::DE, r16::HL),
            0x1A..0x29 => (),
            0x29 => self.add16(r16::HL, r16::HL),
            0x2A..0x39 => (),
            0x39 => self.add16(r16::SP, r16::HL),
            0x3A..=u8::MAX => (),
            _ => (),
        }
    }
}