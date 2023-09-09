use crate::runtime::{Runtime, OpInfo};


// test
pub fn disastest() {
    println!("disastest");
}

pub struct Disasm {
    buffer: Vec<u8>,    
}

impl Disasm {
    pub fn new() -> Self {
        Disasm {
            buffer: Vec::<u8>::new()
        }
    }

    pub fn add(&mut self, vref: &u8) {
        self.buffer.push(*vref);
    }

    pub fn clear(&mut self) {
        self.buffer.clear()
    }

    pub fn get_raw(&self) -> String {        
        let result = self.buffer.iter().map(|x| format!("{:02x}", x)).fold(String::from(""), |acc, x| acc + &x); 
        let len = result.len();
        let rowbytes = result + &" ".repeat(14 - len);
        rowbytes        
    }

    pub fn show_header() {
        eprintln!(" AX   BX   CX   DX   SP   BP   SI   DI  FLAGS IP");
    }

    fn get_reg<'a> (reg: &'a u8, w: &'a u8) -> &'a str {
        match (w, reg) {
            (1, 0x00) => "ax",
            (1, 0x01) => "cx",            
            (1, 0x02) => "dx",
            (1, 0x03) => "bx",
            (1, 0x04) => "sp",
            (1, 0x05) => "bp",
            (1, 0x06) => "si",
            (1, 0x07) => "di",
            (0, 0x00) => "al",
            (0, 0x01) => "cl",
            (0, 0x02) => "dl",
            (0, 0x03) => "bl",
            (0, 0x04) => "ah",
            (0, 0x05) => "ch",
            (0, 0x06) => "dh",                    
            (0, 0x07) => "bh",
            _ => "",
        }
    }

    fn format_val16(val: &u16) -> String { format!("{:04x}", *val) }
    fn format_val8(val: &u8) -> String { format!("{:x}", *val) }

    fn to_string(w: &u8, data: &u16) -> String {
        match w {
            0 => {                
                Disasm::format_val8(&((*data & 0xff) as u8))
            }
            1 => {
                Disasm::format_val16(data)
            }
            _ => panic!("no such w")
        }
    }

    pub fn show_mov(opinfo: &OpInfo) -> String {
        let data_str = Disasm::to_string(&opinfo.w, &opinfo.imd16);
        let reg_str = Disasm::get_reg(&opinfo.reg, &opinfo.w);
        format!("{} {}, {}", "mov", reg_str, data_str)                
    }

    pub fn show_syscall(_opinfo: &OpInfo) -> String {
        "int 20".into()        
    }

    /*
    pub fn get_reg_state(ax: *const u8, bx: *const u8, cx: *const u8, dx: *const u8, 
        sp: *const u8, bp: *const u8, si: *const u8, di: *const u8,
        o: bool, s: bool, z: bool, c: bool, prev_pc: u16) -> String {
        unsafe {
            format!("{:04x} {:04x} {:04x} {:04x} {:04x} {:04x} {:04x} {:04x} {}{}{}{} {:04x}", 
                *ax, *bx, *cx, *dx, *sp, *bp, *si, *di,
                if o { "O" } else { "-" },
                if s { "S" } else { "-" },
                if z { "Z" } else { "-" },
                if c { "C" } else { "-" },
                prev_pc
            )
        }
    }
    */

    pub fn get_log(&self, reginfo: &str, asm: &str) -> String {
        //let reginfo = Disasm::get_reg_state(runtime);
        let rawinfo = self.get_raw();

        reginfo.to_string() + ":" + &rawinfo +  &" ".repeat(14 - rawinfo.len()) + asm
    }

    pub fn get_reg_state(runtime: &Runtime) -> String {
        let regs = runtime.get_regs();
        format!("{:04x} {:04x} {:04x} {:04x} {:04x} {:04x} {:04x} {:04x} {}{}{}{} {:04x}", 
            regs[0], regs[3], regs[1], regs[2], regs[4], regs[5], regs[6], regs[7],
            if runtime.o() { "O" } else { "-" },
            if runtime.s() { "S" } else { "-" },
            if runtime.z() { "Z" } else { "-" },
            if runtime.c() { "C" } else { "-" },
            runtime.get_prev_pc()
        )
    }

}