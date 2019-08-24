//! Types for reducing lambda calucus expressions.
//!
//! An expression of the form `(\x.t1) t2` is reducible, and known as a redex.

mod call_by_value;
mod full;
mod lazy;
mod normal;

pub use call_by_value::CallByValue;
pub use full::Full;
pub use lazy::Lazy;
pub use normal::Normal;

// TODO for all strategies, have tests to ensure evaluation happens as expected.

/// A beta reduction strategy.
pub trait Reduction {
    type Term;

    /// Reduce a given term
    fn reduce(&self, term: &Self::Term) -> Self::Term;
}
