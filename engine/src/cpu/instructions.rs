use crate::cpu::Cpu;
use crate::cpu::register_page::{r16, r8, Flags};

impl Cpu {
    pub fn ld_from_addr(&mut self, to: r8, from: r16, inc: bool, dec: bool) {
        let addr = self.reg_page.read16(from.clone());
        let val = self.bus.read_byte(addr);
        self.write(to, val);
        if inc {self.inc16(from);}
        else if dec {self.dec16(from);}
    }

    pub fn ld_from_imm16(&mut self, to: r16) {
        let val = self.get_immediate_16();
        self.reg_page.write16(to, val);
    }

    pub fn ld_from_imm8(&mut self, to: r8) {
        let val = self.get_immediate_8();
        self.write(to, val);
    }

    pub fn ld_a16_sp(&mut self) {
        let s = (self.reg_page.read16(r16::SP) >> 8) as u8;
        let p = (self.reg_page.read16(r16::SP) & 0xFF) as u8;
        let addr = self.get_immediate_16();
        self.bus.write_byte(addr, p);
        self.bus.write_byte(addr + 1, s);
    }

    pub fn rlc(&mut self, reg: r8) {
        let val = self.read(reg.clone());
        let carry = val & 0x80;
        let new_reg = val.rotate_left(1);

        self.reg_page.set_z(new_reg == 0);
        self.reg_page.set_n(false);
        self.reg_page.set_h(false);
        self.reg_page.set_c(carry != 0);
        self.write(reg, new_reg);
    }

    pub fn rl(&mut self, reg: r8) {
        let val = self.read(reg.clone());
        let c_flag = self.reg_page.check_c() as u8;
        let carry = val & 0x80;
        let new_reg = (val << 1) | c_flag;

        self.reg_page.set_z(new_reg == 0);
        self.reg_page.set_n(false);
        self.reg_page.set_h(false);
        self.reg_page.set_c(carry != 0);
        self.write(reg, new_reg);
    }

    pub fn rrc(&mut self, reg: r8) {
        let val = self.read(reg.clone());
        let carry = val & 0x01;
        let new_reg = val.rotate_right(1);

        self.reg_page.set_z(new_reg == 0);
        self.reg_page.set_n(false);
        self.reg_page.set_h(false);
        self.reg_page.set_c(carry != 0);
        self.write(reg, new_reg);
    }

    pub fn rr(&mut self, reg: r8) {
        let val = self.read(reg.clone());
        let c_flag = self.reg_page.check_c() as u8;
        let carry = val & 0x01;
        let new_reg = (val >> 1) | (c_flag << 7);

        self.reg_page.set_z(new_reg == 0);
        self.reg_page.set_n(false);
        self.reg_page.set_h(false);
        self.reg_page.set_c(carry != 0);
        self.write(reg, new_reg);
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

    pub fn inc(&mut self, reg: r8) {
        let val = self.read(reg.clone()).wrapping_add(1);
        self.reg_page.set_n(false);
        self.reg_page.set_z(val == 0);
        self.reg_page.set_h(val & 0xf == 0);
        self.write(reg, val);
    }

    pub fn dec16(&mut self, reg: r16) {
        let val = self.reg_page.read16(reg.clone()).wrapping_sub(1);
        self.reg_page.write16(reg, val);
    }

    pub fn dec(&mut self, reg: r8) {
        let val = self.read(reg.clone()).wrapping_sub(1);
        self.reg_page.set_n(true);
        self.reg_page.set_z(val == 0);
        self.reg_page.set_h(val & 0xf == 0);
        self.write(reg, val);
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

    pub fn jr(&mut self) {
        let s8 = self.get_immediate_8() as i8;
        let cpc = self.reg_page.pc;
        self.reg_page.pc = cpc.wrapping_add(s8 as u16);
    }

    pub fn jr_z(&mut self) {
        if(self.reg_page.check_z()) {
            self.jr();
        }
    }

    pub fn jr_c(&mut self) {
        if(self.reg_page.check_c()) {
            self.jr();
        }
    }

    pub fn stop(&self) {
        panic!("STOP");
    }
}