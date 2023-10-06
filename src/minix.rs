
extern "C" {
    #[allow(dead_code)]
    fn hello() -> ();
    fn sys_write(fildes: u16, buffer: *const u8, nbytes: u16) -> u16;
}

pub struct Minix {    
    debug: bool
}

impl Minix {
    pub fn new(dbg: bool) -> Self {
        Self {
            debug: dbg,
        }
    }

    fn fetch2(idx: u16, data: &mut [u8]) -> u16 {
        let val1 = data[idx as usize];
        let val2 = data[(idx+1) as usize];
        (val2 as u16) << 8 | val1 as u16        
    }

    pub fn syscall(&self, bx: u16, dmem: &mut [u8]) {
        let mut idx = bx;        
        let _m_source = Minix::fetch2(idx, dmem);
        idx += 2;
        let m_type = Minix::fetch2(idx, dmem);
        idx += 2;

        //println!("m_source {}, m_type {}", m_source, m_type);
        match m_type {
            1 => { // exit; temporary
                if self.debug {
                    eprintln!("<exit(0)>");
                }
                std::process::exit(0);
            }
            4 => {// write
                self.write(idx, dmem);
            }
            _ => panic!("not supported {}", m_type),

        }
    }

    pub fn write(&self, idx_: u16, dmem : &mut [u8]) {        
        let mut idx = idx_;
        let fd = Minix::fetch2(idx, dmem);
        idx += 2;
        let nbytes = Minix::fetch2(idx, dmem);
        idx += 4;
        let buffer = Minix::fetch2(idx, dmem);

        let bptr: *const u8 = &dmem[buffer as usize] as *const u8;

        if self.debug {
            eprint!("<write({}, 0x{:04x}, {})", fd, buffer, nbytes);
        }
        unsafe { 
            let ret = sys_write(fd, bptr, nbytes); 
            if self.debug {
                eprintln!(" => {}>", ret);
            }
        }



        
        

    }
}