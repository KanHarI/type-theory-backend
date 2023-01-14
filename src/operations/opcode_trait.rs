use crate::operations::opcode_labels::OpcodeLabel;

pub trait OpcodeTrait {
    fn opcode_label(&self) -> OpcodeLabel;
}
