use rlox::{
    instr::OpPrefix,
    chunk::Chunk,
    value::Value,
};

fn main() {
    let mut chunk = Chunk::new();

    let foo = chunk.add_const(Value::numeric(1.2));
    chunk.write(OpPrefix::CONSTANT.into(), 123);
    chunk.write(foo, 123);

    chunk.write(OpPrefix::CONSTANT.into(), 125);
    chunk.write(foo, 125);

    chunk.write(OpPrefix::RETURN.into(), 125);

    chunk.write(OpPrefix::RETURN.into(), 125);

    chunk.write(12u8, 126);

    chunk.write(OpPrefix::CONSTANT.into(), 126);

    chunk.disasm_all("test chunk");
}
