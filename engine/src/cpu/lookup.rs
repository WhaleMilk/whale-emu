use crate::cpu::Cpu;
use crate::cpu::register_page::{r8, r16};

impl Cpu{
    pub fn execute(&mut self, byte: u8) -> u16{
        match byte {
            0x00 => (),
            0x01 => self.ld_from_imm16(r16::BC),
            0x02 => self.ld_to_addr(r16::BC, r8::A, false, false), 
            0x03 => self.inc16(r16::BC),
            0x04 => (),
            0x05 => (),
            0x06 => (),
            0x07 => (),
            0x08 => (),
            0x09 => self.add16(r16::BC, r16::HL),
            0x0A => self.ld_from_addr(r8::A, r16::BC, false, false),
            0x0B..0x19 => (),
            0x19 => self.add16(r16::DE, r16::HL),
            0x1A..0x29 => (),
            0x29 => self.add16(r16::HL, r16::HL),
            0x2A..0x39 => (),
            0x39 => self.add16(r16::SP, r16::HL),
            0x3A..0x80 => (),
            0x80 => self.add(self.read(r8::B)),
            0x80..=u8::MAX => (),
            _ => (),
        }
        self.reg_page.pc.wrapping_add(1)
    }
}
