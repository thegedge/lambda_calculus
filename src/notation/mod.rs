pub mod named;

pub trait Notation
    : std::fmt::Display
{
    type VariableName;

    /// Returns a term representing the application of `arg` to `func`
    fn application(func: Self, arg: Self) -> Self;

    /// Returns an abstraction term with the given body and bound variable name
    fn abstraction(bound_var_name: Self::VariableName, body: Self) -> Self;

    /// Returns a variable term with the given name
    fn variable(name: Self::VariableName) -> Self;
}
