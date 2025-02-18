use whale_emu::cpu::{Cpu, register_page::r16, register_page::r8};


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

#[test]
fn test_adder_with_memory_read() {
    let mut p = Cpu::new();
    p.bus.write_byte(0, 0x80);
    p.reg_page.a = 2;
    p.reg_page.b = 3;
    p.cpuStep();
    assert_eq!(p.reg_page.a, 5);
}

#[test]
fn test_adder_with_mult_memory_read() {
    let mut p = Cpu::new();
    p.bus.write_byte(0, 0x80);
    p.bus.write_byte(1, 0x80);
    p.reg_page.a = 2;
    p.reg_page.b = 3;
    p.cpuStep();
    p.cpuStep();
    assert_eq!(p.reg_page.a, 8);
}

#[test]
fn load_basic() {
    let mut p = Cpu::new();
    p.bus.write_byte(0x50, 2);
    p.bus.write_byte(0x0, 0x0A);
    p.write(r8::C, 0x50);
    p.cpuStep();
    assert_eq!(p.reg_page.a, 2);
}

#[test]
fn load_with_add() {
    let mut p = Cpu::new();
    p.bus.write_byte(0x50, 0x02);
    p.bus.write_byte(0x0, 0x0A);
    p.bus.write_byte(1, 0x80);
    p.reg_page.c = 0x50;
    p.cpuStep(); //cpu instruction 0x0A (load info at memory location 0x30 into register a)
    p.reg_page.b = 3;
    p.cpuStep(); //0x80 (add register a and b)
    assert_eq!(p.reg_page.a, 5);
}

#[test]
fn load_to() {
    let mut p = Cpu::new();
    p.bus.write_byte(0, 0x02);
    p.reg_page.write16(r16::BC, 0x44);
    p.reg_page.a = 4;
    p.execute(0x02);
    assert_eq!(p.bus.read_byte(0x44), 4);
}