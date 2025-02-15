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

use crate::core::registers::registers;

pub struct Cpu {
    reg_page: registers,
    IME: bool,
    op_code: u16
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            reg_page: registers::new(),
            IME: false,
            op_code: 0x00
        }
    }

    pub fn cpuStep(&mut self) {
        //read instruction from self
        // 
    }
}