// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-application-permission-management: application permission management component model.
//!
//! Real implementation: permissions, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Permission {
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
pub enum ApplicationPermissionManagementError {
    EmptyPermissionName,
    DuplicatePermission(String),
    EmptyEventSubject,
}

impl fmt::Display for ApplicationPermissionManagementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPermissionName => write!(f, "permission name must not be empty"),
            Self::DuplicatePermission(id) => write!(f, "duplicate permission {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for ApplicationPermissionManagementError {}

pub struct ApplicationPermissionManagement {
    pub name: String,
    permissions: BTreeMap<String, Permission>,
    events: BTreeMap<String, Event>,
}

impl ApplicationPermissionManagement {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            permissions: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_permission(&mut self, item: Permission) -> Result<(), ApplicationPermissionManagementError> {
        if item.name.trim().is_empty() {
            return Err(ApplicationPermissionManagementError::EmptyPermissionName);
        }
        let id = item.id.clone();
        if self.permissions.contains_key(&id) {
            return Err(ApplicationPermissionManagementError::DuplicatePermission(id.clone()));
        }
        self.permissions.insert(id, item);
        Ok(())
    }

    pub fn permission_count(&self) -> usize {
        self.permissions.len()
    }

    pub fn get_permission(&self, id: &str) -> Option<&Permission> {
        self.permissions.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), ApplicationPermissionManagementError> {
        if event.subject.trim().is_empty() {
            return Err(ApplicationPermissionManagementError::EmptyEventSubject);
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

    fn sample_permission(id: &str, name: &str) -> Permission {
        Permission {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = ApplicationPermissionManagement::new("ApplicationPermissionManagement");
        assert_eq!(m.name, "ApplicationPermissionManagement");
        assert_eq!(m.permission_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_permission() {
        let mut m = ApplicationPermissionManagement::new("ApplicationPermissionManagement");
        m.add_permission(sample_permission("a1", "Alpha")).unwrap();
        assert_eq!(m.permission_count(), 1);
        assert!(m.get_permission("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = ApplicationPermissionManagement::new("ApplicationPermissionManagement");
        assert!(m.add_permission(sample_permission("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = ApplicationPermissionManagement::new("ApplicationPermissionManagement");
        m.add_permission(sample_permission("x", "One")).unwrap();
        assert!(m.add_permission(sample_permission("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = ApplicationPermissionManagement::new("ApplicationPermissionManagement");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
