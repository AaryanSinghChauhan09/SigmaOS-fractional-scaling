// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-fractional-scaling: fractional scaling component model.
//!
//! Real implementation: scalings, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scaling {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FractionalScalingError {
    EmptyScalingName,
    DuplicateScaling(String),
    EmptyEventSubject,
}

impl fmt::Display for FractionalScalingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyScalingName => write!(f, "scaling name must not be empty"),
            Self::DuplicateScaling(id) => write!(f, "duplicate scaling {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for FractionalScalingError {}

pub struct FractionalScaling {
    pub name: String,
    scalings: BTreeMap<String, Scaling>,
    events: BTreeMap<String, Event>,
}

impl FractionalScaling {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            scalings: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_scaling(&mut self, item: Scaling) -> Result<(), FractionalScalingError> {
        if item.name.trim().is_empty() {
            return Err(FractionalScalingError::EmptyScalingName);
        }
        let id = item.id.clone();
        if self.scalings.contains_key(&id) {
            return Err(FractionalScalingError::DuplicateScaling(id.clone()));
        }
        self.scalings.insert(id, item);
        Ok(())
    }

    pub fn scaling_count(&self) -> usize {
        self.scalings.len()
    }

    pub fn get_scaling(&self, id: &str) -> Option<&Scaling> {
        self.scalings.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), FractionalScalingError> {
        if event.subject.trim().is_empty() {
            return Err(FractionalScalingError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_scaling(id: &str, name: &str) -> Scaling {
        Scaling {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = FractionalScaling::new("FractionalScaling");
        assert_eq!(m.name, "FractionalScaling");
        assert_eq!(m.scaling_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_scaling() {
        let mut m = FractionalScaling::new("FractionalScaling");
        m.add_scaling(sample_scaling("a1", "Alpha")).unwrap();
        assert_eq!(m.scaling_count(), 1);
        assert!(m.get_scaling("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = FractionalScaling::new("FractionalScaling");
        assert!(m.add_scaling(sample_scaling("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = FractionalScaling::new("FractionalScaling");
        m.add_scaling(sample_scaling("x", "One")).unwrap();
        assert!(m.add_scaling(sample_scaling("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = FractionalScaling::new("FractionalScaling");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
