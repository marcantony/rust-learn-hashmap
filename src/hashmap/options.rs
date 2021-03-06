//! HashMap configuration options and validation.

pub const DEFAULT_CAPACITY: usize = 16;
pub const DEFAULT_LOAD_FACTOR: f64 = 0.75;
pub const DEFAULT_DYNAMIC_RESIZING: bool = true;

/// An unvalidated set of hash map options. Create an [Options]
/// and call `validate` to produce a [ValidatedOptions] which can then be used
/// to create a hash map. Properties left as [None] will be set to sensible defaults.
#[derive(Default)]
pub struct Options {
    pub initial_capacity: Option<usize>,
    pub load_factor: Option<f64>,
    pub dynamic_resizing: Option<bool>
}

pub struct ValidatedOptions {
    initial_capacity: usize,
    load_factor: f64,
    dynamic_resizing: bool
}

impl Options {
    /// Validates an [Options] to produce a [ValidatedOptions] or a list of errors.
    pub fn validate(self) -> Result<ValidatedOptions, Vec<&'static str>> {
        let mut errors = Vec::new();

        self.load_factor.map(|lf| {
            if lf <= 0.0 {
                errors.push("Load factor cannot be zero or less");
            };
        });

        if errors.is_empty() {
            Ok(ValidatedOptions {
                initial_capacity: self.initial_capacity.unwrap_or(DEFAULT_CAPACITY),
                load_factor: self.load_factor.unwrap_or(DEFAULT_LOAD_FACTOR),
                dynamic_resizing: self.dynamic_resizing.unwrap_or(DEFAULT_DYNAMIC_RESIZING)
            })
        } else {
            Err(errors)
        }

    }
}

impl ValidatedOptions {
    pub fn initial_capacity(&self) -> usize {
        self.initial_capacity
    }

    pub fn load_factor(&self) -> f64 {
        self.load_factor
    }

    pub fn dynamic_resizing(&self) -> bool {
        self.dynamic_resizing
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passing_validation() {
        let options = Options {
            initial_capacity: Some(DEFAULT_CAPACITY),
            load_factor: Some(DEFAULT_LOAD_FACTOR),
            dynamic_resizing: Some(DEFAULT_DYNAMIC_RESIZING)
        };

        assert!(options.validate().is_ok());
    }

    #[test]
    fn load_factor_invalid() {
        let options = Options {
            initial_capacity: Some(DEFAULT_CAPACITY),
            load_factor: Some(-0.5),
            dynamic_resizing: Some(DEFAULT_DYNAMIC_RESIZING)
        };

        assert!(options.validate().is_err());
    }
}
