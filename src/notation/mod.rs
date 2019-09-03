pub mod named;

pub trait NotationSystem {
    type Term : std::fmt::Display;
    type VariableName;

    /// Returns a term representing the application of `arg` to `func`
    fn application(func: Self::Term, arg: Self::Term) -> Self::Term;

    /// Returns an abstraction term with the given body and bound variable name
    fn abstraction(bound_var_name: Self::VariableName, body: Self::Term) -> Self::Term;

    /// Returns a variable term with the given name
    fn variable(name: Self::VariableName) -> Self::Term;
}
