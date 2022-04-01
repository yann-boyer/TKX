#[derive(Copy, Clone)]
pub enum Opcodes {
    IncPtr,
    DecPtr,
    IncByte,
    DecByte,
    WriteByte,
    ReadByte,
    LoopStart,
    LoopEnd
}