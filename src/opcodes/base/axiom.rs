use crate::opcodes::opcode_labels::OpcodeLabel;
use crate::opcodes::opcode_trait::OpcodeTrait;

pub struct Axiom {}

impl OpcodeTrait for Axiom {
    fn opcode_label(&self) -> OpcodeLabel {
        OpcodeLabel::Axiom
    }

    fn opcode_mnemonic(&self) -> String {
        "AXIOM".to_string()
    }
}
