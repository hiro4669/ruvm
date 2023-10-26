use super::disasm::Disasm;
use super::minix::Minix;


pub fn runtest() {
    println!("RunTest");
}

const O_BIT:u16 = 0x0800; //xxxx100000000000
const S_BIT:u16 = 0x0080;
const Z_BIT:u16 = 0x0040;
const C_BIT:u16 = 0x0001;

const AX: u8 = 0;
const CX: u8 = 1;
const DX: u8 = 2;
const BX: u8 = 3;
const SP: u8 = 4;
const BP: u8 = 5;
const SI: u8 = 6;
const DI: u8 = 7;

pub struct OpInfo {
    pub d: u8,
    pub s: u8,
    pub w: u8,
    pub m: u8,
    pub reg: u8,
    pub rm: u8,
    pub imd16: u16, // immediate data
    pub eaddr: u16, // effective address
    pub disp: i16,  // displacement
    pub jpc: u16,   // program counter for jump instruction

    pub memval: Option<String> // for -m mode

}

impl OpInfo {
    pub fn new() -> Self {
        OpInfo {
            d: 0,
            s: 0,
            w: 0,
            m: 0,
            reg: 0,
            rm: 0,
            imd16: 0,
            eaddr: 0,
            disp: 0,
            jpc: 0,
            memval: None,            
            
        }
    }
    pub fn clear(&mut self) {
        self.d = 0;
        self.s = 0;
        self.w = 0;
        self.m = 0;
        self.reg = 0;
        self.rm = 0;        
        self.imd16 = 0;
        self.eaddr = 0;
        self.disp = 0;
        self.jpc = 0;
        self.memval = None;
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
        
             // for test
            //*self.sp = 0xffe0;
        }
        
    }
    
    fn get_reg(&self, reg:u8) -> u16 {
        return self.regs[reg as usize];
    }

    fn set_reg(&mut self, reg:u8, val: u16) {
        self.regs[reg as usize] = val;
    }

    fn set_sp(&mut self, val: u16) {        
        self.set_reg(SP, val);
    }

    fn get_sp(&self) -> u16 {
        self.get_reg(SP)
    }

    fn push_byte(&mut self, val: u8) {
        self.dec_sp();
        self.data[self.get_sp() as usize] = val;
    }
    pub fn dec_sp(&mut self) {
        self.set_sp(self.get_sp().wrapping_sub(1));
    }

    pub fn inc_sp(&mut self) {
        self.set_sp(self.get_sp().wrapping_add(1));
    }

    pub fn load_data(&mut self, data: &[u8]) {
        //println!("data len = {}", data.len());        
        self.data[..data.len()].clone_from_slice(data);        
    }

    pub fn init_stack(&mut self, args: &[String]) {
        let psize: u16 = 2; // size of pointer;
        let env = "PATH=/usr:/usr/bin";
        let mut frame_size:u16 = 0;

        let mut offset = 0;
        // for args
        for arg in args {
            frame_size += arg.len() as u16 + 1;
            frame_size += psize;
            offset += psize;            
        }

        // for env
        frame_size += env.len() as u16 + 1;
        frame_size += psize;
        offset += psize;
        
        // for null pointer and size of argument
        frame_size += psize + psize + 2;
        offset += psize + psize + 2;

        // alignment
        if frame_size % 2 == 1 { frame_size += 1}

        //println!("frame_size = {:04x}", frame_size);                
        //let nsp = self.data.len() - frame_size as usize;
        //self.set_sp(nsp as u16); // initial stack pointer
        self.set_sp((self.data.len() - frame_size as usize) as u16);

        let nsp = self.get_sp() as usize;
        
        let mut vp = nsp + 2;
        let mut strp: usize = nsp + offset as usize;

        println!("nsp = {:04x}", nsp);
        println!("vp  = {:04x}", vp);
        println!("strp= {:04x}", strp);

        // push arg number
        let arg_size = args.len() as u16;
        self.data[nsp] = (arg_size & 0xff) as u8;
        self.data[nsp + 1] = (arg_size >> 8 & 0xff) as u8;

        // push arguments
        for arg in args {
            let ary = arg.as_bytes();
            self.data[vp]   = (strp & 0xff) as u8;
            self.data[vp+1] = (strp >> 8 & 0xff) as u8;
            for b in ary {
                self.data[strp] = *b;
                strp += 1;
            }
            self.data[strp] = 0; //
            strp += 1;
            vp += 2;
            (self.data[vp], self.data[vp+1]) = (0, 0);
            vp += 2;
        }

        // push environment
        let ary = env.as_bytes();
        self.data[vp]   = (strp & 0xff) as u8;
        self.data[vp+1] = (strp >> 8 & 0xff) as u8;
        for b in ary {
            self.data[strp] += *b;
            strp += 1;
        }
        self.data[strp] = 0;

        /*
        println!("--- stack info ---");        
        for i in self.get_sp() ..= 0xffff {
            if i % 16 == 0 {println!()}
            print!("{:02x} ", self.data[i as usize]);
        }
        println!("\n-----------------");
        */
        

        //std::process::exit(1);
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

    /*
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
    */

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

    fn get_mod_reg_rm(arg: u8) -> (u8, u8, u8) {        
        (arg >> 6 & 3, arg >> 3 & 7, arg & 7)           
    }

    fn get_sw(arg: u8) -> (u8, u8) {
        (arg >> 1 & 1, arg & 1)
    }

    fn get_dw(arg: u8) -> (u8, u8) {
        (arg >> 1 & 1, arg & 1)
    }

    fn get_w(arg: u8) -> u8 {
        arg & 1
    }

    fn get_eaddr(&mut self) -> &mut Runtime<'a> {        
        if self.oi.m == 0 && self.oi.rm == 6 {
            self.oi.eaddr = self.fetch2();
            return self;
        }

        //println!("mod = {}, rm = {}", self.oi.m, self.oi.rm);
        
        // decide displacement
        match self.oi.m {
            0 => { // disp == 0
                // do nothing
                //eprintln!("m == 0 -> displacement is zero");
            }
            1 => {
                self.oi.disp = (self.fetch() as i8) as i16;
                //println!("disp = {}", self.oi.disp);
                //8d5702
                //10001101 01 010 111 02 --- m=1 reg=2 rm=7
            }
            2 => {
                panic!("not yet implemented");
            }
            3 => {
                self.oi.eaddr = self.oi.rm as u16;               
                return self;
            }            
            _ => panic!("no such pattern in get_eaddr"),
        }
        
        match self.oi.rm {
            7 => {                
                unsafe {
                    //let tad: i32 = *self.bx as i32;
                    //self.oi.eaddr = (tad + disp as i32) as u16;                    
                    self.oi.eaddr = (*self.bx as i32 + self.oi.disp as i32) as u16;                    
                }
                //println!("eader = {:04x}", self.oi.eaddr);
            },
            _ => panic!("not implemented yet"),
        }
        


        
        self
    }

    fn get_data(&mut self, w: u8) -> u16 {       
        match w {
            0 => { self.oi.imd16 = self.fetch() as u16; }
            1 => { self.oi.imd16 = self.fetch2(); }
            _ => panic!("no such w: {}", w)
        }
        self.oi.imd16
    }

    fn get_data_sw(&mut self) -> u16 {
        match (self.oi.s, self.oi.w) {
            (0, 1) => self.get_data(1),
            _ => self.get_data(0),
        }
    }

    fn read_register(&self, reg: u8) -> u16 {
        match self.oi.w {            
            0 => {
                unsafe {
                    match reg {
                        0 => { return *self.al as u16; }
                        1 => { return *self.cl as u16; }
                        2 => { return *self.dl as u16; }
                        3 => { return *self.bl as u16; }
                        4 => { return *self.ah as u16; }
                        5 => { return *self.ch as u16; }
                        6 => { return *self.dh as u16; }
                        7 => { return *self.bh as u16; }
                        _ => panic!("not supported"),
                    }
                }
            }
            1 => {
                return self.regs[reg as usize];
            }
            _ => panic!(),
        }        
    }

    fn add_meminfo(&mut self, val: u16) {
        if self.oi.w == 0 {
            panic!("not implemented yet");
        } else if self.oi.w == 1 {
            let memval_str = format!(" ;[{:04x}]{:04x}", self.oi.eaddr, val);
            self.oi.memval = Some(memval_str);
        } else {
            panic!("no such w");
        }
    }

    fn read_memory(&self, addr: u16) -> u16 {
        if self.oi.w == 0 {
            panic!("not implemented yet");
        } else if self.oi.w == 1 {
            let val:u16 = (self.data[addr as usize] as u16 | (self.data[addr as usize + 1] as u16) << 8) as u16;
            return val;
        } else {
            panic!("no such w");
        }
    }

    fn read_effective(&mut self) -> u16 {
        
        match self.oi.m {
            0 ..= 2 => {
                let val = self.read_memory(self.oi.eaddr);
                if self.debug {
                    self.add_meminfo(val);
                }
                return val;
            }
            3 => {
                return self.read_register(self.oi.eaddr as u8);
            }
            _ => panic!("impossible"),
        }

        /*
        if self.oi.m == 0 && self.oi.rm == 6 {            
            match self.oi.w {
                0 => {
                    panic!("not confirmed yet");                    
                    //return self.data[self.oi.eaddr as usize] as u16;
                }
                1 => {
                    // [Todo] create a useful function
                    let val = (self.data[self.oi.eaddr as usize] as u16 
                        | (self.data[self.oi.eaddr as usize + 1] as u16) << 8) as u16;
                    if self.debug {                        
                        let memval_str = format!(" ;[{:04x}]{:04x}", self.oi.eaddr, val);
                        self.oi.memval = Some(memval_str);
                    }
                    return val;
                }
                _ => panic!("impossible w"),
            }
        }

        match self.oi.m {
            0 => {
                panic!("not implemented yet");
            }
            1 => {
                panic!("not implemented yet");
            }
            2 => {
                panic!("not implemented yet");
            }
            3 => {
                //println!("read as register");
                return self.read_register(self.oi.eaddr as u8);
                

            }
            _ => panic!("impossible"),
        }
        */        
    }


    fn write_effective(&mut self, val: u16) {
        if self.oi.m == 3 {
            self.write_register(self.oi.eaddr as u8, self.oi.w, val);
        }
        match self.oi.w {
            0 => {
                self.data[self.oi.eaddr as usize] = (val & 0xff) as u8;                
            }
            1 => {
                self.data[self.oi.eaddr as usize] = (val & 0xff) as u8;
                self.data[self.oi.eaddr as usize + 1] = ((val >> 8) & 0xff) as u8;

            }
            _ => panic!("impossible"),
        }
        
    }

    fn write_register(&mut self, reg: u8, w: u8, data: u16) {
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


    
    fn get_dwmrrm(&mut self, op: &u8) {
        (self.oi.d, self.oi.w) = Runtime::get_dw(*op);
        (self.oi.m, self.oi.reg, self.oi.rm) = Runtime::get_mod_reg_rm(self.fetch());
    }

    fn get_mrrm(&mut self) {
        (self.oi.m, self.oi.reg, self.oi.rm) = Runtime::get_mod_reg_rm(self.fetch());
    }

    fn calc_disp(&self, addr: i16) -> u16 {
        let disp = addr as i32;
        let tmp = self.pc as i32 + disp;
        (tmp & 0xffff) as u16        
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
                0x00 ..= 0x03 => { // add
                    self.get_dwmrrm(&op);
                    let eval = self.get_eaddr().read_effective();
                    let rval = self.read_register(self.oi.reg);
                    //println!("eval: {:04x}, rval: {:04x}", eval, rval);
                    match op {
                        0 => { // add r/m r8
                            panic!("not implemented yet");
                        }
                        1 => { // add r/m r16                        
                            let c_flg = eval.overflowing_add(rval).1; // carry flg
                            let (val, o_flg) = (eval as i16).overflowing_add(rval as i16);
                            self.write_effective(val as u16);                            
                            self.set_flags(c_flg, val == 0, val < 0, o_flg);                            
                        }
                        2 => { // add r8 r/m
                            panic!("not implemented yet");
                        }
                        3 => { // add r16 r/m
                            panic!("not implemented yet");
                        }
                        _ => panic!("no such op in add"),
                    }

                    callback = Some(Disasm::show_add);                    
                    
                }
                0x30 ..= 0x33 => {// xor
                    self.get_dwmrrm(&op);
                    let eval = self.get_eaddr().read_effective();
                    let rval = self.read_register(self.oi.reg);

                    let val16 = eval ^ rval;                    
                    match self.oi.d {
                        0 => { // to r/m
                            //println!("to r/m");
                            self.write_effective(val16);
                        },
                        1 => { // to register                                                        
                            panic!("to register not implemented yet");
                        },
                        _ => panic!("impossible"),
                    }

                    match self.oi.w { // for set flag
                        0 => {
                            let ival8 = (val16 & 0xff) as i8;
                            // self.set_flags...
                            panic!("not implemented yet ");
                        }
                        1 => {
                            let ival16 = val16 as i16;
                            self.set_flags(false, ival16 == 0, ival16 < 0, false);
                        }
                        _ => panic!("impossible"),
                    }
                    callback = Some(Disasm::show_xor);                    
                }
                0x73 => { // jnb
                    let disp = self.fetch();                    
                    self.oi.jpc = self.calc_disp((disp as i8) as i16);
                    //println!("jpc = {:04x}", self.oi.jpc);
                    if self.c() == false {
                        self.pc = self.oi.jpc;
                    }

                    callback = Some(Disasm::show_jnb);

                    
                }
                0x80 ..= 0x83 => { // sub
                    (self.oi.s, self.oi.w) = Runtime::get_sw(op);
                    (self.oi.m, self.oi.reg, self.oi.rm) = Runtime::get_mod_reg_rm(self.fetch());                    
                    let dst = self.get_eaddr().read_effective();
                    self.get_data_sw(); // read imdata
                    
                    match op {
                        0x80 => {
                            panic!("not yet implemented");
                        }
                        0x81 => { // imd16
                            let c_flg = dst.overflowing_sub(self.oi.imd16).1;                    
                            let (val, o_flg) = (dst as i16).overflowing_sub(self.oi.imd16 as i16); 
                            self.set_flags(c_flg, val == 0, val < 0, o_flg);                            
                            
                            match self.oi.reg {
                                5 => {
                                    self.write_effective(val as u16);
                                    callback = Some(Disasm::show_sub);
                                }
                                7 => {
                                    callback = Some(Disasm::show_cmp);
                                }
                                _ => panic!("no such reg pattern"),
                            }
                        }
                        0x82 => panic!("no such op"),
                        0x83 => {
                            panic!("not yet implemented");
                        }
                        _ => panic!("no such pattern"),
                    }

                }
                0x88 ..= 0x8b => { // mov
                    self.get_dwmrrm(&op);                    
                    let eval = self.get_eaddr().read_effective();
                    let rval = self.read_register(self.oi.reg);
                    match self.oi.d {
                        0 => { // to r/m
                            //println!("to r/m"); 
                            self.write_effective(rval);
                        },
                        1 => { // to register                                                         
                            self.write_register(self.oi.reg, self.oi.w, eval);
                        },
                        _ => panic!("impossible"),
                    }
                    callback = Some(Disasm::show_mov2);

                }
                0x8d => { // lea
                    self.get_mrrm();
                    self.get_eaddr();                    
                    self.oi.w = 1;
                    self.write_register(self.oi.reg, self.oi.w, self.oi.eaddr);
                    if self.debug {                                                
                        self.add_meminfo(self.read_memory(self.oi.eaddr));
                    }
                    callback = Some(Disasm::show_lea);
                                    
                }
                0xbb => { // mov
                    (self.oi.w, self.oi.reg)= Runtime::get_reg_w(op);
                    self.get_data(self.oi.w);
                    self.write_register(self.oi.reg, self.oi.w, self.oi.imd16); // behavior
                    
                    callback = Some(Disasm::show_mov1);                   
                }
                0xcd => {
                    let _tp = self.fetch();                    
                    eprintln!("{}", self.disasm.get_log(&regstatus, &Disasm::show_syscall(&self.oi), &self.oi.memval));                     
                    self.os.syscall(self.regs[3], &mut self.data);                    
                }
                0xf6 ..= 0xf7 => { // test
                    self.oi.w = Runtime::get_w(op);
                    self.get_mrrm();
            
                    match self.oi.reg {
                        0 => { // test
                            let data = self.get_data(self.oi.w);
                            let edata = self.get_eaddr().read_effective();
                            if self.oi.w == 0 {
                                let val: u8 = (data & 0xff) as u8 & (edata & 0xff) as u8;
                                self.set_flags(false, val == 0, (val as i8) < 0 , false);
                            } else {
                                let val: u16 = data & edata;
                                self.set_flags(false, val == 0, (val as i16) < 0 , false);
                            }
                        }
                        _ => panic!("no such pattern"),
                    }

                    callback = Some(Disasm::show_test);
                }
                _ => {
                    println!("unrecognized operator {:02x}", op);
                    std::process::exit(1);
                }
            }

            
            
            if self.debug {                
                if let Some(f) = callback {
                    eprintln!("{}", self.disasm.get_log(&regstatus, &f(&self.oi), &self.oi.memval));
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

    fn set_flags(&mut self, c: bool, z: bool, s: bool, o: bool) {
        if c { self.sreg |=  C_BIT; } else { self.sreg &= !C_BIT}
        if z { self.sreg |=  Z_BIT; } else { self.sreg &= !Z_BIT}
        if s { self.sreg |=  S_BIT; } else { self.sreg &= !S_BIT}
        if o { self.sreg |=  O_BIT; } else { self.sreg &= !O_BIT}
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