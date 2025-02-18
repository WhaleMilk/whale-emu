/*
 * What needs to happen:
 * CPU needs initialize itself with PC to 0
 * CPU creates a register page
 * CPU creates an op code variable
 * CPU creates variable that tracks step
 * CPU creates variable that records IME (interrupt master enable)
 * starts waiting for call to tick
 * Tick function 
*/

use crate::cpu::register_page::{r16, r8, RegisterPage};
use crate::Context;

mod instructions;
pub mod register_page;

pub trait CpuContext: Context {
    fn write_cycle_interrupt(&mut self, addr: u16, val: u8);
    fn tick_cycle(&mut self);
    fn has_interrupt(&mut self) -> bool;
}

pub struct Cpu {
    reg_page: RegisterPage,
    IME: bool,
    op_code: u8
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            reg_page: RegisterPage::new(),
            IME: false,
            op_code: 0x00
        }
    }

    pub fn get_regPage(&mut self) -> &mut RegisterPage {
        return &mut self.reg_page
    }

    pub fn cpuStep(&mut self) {
        self.execute(self.op_code);
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_adder_1() {
        let mut p = Cpu::new();
        p.reg_page.write16(r16::HL, 3);
        p.reg_page.write16(r16::BC, 2);
        p.execute(0x09);
        assert_eq!(p.reg_page.read16(r16::HL), 5);
    }

    #[test]
    fn test_adder_2(){
        let mut p = Cpu::new();
        p.reg_page.write16(r16::HL, 3);
        p.reg_page.write16(r16::DE, 2);
        p.execute(0x19);
        assert_eq!(p.reg_page.read16(r16::HL), 5);
    }
}