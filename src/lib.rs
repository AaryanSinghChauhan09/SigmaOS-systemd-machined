// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-systemd-machined: systemd machined component model.
//!
//! Real implementation: machineds, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Machined {
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
pub enum SystemdMachinedError {
    EmptyMachinedName,
    DuplicateMachined(String),
    EmptyEventSubject,
}

impl fmt::Display for SystemdMachinedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyMachinedName => write!(f, "machined name must not be empty"),
            Self::DuplicateMachined(id) => write!(f, "duplicate machined {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for SystemdMachinedError {}

pub struct SystemdMachined {
    pub name: String,
    machineds: BTreeMap<String, Machined>,
    events: BTreeMap<String, Event>,
}

impl SystemdMachined {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            machineds: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_machined(&mut self, item: Machined) -> Result<(), SystemdMachinedError> {
        if item.name.trim().is_empty() {
            return Err(SystemdMachinedError::EmptyMachinedName);
        }
        let id = item.id.clone();
        if self.machineds.contains_key(&id) {
            return Err(SystemdMachinedError::DuplicateMachined(id.clone()));
        }
        self.machineds.insert(id, item);
        Ok(())
    }

    pub fn machined_count(&self) -> usize {
        self.machineds.len()
    }

    pub fn get_machined(&self, id: &str) -> Option<&Machined> {
        self.machineds.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), SystemdMachinedError> {
        if event.subject.trim().is_empty() {
            return Err(SystemdMachinedError::EmptyEventSubject);
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

    fn sample_machined(id: &str, name: &str) -> Machined {
        Machined {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = SystemdMachined::new("SystemdMachined");
        assert_eq!(m.name, "SystemdMachined");
        assert_eq!(m.machined_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_machined() {
        let mut m = SystemdMachined::new("SystemdMachined");
        m.add_machined(sample_machined("a1", "Alpha")).unwrap();
        assert_eq!(m.machined_count(), 1);
        assert!(m.get_machined("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = SystemdMachined::new("SystemdMachined");
        assert!(m.add_machined(sample_machined("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = SystemdMachined::new("SystemdMachined");
        m.add_machined(sample_machined("x", "One")).unwrap();
        assert!(m.add_machined(sample_machined("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = SystemdMachined::new("SystemdMachined");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
