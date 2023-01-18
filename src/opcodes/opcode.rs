use crate::opcodes::opcode_input::OpcodeInput;
use crate::opcodes::opcode_labels::OpcodeLabel;

pub struct Opcode {
    pub label: OpcodeLabel,
    pub input: OpcodeInput,
}
