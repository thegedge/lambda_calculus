pub mod conversion;
pub mod de_bruijn;
pub mod named;

pub use de_bruijn::Term as DeBruijn;
pub use named::Term as Named;

pub trait Notation
    : std::fmt::Display
{
    /// Type used to represent variable names
    type VariableName;

    /// Returns a term representing the application of `arg` to `func`
    fn application(func: Self, arg: Self) -> Self;

    /// Returns an abstraction term with the given body and bound variable name
    fn abstraction(bound_var_name: Self::VariableName, body: Self) -> Self;

    /// Returns a variable term with the given name
    fn variable(name: Self::VariableName) -> Self;
}
