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
    type Term : Clone;

    /// Perform one small step reduction on the given term
    fn step(&self, term: Self::Term) -> Option<Self::Term>;

    /// Reduce the given term as much as possible
    fn reduce(&self, mut term: Self::Term) -> Self::Term {
        while let Some(new_term) = self.step(term.clone()) {
            term = new_term;
        }
        term
    }
}
