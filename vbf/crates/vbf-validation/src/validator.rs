use crate::ValidationReport;

pub trait Validator<T> {
    fn validate(&self, subject: &T, report: &mut ValidationReport);
}
