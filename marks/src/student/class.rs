use super::subject;
#[derive(Debug, Eq, PartialEq)]
pub enum Class {
    Distinction,
    FirstClass,
    SecondClass,
}

pub fn compute_class(total: subject::Marks, total_marks: subject::Marks) -> Option<Class> {
    let percent: f64 = (total as f64 / total_marks as f64) * 100.0;

    if percent >= 80.0 {
        return Some(Class::Distinction);
    } else if percent >= 60.0 && percent < 80.0 {
        return Some(Class::FirstClass);
    } else if percent >= 40.0 && percent < 60.0 {
        return Some(Class::SecondClass);
    }
    None
}
