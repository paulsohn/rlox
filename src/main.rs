use rlox::{
    op::OpCode,
    chunk::Chunk,
    value::Value,
};

fn main() {
    let mut chunk = Chunk::new();

    let foo = chunk.add_const(Value::numeric(1.2));
    chunk.write(OpCode::CONSTANT.into(), 123);
    chunk.write(foo, 123);

    chunk.write(OpCode::RETURN.into(), 123);

    chunk.disasm_all("test chunk");
}
