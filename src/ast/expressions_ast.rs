use std::sync::Arc;

pub enum ExpAst {
    U {
        i: usize,
    }, // The i th type universe, starting with 0
    Void,      // The empty type
    Unit,      // The unit type
    Singleton, // The singleton, element of the unit type
    Lambda {
        base: Arc<ExpAst>,
        body: Arc<ExpAst>,
    }, // Lambda expression
    Pi {
        base: Arc<ExpAst>,
        body: Arc<ExpAst>,
    }, // Pi (forall, function, dependent function) dependent type
    Sigma {
        base: Arc<ExpAst>,
        body: Arc<ExpAst>,
    }, // Sigma (exists, tuple, dependent pair) dependent type
    Sum {
        left: Arc<ExpAst>,
        right: Arc<ExpAst>,
    }, // Sum (either, coproduct, disjoint union) type
    Eq {
        t: Arc<ExpAst>,
        left: Arc<ExpAst>,
        right: Arc<ExpAst>,
    }, // Equality =_t (identity) type, left =_t right, judgmental equality
    Nat,       // The natural numbers
    Zero,      // The natural number 0
    Succ {
        n: Arc<ExpAst>,
    }, // Natural number, The successor of n
    IndSigma {
        base: Arc<ExpAst>,
        body: Arc<ExpAst>,
        result_type: Arc<ExpAst>,
        result_value: Arc<ExpAst>,
        input: Arc<ExpAst>,
    }, // Induction principle for Sigma types
    IndSum {
        left: Arc<ExpAst>,
        right: Arc<ExpAst>,
        result_type: Arc<ExpAst>,
        result_value_left: Arc<ExpAst>,
        result_value_right: Arc<ExpAst>,
        input: Arc<ExpAst>,
    }, // Induction on the sum type
    IndNat {
        result_type: Arc<ExpAst>,
        c_0: Arc<ExpAst>,
        c_s: Arc<ExpAst>,
        input: Arc<ExpAst>,
    }, // Induction on the natural numbers
    IndEq {
        base_type: Arc<ExpAst>,
        result_type: Arc<ExpAst>,
        result_value: Arc<ExpAst>,
        source: Arc<ExpAst>,
        dest: Arc<ExpAst>,
        input: Arc<ExpAst>,
    }, // Induction principle for equality
    IndVoid {
        result_type: Arc<ExpAst>,
        result_value: Arc<ExpAst>,
    }, // Induction principle for the empty type
    IndUnit {
        result_type: Arc<ExpAst>,
        result_value: Arc<ExpAst>,
        input: Arc<ExpAst>,
    }, // Induction principle for the unit type
    Pr1 {
        input: Arc<ExpAst>,
    },
    Pr2 {
        input: Arc<ExpAst>,
    }, // Pr1 and Pr2 are the projections of Sigma types
    Inl {
        input: Arc<ExpAst>,
    },
    Inr {
        input: Arc<ExpAst>,
    }, // Inl and Inr are the injections of Sum types
    Refl {
        input: Arc<ExpAst>,
    }, // Refl is the identity constructor for equality
    High {
        input: Arc<ExpAst>,
    }, // Highlighted subexpression
    DefEq {
        left: Arc<ExpAst>,
        right: Arc<ExpAst>,
    }, // Definitional equality left === right
       // W {
       //     base: Arc<ExpAst>,
       //     body: Arc<ExpAst>,
       // }, // W (fixpoint, well founded) type
}
