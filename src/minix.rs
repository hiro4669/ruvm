
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

    fn write2(idx: u16, val: u16, data: &mut [u8]) {
        data[idx as usize] = (val & 0xff) as u8;
        data[(idx+1) as usize] = ((val >> 8) & 0xff) as u8;
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
                let ret = self.write(idx, dmem);
                Minix::write2(bx+2, ret, dmem); // temporary
            }

            _ => panic!("not supported {}", m_type),

        }
    }

    pub fn write(&self, idx_: u16, dmem : &mut [u8]) -> u16 {
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
        let mut ret: u16 = 0xffff; // as -1
        unsafe { 
            ret = sys_write(fd, bptr, nbytes); 
            if self.debug {
                eprintln!(" => {}>", ret);
            }
        }
        return ret;
    }
}