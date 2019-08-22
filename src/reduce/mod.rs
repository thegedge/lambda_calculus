//! Types for reducing lambda calucus expressions.
//!
//! An expression of the form `(\x.t1) t2` is reducible, and known as a redex.

mod full;
mod normal;

pub use full::Full;
pub use normal::Normal;

/// Use the call by name reduction strategy.
///
/// This strategy uses the normal order strategy, but does not reduce within abstractions.
//struct CallByName;

/// Use the call by value reduction strategy.
///
/// This strategy reduces the outermost redexes first, but only after the right-hand side has been
/// reduced to a value. For standard lambda calculus, values are simply abstractions.
//struct CallByValue;

/// A reduction strategy.
trait Reduction {
    type Term;

    /// Reduce a given term
    fn reduce(&self, term: &Self::Term) -> Self::Term;
}
