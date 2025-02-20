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

use register_page::Flags;

use crate::cpu::register_page::{r16, r8, RegisterPage};
use crate::Context;

mod lookup;
mod instructions;
pub mod register_page;

pub trait CpuContext: Context {
    fn write_cycle_interrupt(&mut self, addr: u16, val: u8);
    fn tick_cycle(&mut self);
    fn has_interrupt(&mut self) -> bool;
}

pub struct MemoryBus {
    memory: [u8; 0xFFFF]
}

impl MemoryBus {
    fn new() -> MemoryBus {
        MemoryBus {
            memory: [0; 0xFFFF]
        }
    }
    pub fn read_byte(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }
    pub fn write_byte(&mut self, addr: u16, val: u8) {
        self.memory[addr as usize] = val;
    }
}

pub struct Cpu {
    pub reg_page: RegisterPage,
    pub bus: MemoryBus,
    pub op_code: u8
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            reg_page: RegisterPage::new(),
            bus: MemoryBus::new(),
            op_code: 0x0
        }
    }

    pub fn cpuStep(&mut self) {
        let mut instr = self.bus.read_byte(self.reg_page.pc);
        let cb = instr == 0xCB;
        if cb {
            instr = self.bus.read_byte(self.reg_page.pc + 1);
        }

        let next_pc = self.execute(instr);
        self.reg_page.pc = next_pc;
    }

    pub fn get_regPage(&mut self) -> &mut RegisterPage {
        return &mut self.reg_page
    }

    fn get_immediate_8 (&mut self) -> u8 {
        let addr = self.reg_page.pc;
        self.reg_page.pc = self.reg_page.pc.wrapping_add(1);
        self.bus.read_byte(addr)
    }

    fn get_immediate_16 (&mut self) -> u16 {
        let bot = self.get_immediate_8();
        let top = self.get_immediate_8();
        u16::from_le_bytes([bot, top])
    }

    pub fn read(&self, reg: r8) -> u8 {
        use self::r8::*;
        match reg {
            A => self.reg_page.a,
            B => self.reg_page.b,
            C => self.reg_page.c,
            D => self.reg_page.d,
            E => self.reg_page.e,
            F => self.reg_page.f.bits(),
            H => self.reg_page.h,
            L => self.reg_page.l,
        }
    }

    pub fn write(&mut self, reg: r8, val: u8) {
        use self::r8::*;
        match reg {
            A => self.reg_page.a = val,
            B => self.reg_page.b = val,
            C => self.reg_page.c = val,
            D => self.reg_page.d = val,
            E => self.reg_page.e = val,
            F => self.reg_page.f = Flags::from_bits_truncate(val as u8),
            H => self.reg_page.h = val,
            L => self.reg_page.l = val,
        }
    } 
}