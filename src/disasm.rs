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

    fn get_rm(m: & u8, rm: &u8, w: &u8, reg:& u8, eaddr: &u16) -> String {
        if *m == 0 && *rm == 6 {
            return format!("[{:04x}]", eaddr);
        }

        if *m == 3 {
            return Disasm::get_reg(reg, w).to_string();
        }

        match rm {
            0 => {},
            _ => panic!("not implemented yet"),
        }

        "".to_string()
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

    pub fn show_sub(opinfo: &OpInfo) -> String {
        //let rm = format!("[{:04x}]", opinfo.eaddr); // should be replaced
        let rm = Disasm::get_rm(&opinfo.m, &opinfo.rm, &opinfo.w, &opinfo.reg, &opinfo.eaddr);
        match (opinfo.s, opinfo.w) {
            (0, 1) => format!("{} {}, {:04x}", "sub", rm, opinfo.imd16),            
            _ => format!("{} {}, {:02x}", "sub", rm, opinfo.imd16),
        }        
    }

    pub fn show_xor(opinfo: &OpInfo) -> String {
        let reg_str = Disasm::get_reg(&opinfo.reg, &opinfo.w);
        let rm_str = Disasm::get_rm(&opinfo.m, &opinfo.rm, &opinfo.w, &opinfo.reg, &opinfo.eaddr);
        let mut arg_str = format!("{}, {}", rm_str, reg_str);
        if opinfo.d == 1 {
            arg_str = format!("{}, {}", reg_str, rm_str);
        }
        format!("{} {}", "xor", arg_str)
    }

    pub fn show_syscall(_opinfo: &OpInfo) -> String {
        "int 20".into()        
    }
    
    pub fn get_log(&self, reginfo: &str, asm: &str, meminfo: &Option<String>) -> String {
        //let reginfo = Disasm::get_reg_state(runtime);
        let rawinfo = self.get_raw();

        let mut log_str = reginfo.to_string() + ":" + &rawinfo +  &" ".repeat(14 - rawinfo.len()) + asm;
        if let Some(v) = meminfo {
            log_str += v;
        }

        return log_str;
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