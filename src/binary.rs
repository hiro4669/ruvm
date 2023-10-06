use std::{fs::File, io::Read};

pub fn bintest() {
    println!("Hello BinTest");
}

pub struct Binary {
    #[allow(dead_code)]
    fname: String,
    aout: Vec<u8>,
}

impl Binary {
    pub fn new(fname: &str) -> Self {
        println!("Binary");
        println!("fname = {}", fname);
        let mut buffer: Vec<u8> = Vec::<u8>::new();

        
        if let Ok(mut file) = File::open(fname) {
            file.read_to_end(&mut buffer).expect("msg: cannot read file");            
        } else {
            println!("Failed to open {}", fname);
            std::process::exit(1);
        }
        
        /*
        for i in 0 .. buffer.len() {
            if i > 0 && i % 16 == 0 {println!("")}
            print!("{:02x} ", buffer[i]);
        }
        */
        
        
        Binary {
            fname: String::from(fname),
            aout: buffer,
        }
    }

    
    pub fn fetch4(idx: usize, data: &[u8]) -> u32{
        let v1: u32 = data[idx] as u32;
        let v2: u32 = data[idx+1] as u32;
        let v3: u32 = data[idx+2] as u32;
        let v4: u32 = data[idx+3] as u32;
        v4 | v3 | v2 | v1
    }

    // &[u8]のライフライムはselfと同じ
    //pub fn get_text(&self) -> &[u8] {
    pub fn get_text<'a>(&'a self) -> &'a [u8] {
        let length = Binary::fetch4(8, &self.aout);        
        &self.aout[0x20..0x20 + length as usize]        
    }

    pub fn get_data(&self) -> &[u8] {
        let data_len = Binary::fetch4(12, &self.aout) as usize;
        let text_len = Binary::fetch4(8, &self.aout) as usize;
        &self.aout[0x20 + text_len .. 0x20 + text_len + data_len]
        
        //println!("data_len = {}", data_len);
        //&self.aout[0..10]
    }
    
    /*
    pub fn init(&'a mut self) -> () {
        
        let length = Binary::fetch4(8, &self.aout);
        println!("len = {}", length);
        
        self.text = &self.aout[0x20..0x20 + length as usize];        
    }
    */

     pub fn show(&self) {
        println!("size = {}", self.aout.len());
        /*
        for i in 0 .. self.aout.len() {
            if i > 0 && i % 16 == 0 { println!("")}
            print!("{:02x} ", self.aout[i]);
        }
        */
    }


}