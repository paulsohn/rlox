use rlox::{
    instr::OpPrefix,
    chunk::Chunk,
    value::Value,
    vm::VM,
};

fn main() {
    // let mut chunk = Chunk::new();

    // let foo = chunk.add_const(Value::numeric(1.2));
    // chunk.write(OpPrefix::CONSTANT, 123);
    // chunk.write(foo, 123);

    // chunk.write(OpPrefix::CONSTANT, 125);
    // chunk.write(foo, 125);

    // chunk.write(OpPrefix::RETURN, 125);

    // chunk.write(OpPrefix::RETURN, 125);

    // chunk.write(12u8, 126);

    // chunk.write(OpPrefix::CONSTANT, 126);

    // chunk.disasm_all("test chunk");

    let mut chunk = Chunk::new();

    let foo = chunk.add_const(Value::numeric(1.2));
    chunk.write(OpPrefix::CONSTANT, 123);
    chunk.write(foo, 123);

    chunk.write(OpPrefix::RETURN, 125);

    println!("------ Disassemble Result -----");
    chunk.disasm_all("test chunk");

    println!("------ VM Execution Result -----");
    let mut vm = VM::new(&mut chunk);
    vm.run();
}
