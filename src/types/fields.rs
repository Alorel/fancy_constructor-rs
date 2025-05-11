use super::Field;

pub enum Fields {
    Unit,
    Named(Vec<Field>),
    Unnamed(Vec<Field>),
}

impl Fields {
    /// The 1st tuple element is true if the fields are named, false otherwise.
    pub fn to_slice(&self) -> Option<(bool, &[Field])> {
        match *self {
            Fields::Unit => None,
            Fields::Named(ref fields) => Some((true, fields)),
            Fields::Unnamed(ref fields) => Some((false, fields)),
        }
    }

    /// The 1st tuple element is true if the fields are named, false otherwise.
    pub fn into_vec(self) -> Option<(bool, Vec<Field>)> {
        match self {
            Fields::Unit => None,
            Fields::Named(fields) => Some((true, fields)),
            Fields::Unnamed(fields) => Some((false, fields)),
        }
    }

    /// Returns true if there will be no constructor arguments.
    pub fn is_argless(&self) -> bool {
        if let Some((_, fields)) = self.to_slice() {
            for field in fields {
                if !field.opts.should_skip_args() {
                    return false;
                }
            }
        }

        true
    }
}
