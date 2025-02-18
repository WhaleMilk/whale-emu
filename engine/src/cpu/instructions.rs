use crate::cpu::Cpu;
use crate::cpu::register_page::{r16, r8};

impl Cpu {
    pub fn ld_from_addr(&mut self, to: r8, from: r16, inc: bool, dec: bool) {
        let addr = self.reg_page.read16(from);
        let val = self.bus.read_byte(addr);
        self.write(to, val);
        //increment
    }

    pub fn ld_from_imm16(&mut self, to: r16) {
        let val = self.get_immediate_16();
        self.reg_page.write16(to, val);
    }

    pub fn ld_to_addr(&mut self, to: r16, data: r8, inc: bool, dec: bool) {
        let d = self.read(data);
        let a = self.reg_page.read16(to);
        self.bus.write_byte(a, d);
        //increment
    }

    pub fn inc16(&mut self, reg: r16) {
        let val = self.reg_page.read16(reg.clone()).wrapping_add(1);
        self.reg_page.write16(reg, val);
    }

    pub fn and(&mut self, reg1: r8, reg2: r8) {
        self.reg_page.a = self.read(reg1) & self.read(reg2);
    }



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
}