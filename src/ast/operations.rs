pub enum Operations {
    ////////////
    // AXIOMS //
    ////////////

    // Define a new axiom, extending the intuitionist constructive logic build into the type system
    // Used to define the law of excluded middle, axiom of choice, univalence axiom, etc.
    Axiom,

    ////////////////////////
    // CONTEXT OPERATIONS //
    ////////////////////////
    // Context operations are used to define the scope of a variable,
    // and to define the type of a variable,
    // as well as introducing new types

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

    /////////////////////////
    // UNIVERSE OPERATIONS //
    /////////////////////////
    // Universes are the equivalent of "The type of all types"
    // Not every AST expression is part of a universe, but the type of every AST
    // expression is part of a universe.
    // We will usually work with U_0, the "smallest universe"
    // When we would have to abstract over all types, we will sometimes require higher universes
    // For example, the universe U_1 is a type of every type that
    // is part of U_0 ("ordinary types"), and it is also the type of U_0 itself
    // Universes are required to abstract over types while preventing
    // self reference paradoxes like Russel's paradox

    // Introduction of a new universe
    // [Context] => [Context] |- U_i: U_(i+1)
    UIntro,

    // Type comulation from the ith to the (i+1)th universe
    // [Context] |- A: U_i => [Context] |- A: U_(i+1)
    UCumul,

    ///////////////////////////////////
    // PI (FUNCTION TYPE) OPERATIONS //
    ///////////////////////////////////
    // These are functions (A -> B === forall _ in A. B), and dependent functions (forall x in A. B[x])
    // When B is a constant, this is regular a function type
    // When B depends on x, this is a dependent function type

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

    //////////////////////////////////
    // SIGMA (PAIR TYPE) OPERATIONS //
    //////////////////////////////////
    // These are pairs (a, b): (A, B) and dependent pairs (a, b): (exists x in A. B[x])
    // When B is a constant, this is regular a pair type
    // When B depends on x, this is a dependent pair type

    // Form a dependent pair type
    // [Context] |- A: U, [Context, x: A] |- B: U => [Context] |- exists x in A. B: U
    SigmaForm,

    // Creates an element of a dependent pair type
    // [Context, x: A] |- B: U, [context] |- a: A,  [Context] |- b: B[a/x] => [Context] |- (a, b): exists x in A. B
    SigmaIntro,

    // Eliminate a dependent pair type symbolically (without applying beta reduction)
    // [Context, z: exists x in A. B] |- C: U, [Context, x: A, y: B] |- f: C[(x, y)/z], [Context] |- p: exists x in A. B => TBD
    SigmaElim,

    // Compute a dependent pair type (apply beta reduction)
    // TBD
    SigmaComp,

    //////////////////////////////////////
    // PLUS TYPE (COPRODUCT) OPERATIONS //
    //////////////////////////////////////
    // These are coproducts - the analog of:
    // * Either type from haskell
    // * Enum types in rust
    // * Disjoint unions in set theory
    // * Coproducts in type theory and category theory
    // if a: A, b: B:
    // Left a: A + B
    // Right b: A + B
    PlusForm,
    PlusIntroL,
    PlusIntroR,
    PlusElim,
    PlusCompL,
    PlusCompR,

    /////////////////////////
    // VOID (0) OPERATIONS //
    /////////////////////////
    // Void is the type with no elements, and is the analog of:
    // * The type Void in haskell
    // * The empty set

    // Form the empty type
    // [Context] => [Context] |- Void: U
    VoidForm,

    // ex falso quodlibet, contradiction is a proof of anything
    // [Context, x: Void] |- C: U, [Context] |- a: Void => TBD
    VoidElim,

    /////////////////////////
    // UNIT (1) OPERATIONS //
    /////////////////////////
    // Unit is the type with one element, and is the analog of:
    // * The type () in haskell
    // * The type
    // * The type void in C
    // * The singleton set
    // * Tuples of length 0

    // Form the unit type
    // [Context] => [Context] |- Unit: U
    UnitForm,

    // Form the singleton element
    // [Context] => [Context] |- *: Unit
    UnitIntro,

    // Eliminate the singleton type
    // [Context, x: Unit] |- C: U, [Context] |- c: C[*/x], [Context] |- a: Unit => TBD
    UnitElim,

    // Compute the singleton type (apply beta reduction)
    // [Context, x: Unit] |- C: U, [Context] |- c: C[*/x] => TBD
    UnitComp,

    ////////////////////
    // EQUALITY TYPES //
    ////////////////////
    // These are the equality types
    // a ===_A b is written as Eq A a b

    // Form an equality type
    // [Context] |- A: U, [Context] |- a: A, [Context] b: A => [Context] |- Eq A a b: U
    EqForm,
    EqIntro,
    EqElim,
    EqComp,

    /////////////////////////
    // HOMOTOPY OPERATIONS //
    /////////////////////////
    //

    // The principle of functional extensionality states that two functions are equal
    // if their values are equal at every argument
    PiExt,

    // Univalence states that an identity type is equivalent to an equivalence type
    UUniv,
}
