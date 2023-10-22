use ruvm::binary;
use ruvm::runtime;
use clap::Parser;
use ruvm::runtime::Runtime;

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

    //file: Option<String>,

    args: Vec<String>,
}

/*
fn init_stack(args: &[String], runtime:&mut Runtime) {
    println!("args len = {}", args.len());
    runtime.dec_sp();

    for arg in args {
        println!("{}", arg);
        //let ary = arg.clone().into_bytes();
        let ary = arg.as_bytes();
        //let ary = arg.into_bytes();
        
        for b in ary {
            print!("{:02x} ", *b);
        }
        
    }


    // test
    std::process::exit(1);
}
*/

fn main() {

    let config = Config::parse();

    println!("disasm = {:?}", config.disassemble);
    println!("arg length = {}", config.args.len());

    if config.args.len() == 0 {
        eprintln!("usage:");
        std::process::exit(1);
    }
    let fname = config.args.get(0);
    let binary: binary::Binary = binary::Binary::new(fname.unwrap().as_str());

    
    

    /*
    if let Some(fname) = config.file.as_deref() {
        eprintln!("fname = {}", fname);
    } else {
        eprintln!("usage:");
        std::process::exit(1);
    }
    */




    //let binary: binary::Binary = binary::Binary::new("1s");
    //let binary: binary::Binary = binary::Binary::new(config.file.unwrap().as_str());

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
        eprintln!("-m option");
        let mut runtime = runtime::Runtime::new(text);
        runtime.init();
        runtime.load_data(data);        
        runtime.init_stack(&config.args);

        runtime.run();
    } else {
        let mut runtime = runtime::Runtime::new(text);
        runtime.init();
        runtime.load_data(data);
        runtime.init_stack(&config.args);
        

        runtime.run();
    }
}
