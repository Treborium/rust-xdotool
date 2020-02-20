use std::fmt;

/// A Vec wrapper that implements the `fmt::Display` trait for easy command creation.
///
/// # Examples
///
/// Create an empty `OptionVec`:
///
/// ```
/// # use xdotool::{OptionVec, option_vec};
/// # use xdotool::command::options;
/// let o1 = OptionVec::<options::KeyboardOption>::new();
/// // Or
/// let o2: OptionVec<options::KeyboardOption> = option_vec![];
///
/// ```
///
/// Create an `OptionVec` for the search command:
///
/// ```
/// # use xdotool::{OptionVec, option_vec};
/// # use xdotool::command::options;
///
/// let o = option_vec![
///     options::SearchOption::Name,
///     options::SearchOption::ClassName,
///     options::SearchOption::Any,
/// ];
/// ```
#[derive(Debug, Default)]
pub struct OptionVec<T: fmt::Display>(pub Vec<T>);

impl<T: fmt::Display> OptionVec<T> {
    pub fn new() -> OptionVec<T> {
        OptionVec(Vec::new())
    }
}

impl<T: fmt::Display> fmt::Display for OptionVec<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut space_separated = String::new();

        if !self.0.is_empty() {
            for thing in &self.0[0..self.0.len() - 1] {
                space_separated.push_str(&thing.to_string());
                space_separated.push_str(" ");
            }

            space_separated.push_str(&self.0[self.0.len() - 1].to_string());
        }

        write!(f, "{}", space_separated)
    }
}

#[macro_export]
macro_rules! option_vec {
    ($elem:expr; $n:expr) => (
        OptionVec(vec![$elem; $n])
    );
    ($($x:expr),*) => (
        OptionVec(<[_]>::into_vec(Box::new([$($x),*])))
    );
    ($($x:expr,)*) => (OptionVec(vec![$($x),*]))
}
