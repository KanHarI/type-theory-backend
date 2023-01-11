pub enum Operations {
    // Define a new axiom; Populates the type representing the axiom with a new constant
    Axiom,

    // Creates a new empty context
    CtxEmp,

    // Extends the context with a type definition
    // [Context] |- A: U => [Context, x: A]
    CtxExt,

    // Create a new variable from a context definition
    // [Context, a: X] => [Context, a: X] |- a: X
    Vble,


    // Substitution
    // [Context1] |- a: A, [Context1, x: A, Context2] |- b: B => [Context1, Context2[a/x]] |- b[a/x]: B[a/x]
    Subst,

    // Weakening of a structure (probably not necessary in a DAG context formulation?)
    // [Context1] |- A: U, [Context1, Context2] |- b: B => [Context1, x: A, Context2] |- b: B
    Wkg,

    // Introduction of a new universe
    // [Context] => [Context] |- U_i: U_(i+1)
    UIntro,

    // Type comulation from the ith to the (i+1)th universe
    // [Context] |- A: U_i => [Context] |- A: U_(i+1)
    UCumul,

    // Form a dependent function type
    // [Context] |- A: U, [Context, x: A] |- B: U => [Context] |- forall x: A. B: U
    PiForm,

    // Creates an element of a dependent function type
    // [Context, x: A] |- b: B, [Context] |- \x in A. b: forall x in A. B
    PiIntro,

    // Eliminate a dependent function type symbolically (without applying beta reduction)
    // [Context] |- f: forall x in A. B, [Context] |- a: A => [Context] |- f a: B[a/x]
    PiElim,

    // Compute a dependent function type (apply beta reduction)
    // [Context, x: A] |- b: B, [Context] |- a: A => [Context] |- (\x in A. b) a === b[a/x]: B[a/x]
    PiComp,

    // Uniqueness property on dependent function types
    // [Context] |- f: forall x in A. B => [Context] |- f === (\x in A. f x): forall x in A. B
    PiUniq,
}
