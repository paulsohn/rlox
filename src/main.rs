use rlox::{
    // instr::OpPrefix,
    // chunk::Chunk,
    // value::Value,
    // vm::VM,
    scanner::Scanner,
};
use std::{
    env,
    process,
    fs
};

fn main() {
    // let mut chunk = Chunk::new();

    // chunk.write_const(Value::Number(1.2), 123);
    // chunk.write_const(Value::Number(3.4), 123);
    // chunk.write(OpPrefix::ADD, 123);

    // chunk.write_const(Value::Number(5.6), 123);
    // chunk.write(OpPrefix::DIVIDE, 123);
    // chunk.write(OpPrefix::NEGATE, 123);

    // chunk.write(OpPrefix::RETURN, 125);

    // chunk.write(66u8, 126);

    // println!("------ Disassemble Result -----");
    // chunk.disasm_all("test chunk");

    // println!("------ VM Execution Result -----");
    // let mut vm = VM::new(chunk);
    // vm.run();

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 { // no repl here
        eprintln!("Usage: rlox <path>");
        process::exit(64);
    }

    let path = &args[1];

    // read bytes
    let source = match fs::read_to_string(path) {
        Ok(source) => source,
        Err(err) => {
            eprintln!("Failed to read file \"{}\". ({})", path, err.to_string());
            process::exit(74)
        }
    };

    let scanner = Scanner::from_source(&source);

    for token in scanner {
        println!("{:?}",token);
    }

}
