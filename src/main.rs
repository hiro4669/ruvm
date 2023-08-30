use ruvm::binary;
//use ruvm::runtime::runtest;
use ruvm::runtime;

fn main() {
    /*
    println!("Hello, world!");
    binary::bintest();
    runtest();
    */
    let mut memory = [0; 0x10000];

    let binary: binary::Binary = binary::Binary::new("1s");

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
    println!("memlen = {}", memory.len());
    
    let mut runtime = runtime::Runtime::new(text);
    runtime.init();
    runtime.load_data(data);
    runtime.run();
    //runtime.show();

}
