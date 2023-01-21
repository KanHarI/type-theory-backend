pub enum OpcodeLabel {
    ////////////
    // AXIOMS //
    ////////////

    // Define a new axiom, extending the intuitionist constructive logic build into the type system
    // Used to define the law of excluded middle, axiom of choice, univalence axiom, etc.
    // Of course automated provers are not allowed to use this operation, but it is useful for
    // extending the logical system with new axioms such as the axiom of choice, and the law of
    // excluded middle. In Homotopy Type Theory, univalence is also defined as an axiom.
    Axiom,

    ////////////////////////
    // CONTEXT OPERATIONS //
    ////////////////////////
    // Context opcodes are used to define the scope of a variable,
    // and to define the type of a variable,
    // as well as introducing new types

    // Creates a new empty context
    // The first opcode in any proof or definition
    // []
    // CtxEmp,

    // Extends the context with a type definition
    // [Context] |- A: U => [Context, x: A]
    CtxExt,

    // Create a new variable from a context definition
    // [Context, a: X] => [Context, a: X] |- a: X
    Vble,

    // Create a new variable equal to another expression
    // [Context, a: X] => [Context, a: X] |- b === a: X
    VbleEq,

    // Substitution
    // [Context1] |- a: A, [Context1, x: A, Context2] |- b: B => [Context1, Context2[a/x]] |- b[a/x]: B[a/x]
    // Note that this opcode requires subexpression highlighting
    Subst,

    // Highlight a subexpression
    // [Context] |- (\a in A. a) b: A => [0, 0] => [Context] |- (\a in ($ A). a) b: A
    // This opcode takes an expression and an index and, and highlights the subexpression at the index
    High,

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
    //
    // In the future, after defining the ordinals, we may want to introduce
    // a function that takes an ordinal and returns the corresponding universe
    // Universe === \w in Ord. (LT w kappa) -> U_w: Ord -> U_1 -> U_kappa
    // Where kappa is an ordinal of a large inaccessible cardinal, and LT is the
    // less than relation on ordinals.
    // Such a function would need axiomatic justification
    //
    // In that case we would probably also want to define
    // UComul === \i in Ord. \j in Ord. (LEQ i j) -> \T in U_i. T:
    //                          forall i in Ord. forall j in Ord. U_1 -> U_i -> U_j
    // Where LEQ is the ordinal less than or equal to relation.
    // The signature of LEQ is Ord -> Ord -> U_1, and not - as one might expect - Ord -> Ord -> U_0
    // This is because the ordinals are too large to be part of U_0,
    // making them and functions between them a part of U_1

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
    // [Context] |- A: U, [Context, x: A] |- B: U => [Context] |- forall x in A. B: U
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
    // PlusCompL,
    // PlusCompR,
    PlusComp,

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

    /////////////////////
    // NATURAL NUMBERS //
    /////////////////////

    NatForm,
    NatIntroZ,
    NatIntroS,
    NatElim,
    NatComp,

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
