use std::{iter::FusedIterator, mem::take};

/// Stores the part of the path that hasn't been used
#[derive(Debug)]
pub struct RemainingPath(pub(crate) Option<String>);

impl RemainingPath {
    /// Returns the next element without changing the remaining path
    pub fn peek(&self) -> Option<String> {
        let Some((next_element, _)) = self.0.as_ref()?.split_once('/') else {
            // unwrap is okay because it would have returned above
            return Some(self.0.clone().unwrap());
        };

        Some(next_element.to_string())
    }

    /// Returns the next last element without changing the remaining path
    pub fn peek_back(&self) -> Option<String> {
        let Some((_, next_element)) = self.0.as_ref()?.rsplit_once('/') else {
            // unwrap is okay because it would have returned above
            return Some(self.0.clone().unwrap());
        };

        Some(next_element.to_string())
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
            Some(remaining_path.clone())
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
            Some(remaining_path.clone())
        };

        Some(next_element)
    }
}
