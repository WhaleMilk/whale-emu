use crate::core::registers::r16;
use crate::core::cpu::Cpu;

use super::registers::registers;

impl Cpu {
    pub fn ADD(&mut self, from: r16, to: r16, out: r16) {
        let page = self.get_regPage();
        let (new_value, overflow) = registers::read16(page, to).overflowing_add(registers::read16(page, from)); 
    }

    pub fn from_byte(&mut self, byte: u8) {
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
            0x09 => self.ADD(r16::BC, r16::HL, r16::HL),
            0x0A => (),
            0x0B => (),
            0x0C => (),
            0x0D => (),
            0x0E => (),
            0x10 => (),
            0x11 => (),
            0x12 => (),
            0x13 => (),
            0x14 => (),
            0x15 => (),
            0x16 => (),
            0x17 => (),
            0x18 => (),
            0x19 => self.ADD(r16::DE, r16::HL, r16::HL),
            0x1A => (),
            0x1B => (),
            0x1C => (),
            28_u8..=u8::MAX => (),
            _ => (),
        }
    }
}