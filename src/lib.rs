//! A safe API over macOS Contacts — the access the user gave, and the contacts that have an email
//! address — for an application whose own code may hold no `unsafe` (Skourier's workspace forbids it).
//!
//! It reads; it never writes a contact, and never asks for access unless the caller does
//! (`request_access`). Off macOS, there are no contacts: `access` answers `Unavailable`.
//!
//! An application that calls `request_access` or `contacts` must say why in its `Info.plist`
//! (`NSContactsUsageDescription`) — without it, macOS stops the process — and run from its bundle.

mod contact;
#[cfg(target_os = "macos")]
mod macos;

use std::time::Duration;

pub use contact::Contact;

/// What the user lets the application do with their contacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Access {
    /// All of them, or those the user chose.
    Granted,
    /// The user refused, in the system's prompt or its settings.
    Denied,
    /// Not asked yet: `request_access` shows the system's prompt.
    NotDetermined,
    /// Refused by a restriction the user cannot lift (parental controls, a profile).
    Restricted,
    /// No Contacts here: another system.
    Unavailable,
}

/// Why the contacts could not be read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContactsError {
    /// The user has not granted access.
    NotAllowed(Access),
    /// The system refused, and said this.
    Failed(String),
}

/// What the user lets the application do, without asking anything.
pub fn access() -> Access {
    #[cfg(target_os = "macos")]
    return macos::access();
    #[cfg(not(target_os = "macos"))]
    Access::Unavailable
}

/// Asks the user, the first time, through the system's prompt; then answers what they chose. Waits
/// `patience` at most for the answer — the prompt waits for the user —; past it, gives what is known.
pub fn request_access(patience: Duration) -> Access {
    #[cfg(target_os = "macos")]
    return macos::request_access(patience);
    #[cfg(not(target_os = "macos"))]
    {
        let _ = patience;
        Access::Unavailable
    }
}

/// The contacts that have an email address, once access is granted.
pub fn contacts() -> Result<Vec<Contact>, ContactsError> {
    match access() {
        Access::Granted => read(),
        refused => Err(ContactsError::NotAllowed(refused)),
    }
}

#[cfg(target_os = "macos")]
fn read() -> Result<Vec<Contact>, ContactsError> {
    macos::contacts().map_err(ContactsError::Failed)
}

#[cfg(not(target_os = "macos"))]
fn read() -> Result<Vec<Contact>, ContactsError> {
    Ok(Vec::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    // A test binary is not an application bundle: macOS has never been asked, and nothing is read.
    #[test]
    fn reads_nothing_without_access() {
        let known = access();
        assert_ne!(known, Access::Granted);
        assert_eq!(contacts(), Err(ContactsError::NotAllowed(known)));
    }
}
