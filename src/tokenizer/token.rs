// In every line, the parts in [] are the captured ones
pub enum Token {
    // Line prefixes
    Axiom, // "!Axiom"
    // [!Axiom] LEM: \forall A in U_0. ((A -> 0) -> 0) -> isProp A -> A;
    Builtin, // "!Builtin"
    // [!Builtin] prodCons: \forall A in U_0. \forall B in U_0. A -> B -> (A, B);
    Import, // "!Import"
    // [!Import] .is_prop

    // Import datum
    ImportPath { path: String }, // "(.[a-zA-Z0-9_]+)+"
    // !Import [.is_prop]

    // Variables
    Variable { name: String }, // A variable
    // [pr1]: \forall [A] in U_0. \forall [B] in U_0. ([A], [B]) -> [A];
    // [pr1] [A] [B] [p] = [productRec] [A] [B] [A] (\x in [A]. \y in [B]. [x]) [p];

    // Judgements
    Judgement,      // ":"
    Lambda,         // "\"
    Forall,         // "forall"
    Exists,         // "exists"
    In,             // "in"
    Comment,        // "# <...>\n" ends in end of line
    U { i: usize }, // An indexed universe, e.g. "U_0", "U_17"
    OpenParen,      // "("
    CloseParen,     // ")"
}
