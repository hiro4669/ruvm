
pub fn runtest() {
    println!("RunTest");
}


pub struct Runtime<'a> {
    text: &'a [u8],
    data: [u8; 0x10000],
    regs: [u16; 12],
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

impl<'a> Runtime<'a> {

    pub fn new (text: &'a [u8]) -> Self {
        Runtime {
            text: text,
            data: [0; 0x10000],
            regs: [0; 12],
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
        println!("data len = {}", data.len());
        self.data[..data.len()].clone_from_slice(data);

        /*
        for i in 0..data.len() + 10 {
            if i > 0 && i % 16 == 0 {println!("")}
            print!("{:02x} ", self.data[i]);
        }
        println!("");
        */
    }

    pub fn show(&self) { 
        println!(" AX   BX   CX   DX   SP   BP   SI   DI  FLAGS IP");
        unsafe {
            println!("{:04x} {:04x}", *self.ax, *self.cx);
        }
    }

}