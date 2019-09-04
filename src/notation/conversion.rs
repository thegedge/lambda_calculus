use crate::vars::Variables;

use super::{
    DeBruijn,
    Named,
    Notation,
};

struct RemoveNames {
    naming_context: Vec<<Named as Notation>::VariableName>,
}

impl RemoveNames {
    pub fn call(&mut self, named: &Named) -> DeBruijn {
        match named {
            Named::Variable(s) => {
                DeBruijn::Variable(
                    self.naming_context
                        .iter()
                        .rev()
                        .position(|x| x == s)
                        .unwrap() as u32
                )
            },
            Named::Abstraction(arg, body) => {
                self.naming_context.push(arg.clone());
                let abs = DeBruijn::Abstraction(box self.call(body));
                self.naming_context.pop();
                abs
            },
            Named::Application(t1, t2) => {
                DeBruijn::Application(box self.call(t1), box self.call(t2))
            },
        }
    }
}

impl From<&Named> for DeBruijn {
    fn from(named: &Named) -> DeBruijn {
        RemoveNames { naming_context: named.free_variables().into_iter().collect() }.call(named)
    }
}

#[cfg(test)]
mod test {
    use crate::notation::de_bruijn::{l, a};
    use crate::notation::named::{l as nl, a as na};

    #[test]
    pub fn test_abstraction_with_bound_variables() {
        assert_eq!(
            l(l(a(1u32, 0u32))),
            (&nl("x", nl("y", na("x", "y")))).into()
        );
    }

    #[test]
    pub fn test_abstraction_with_free_variable() {
        assert_eq!(
            l(l(a(a(1u32, 0u32), 2u32))),
            (&nl("x", nl("y", na(na("x", "y"), "z")))).into()
        );
    }
}
