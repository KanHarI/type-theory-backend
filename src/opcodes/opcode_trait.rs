use crate::opcodes::opcode_labels::OpcodeLabel;

pub trait OpcodeTrait {
    fn opcode_label(&self) -> OpcodeLabel;
    fn opcode_mnemonic(&self) -> String;
}
