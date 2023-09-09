
pub struct Minix {    
}

impl Minix {
    pub fn new() -> Self {
        Self {

        }
    }

    fn fetch2(idx: u16, data: &mut [u8]) -> u16 {
        let val1 = data[idx as usize];
        let val2 = data[(idx+1) as usize];
        (val2 as u16) << 8 | val1 as u16        
    }

    pub fn syscall(bx: u16, dmem: &mut [u8]) {
        let mut idx = bx;
        println!("bx: {:04x}", bx);
        let m_source = Minix::fetch2(idx, dmem);
        idx += 2;
        let m_type = Minix::fetch2(idx, dmem);
        idx += 2;

        println!("m_source {}, m_type {}", m_source, m_type);
        match m_type {
            4 => {                
                Minix::write(idx, dmem);
            }
            _ => panic!("not supported {}", m_type),
        }
    }

    pub fn write(idx_: u16, dmem : &mut [u8]) {        
        let mut idx = idx_;
        let fd = Minix::fetch2(idx, dmem);
        idx += 2;
        let nbytes = Minix::fetch2(idx, dmem);
        idx += 4;
        let buffer = Minix::fetch2(idx, dmem);

        println!("{}:{:04x}:{}", fd, buffer, nbytes);
        

    }
}