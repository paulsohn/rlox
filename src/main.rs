use rlox::{
    instr::OpPrefix,
    chunk::Chunk,
    value::Value,
    vm::VM,
};

fn main() {
    let mut chunk = Chunk::new();

    chunk.write_const(Value::Number(1.2), 123);
    chunk.write_const(Value::Number(3.4), 123);
    chunk.write(OpPrefix::ADD, 123);

    chunk.write_const(Value::Number(5.6), 123);
    chunk.write(OpPrefix::DIVIDE, 123);
    chunk.write(OpPrefix::NEGATE, 123);

    chunk.write(OpPrefix::RETURN, 125);

    chunk.write(66u8, 126);

    println!("------ Disassemble Result -----");
    chunk.disasm_all("test chunk");

    println!("------ VM Execution Result -----");
    let mut vm = VM::new(&mut chunk);
    vm.run();
}
