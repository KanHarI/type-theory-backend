use std::sync::Arc;

pub enum ExpAst {
    U {
        i: usize,
    }, // The i th type universe, starting with 0
    Void,      // The empty type
    Unit,      // The unit type
    Singleton, // The singleton, element of the unit type
    Func {
        domain: Arc<ExpAst>,
        codomain: Arc<ExpAst>,
    }, // The type of functions from domain to codomain
    Pi {
        base: Arc<ExpAst>,
        body: Arc<ExpAst>,
    }, // Pi (forall) dependent type
    Sigma {
        base: Arc<ExpAst>,
        body: Arc<ExpAst>,
    }, // Sigma (exists) dependent type
    Product {
        left: Arc<ExpAst>,
        right: Arc<ExpAst>,
    }, // Product (cartesian, tuple) type
    Sum {
        left: Arc<ExpAst>,
        right: Arc<ExpAst>,
    }, // Sum (either, coproduct, disjoint union) type
    Eq {
        t: Arc<ExpAst>,
        left: Arc<ExpAst>,
        right: Arc<ExpAst>,
    }, // Equality =_t (identity) type, left =_t right
    Merely {
        t: Arc<ExpAst>,
    }, // Merely (proposition) type
    // Not included for now - will be added later. For now the natural numbers are
    // defined axiomatically instead of being a first class citizen of the W type.
    // W {
    //     base: Arc<ExpAst>,
    //     body: Arc<ExpAst>,
    // }, // W (fixpoint, well founded) type
    Nat,  // The natural numbers
    Zero, // The natural number zero
    Succ {
        pred: Arc<ExpAst>,
    }, // The successor of a natural number
}
