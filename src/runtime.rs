use super::disasm::Disasm;
use super::minix::Minix;


pub fn runtest() {
    println!("RunTest");
}

const O_BIT:u16 = 0x0800; //xxxx100000000000
const S_BIT:u16 = 0x0080;
const Z_BIT:u16 = 0x0040;
const C_BIT:u16 = 0x0001;

pub struct OpInfo {
    pub d: u8,
    pub w: u8,
    pub reg: u8,
    pub rm: u8,
    pub imd16: u16,
}

impl OpInfo {
    pub fn new() -> Self {
        OpInfo {
            d: 0,
            w: 0,
            reg: 0,
            rm: 0,
            imd16: 0,
        }
    }
    pub fn clear(&mut self) {
        self.d = 0;
        self.w = 0;
        self.reg = 0;
        self.rm = 0;        
        self.imd16 = 0;
    }
}



pub struct Runtime<'a> {
    text: &'a [u8],
    data: [u8; 0x10000],
    regs: [u16; 12],
    sreg: u16,
    pc: u16,
    prev_pc: u16,
    debug: bool,
    disasm: Disasm,
    os: Minix,
    oi: OpInfo,

    ax: *mut u16,
    al: *mut u8,
    ah: *mut u8,
    cx: *mut u16,
    cl: *mut u8,
    ch: *mut u8,
    dx: *mut u16,
    dl: *mut u8,
    dh: *mut u8,
    bx: *mut u16,
    bl: *mut u8,
    bh: *mut u8,

    sp: *mut u16,
    bp: *mut u16,
    si: *mut u16,
    di: *mut u16,
    es: *mut u16,
    cs: *mut u16,
    ss: *mut u16,
    ds: *mut u16,
}


type LogFunc = fn(&OpInfo) -> String;

impl<'a> Runtime<'a> {

    pub fn new (text: &'a [u8]) -> Self {
        Runtime {
            text: text,
            data: [0; 0x10000],
            regs: [0; 12],
            sreg: 0,
            pc: 0,
            prev_pc: 0,
            debug: true,
            disasm: Disasm::new(),            
            oi: OpInfo::new(),
            os: Minix::new(true),

            ax: std::ptr::null_mut(),
            al: std::ptr::null_mut(),
            ah: std::ptr::null_mut(),
            
            cx: std::ptr::null_mut(),
            cl: std::ptr::null_mut(),
            ch: std::ptr::null_mut(),

            dx: std::ptr::null_mut(),
            dl: std::ptr::null_mut(),
            dh: std::ptr::null_mut(),
            
            bx: std::ptr::null_mut(),
            bl: std::ptr::null_mut(),
            bh: std::ptr::null_mut(),

            sp: std::ptr::null_mut(),
            bp: std::ptr::null_mut(),
            si: std::ptr::null_mut(),
            di: std::ptr::null_mut(),

            es: std::ptr::null_mut(),
            cs: std::ptr::null_mut(),
            ss: std::ptr::null_mut(),
            ds: std::ptr::null_mut(),
        }
    }

    pub fn init(&mut self) {
        self.ax = &mut self.regs[0] as *mut u16;
        self.al = self.ax as *mut u8;

        self.cx = &mut self.regs[1] as *mut u16;
        self.cl = self.cx as *mut u8;

        self.dx = &mut self.regs[2] as *mut u16;
        self.dl = self.dx as *mut u8;

        self.bx = &mut self.regs[3] as *mut u16;
        self.bl = self.bx as *mut u8;

        self.sp = &mut self.regs[4] as *mut u16;
        self.bp = &mut self.regs[5] as *mut u16;
        self.si = &mut self.regs[6] as *mut u16;
        self.di = &mut self.regs[7] as *mut u16;
        self.es = &mut self.regs[8] as *mut u16;
        self.cs = &mut self.regs[9] as *mut u16;
        self.ss = &mut self.regs[10] as *mut u16;
        self.ds = &mut self.regs[11] as *mut u16;
        

        unsafe {
            self.ah = self.al.offset(1);
            self.ch = self.cl.offset(1);
            self.dh = self.dl.offset(1);
            self.bh = self.bl.offset(1);
        
        }
        
    }

    pub fn load_data(&mut self, data: &[u8]) {
        //println!("data len = {}", data.len());        
        self.data[..data.len()].clone_from_slice(data);        
    }

