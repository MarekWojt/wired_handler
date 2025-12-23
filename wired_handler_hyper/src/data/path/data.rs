use std::{iter::FusedIterator, mem::take};

/// Stores the part of the path that hasn't been used
#[derive(Debug)]
pub struct RemainingPath(pub(crate) Option<String>);

impl RemainingPath {
    /// Returns the next element without changing the remaining path
    pub fn peek(&self) -> Option<&str> {
        let string_value = self.0.as_ref()?;
        let Some((next_element, _)) = string_value.split_once('/') else {
            return Some(string_value);
        };

        Some(next_element)
    }

    /// Returns the next last element without changing the remaining path
    pub fn peek_back(&self) -> Option<&str> {
        let string_value = self.0.as_ref()?;
        let Some((_, next_element)) = string_value.rsplit_once('/') else {
            return Some(string_value);
        };

        Some(next_element)
    }
}

impl Iterator for RemainingPath {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        let Some((next_element, remaining_path)) = self.0.as_ref()?.split_once('/') else {
            return take(&mut self.0);
        };

        let remaining_path = remaining_path.to_string();
        let next_element = next_element.to_string();

        self.0 = if next_element.is_empty() {
            None
        } else {
            Some(remaining_path)
        };

        Some(next_element)
    }
}

impl FusedIterator for RemainingPath {}

impl DoubleEndedIterator for RemainingPath {
    fn next_back(&mut self) -> Option<Self::Item> {
        let Some((remaining_path, next_element)) = self.0.as_ref()?.rsplit_once('/') else {
            return take(&mut self.0);
        };

        let remaining_path = remaining_path.to_string();
        let next_element = next_element.to_string();

        self.0 = if next_element.is_empty() {
            None
        } else {
            Some(remaining_path)
        };

        Some(next_element)
    }
}
