use ruvm::binary;
use ruvm::runtime;
use clap::Parser;

#[derive(Parser)]
#[command(name = "RMVM")]
#[command(author = "Hiroaki Fukuda <hiroaki@java2.jp>")]
#[command(version = "1.0")]
#[command(about = "8086 with Minix Interpreter")]
struct Config {  

    /// Disassemble
    #[arg(short)]
    disassemble: bool,

    /// Run with states
    #[arg(short)]
    microexec: bool,

    file: Option<String>,
}

fn main() {

    let config = Config::parse();

    println!("disasm = {:?}", config.disassemble);

    if let Some(fname) = config.file.as_deref() {
        eprintln!("fname = {}", fname);
    } else {
        eprintln!("usage:");
        std::process::exit(1);
    }

//    let mut memory = [0; 0x10000];

    //let binary: binary::Binary = binary::Binary::new("1s");
    let binary: binary::Binary = binary::Binary::new(config.file.unwrap().as_str());

    let text = binary.get_text();
    for i in 0 .. text.len() {
        if i > 0 && i % 16 == 0 {println!("");}
        print!("{:02x} ", text[i]);
    }
    println!("");
    println!("----------");

    let data = binary.get_data();
    for i in 0 .. data.len() {
        if i > 0 && i % 16 == 0 {println!("");}
        print!("{:02x} ", data[i]);
    }
    println!("");

    binary.show();
    //println!("memlen = {}", memory.len());

    if config.disassemble {
        eprintln!("not implemented yet");
        std::process::exit(1);
    } else if config.microexec {
        let mut runtime = runtime::Runtime::new(text);
        runtime.init();
        runtime.load_data(data);
        runtime.run();
    } else {
        let mut runtime = runtime::Runtime::new(text);
        runtime.init();
        runtime.load_data(data);
        runtime.run();
    }
}