    pub fn show(&self) {         
        println!(" AX   BX   CX   DX   SP   BP   SI   DI  FLAGS IP");
        unsafe {
            println!("{:04x} {:04x} {:04x} {:04x} {:04x} {:04x} {:04x} {:04x} {}{}{}{} {:04x}", 
            *self.ax, *self.bx, *self.cx, *self.dx, *self.sp, *self.bp, *self.si, *self.di,
            if self.o() {'O'} else {'-'},
            if self.s() {'S'} else {'-'},
            if self.z() {'Z'} else {'-'},
            if self.c() {'C'} else {'-'},
            self.pc,
        );
        }        
    }

    fn showHeader() {
        eprintln!(" AX   BX   CX   DX   SP   BP   SI   DI  FLAGS IP");
    }

    fn getRegLog(&self) -> String {
        unsafe {
            format!("{:04x} {:04x} {:04x} {:04x} {:04x} {:04x} {:04x} {:04x} {}{}{}{} {:04x}", 
            *self.ax, *self.bx, *self.cx, *self.dx, *self.sp, *self.bp, *self.si, *self.di,
            if self.o() {'O'} else {'-'},
            if self.s() {'S'} else {'-'},
            if self.z() {'Z'} else {'-'},
            if self.c() {'C'} else {'-'},
            self.prev_pc,)
        }
    }

    fn fetch(&mut self) -> u8 {
        let val = self.text[self.pc as usize];
        self.pc += 1;
        if self.debug { self.disasm.add(&val)}
        val
    }

    fn fetch2(&mut self) -> u16 {
        let (d1, d2) = (self.fetch(), self.fetch());
        (d2 as u16) << 8 | d1 as u16        
    }

    fn get_reg_w(op: u8) -> (u8, u8) {
        (op >> 3 & 1, op & 7)        
    }

    fn get_data(&mut self, w: u8) -> u16 {
        match w {
            0 => self.fetch() as u16,
            1 => self.fetch2(),
            _ => panic!("no such w: {}", w)
        }
    }

    fn write_to_reg(&mut self, reg: u8, w: u8, data: u16) {
        match w {
            0 => {
                eprintln!("not impleented 0 in writeToReg");
                std::process::exit(1)
            }
            1 => {
                self.regs[reg as usize] = data;
            }
            _ => panic!("no such w: {}", w)
        }
    }


    

    pub fn run(&mut self) -> () {
        println!("Run");
        println!("len = {}", self.text.len());

        if self.debug {
            Disasm::show_header();
        }

        
        
        //let mut callback = Disasm::show_mov;
        let mut callback: Option<LogFunc> = None;
        let mut regstatus: String = "".into();
        loop {
            //print!("{:02x} ", self.text[self.pc as usize]);
            //self.pc += 1;
            
            self.prev_pc = self.pc;
            let op = self.fetch();
            if self.debug {
                regstatus = Disasm::get_reg_state(self);
            }

            match op {
                0xbb => {
                    (self.oi.w, self.oi.reg)= Runtime::get_reg_w(op);
                    self.oi.imd16 = self.get_data(self.oi.w);
                    self.write_to_reg(self.oi.reg, self.oi.w, self.oi.imd16); // behavior
                    callback = Some(Disasm::show_mov);                   
                }
                0xcd => {
                    let tp = self.fetch();                    
                    eprintln!("{}", self.disasm.get_log(&regstatus, &Disasm::show_syscall(&self.oi)));                     
                    self.os.syscall(self.regs[3], &mut self.data);                    
                }
                _ => {
                    println!("unrecognized operator {:02x}", op);
                    std::process::exit(1);
                }
            }

            
            
            if self.debug {                
                if let Some(f) = callback {
                    eprintln!("{}", self.disasm.get_log(&regstatus, &f(&self.oi)));
                    //eprintln!("{}", f(&self.oi));            
                } 
                self.disasm.clear();
            }
            self.oi.clear();
            callback = None;
            


            if self.pc == self.text.len() as u16 {
                break;
            }
        }
        println!("");

    }



    fn f_check(val: u16, mask: u16) -> bool {
        if val & mask == 0 { false } else { true }        
    }

    pub fn c(&self) -> bool {
        Runtime::f_check(self.sreg, C_BIT)        
    }

    pub fn z(&self) -> bool {
        Runtime::f_check(self.sreg, Z_BIT)
    }

    pub fn s(&self) -> bool {
        Runtime::f_check(self.sreg, S_BIT)
    }

    pub fn o(&self) -> bool {
        Runtime::f_check(self.sreg, O_BIT)
    }

    pub fn get_regs(&self) -> &[u16; 12] {
        &self.regs
    }

    pub fn get_prev_pc(&self) -> u16 {
        self.prev_pc
    }

    


}