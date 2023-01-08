use std::sync::Arc;

pub enum ExpAst {
    U {
        i: usize,
    }, // The i th type universe
    Void,      // The empty type
    Unit,      // The unit type
    Singleton, // The singleton, element of the unit type
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
    }, // Equality =_t (identity) type
    W {
        base: Arc<ExpAst>,
        body: Arc<ExpAst>,
    }, // W (fixpoint, well founded) type
}
