use crate::cpu::Cpu;
use crate::cpu::register_page::{r8, r16};

impl Cpu{
    pub fn execute(&mut self, byte: u8) -> u16{
        match byte {
            0x00 => (),
            0x01 => self.ld_from_imm16(r16::BC),
            0x02 => self.ld_to_addr(r16::BC, r8::A, false, false), 
            0x03 => self.inc16(r16::BC),
            0x04 => self.inc(r8::B),
            0x05 => self.dec(r8::B),
            0x06 => self.ld_from_imm8(r8::B),
            0x07 => self.rlc(r8::A),
            0x08 => self.ld_a16_sp(),
            0x09 => self.add16(r16::BC, r16::HL),
            0x0A => self.ld_from_addr(r8::A, r16::BC, false, false),
            0x0B => self.dec16(r16::BC),
            0x0C => self.inc(r8::C),
            0x0D => self.dec(r8::C),
            0x0E => self.ld_from_imm8(r8::C),
            0x0F => self.rrc(r8::A),
            0x10 => self.stop(), //STOP
            0x11 => self.ld_from_imm16(r16::DE),
            0x12 => self.ld_to_addr(r16::DE, r8::A, false, false),
            0x13 => self.inc16(r16::DE),
            0x14 => self.inc(r8::D),
            0x15 => self.dec(r8::D),
            0x16 => self.ld_from_imm8(r8::D),
            0x17 => self.rl(r8::A),
            0x18 => self.jr(), //JR s8
            0x19 => self.add16(r16::DE, r16::HL),
            0x1A => self.ld_from_addr(r8::A, r16::DE, false, false),
            0x1B => self.dec16(r16::DE),
            0x1C => self.inc(r8::E),
            0x1D => self.dec(r8::E),
            0x1E => self.ld_from_imm8(r8::E),
            0x1F => self.rr(r8::A),
            0x20 => self.jr_z(),
            0x21 => self.ld_from_imm16(r16::HL),
            0x22 => self.ld_from_addr(r8::A, r16::DE, true, false),
            0x23 => self.inc16(r16::HL),
            0x24 => self.inc(r8::H),
            0x25 => self.dec(r8::H),
            0x26 => self.ld_from_imm8(r8::H),
            0x27 => (),
            0x28 => (),
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
