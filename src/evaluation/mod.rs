//! Types for evaluating lambda calucus expressions.
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

pub struct EmptyContext;

// TODO for all strategies, have tests to ensure evaluation happens as expected.

/// A beta reduction strategy.
pub trait Evaluable {
    type Term : Into<crate::term::DeBruijn>;
    type Context;

    /// Perform one small step evaluation on the given term
    fn step(&self, ctx: &mut Self::Context, term: Self::Term) -> Option<Self::Term>;

    /// Evaluate the given term as much as possible
    fn evaluate(&self, ctx: &mut Self::Context, mut term: Self::Term) -> Self::Term {
        while let Some(new_term) = self.step(ctx, term.clone()) {
            term = new_term;
        }
        term
    }
}
