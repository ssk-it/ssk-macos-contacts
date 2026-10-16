//! The calls to the Contacts framework — INV-1: the only module that holds `unsafe`, each block with
//! the reason it is sound.
#![allow(unsafe_code)]

use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use block2::RcBlock;
use objc2::AllocAnyThread;
use objc2::rc::Retained;
use objc2::runtime::{Bool, ProtocolObject};
use objc2_contacts::{
    CNAuthorizationStatus, CNContact, CNContactEmailAddressesKey, CNContactFamilyNameKey,
    CNContactFetchRequest, CNContactGivenNameKey, CNContactOrganizationNameKey, CNContactStore,
    CNEntityType, CNKeyDescriptor,
};
use objc2_foundation::{NSArray, NSError};

use crate::Access;
use crate::contact::{Contact, Parts, contact_of};

pub(crate) fn access() -> Access {
    // SAFETY: a class method taking a plain integer, documented as thread safe; it reads the status, asks nothing.
    let status =
        unsafe { CNContactStore::authorizationStatusForEntityType(CNEntityType::Contacts) };
    access_of(status)
}

fn access_of(status: CNAuthorizationStatus) -> Access {
    match status {
        CNAuthorizationStatus::Authorized | CNAuthorizationStatus::Limited => Access::Granted,
        CNAuthorizationStatus::Denied => Access::Denied,
        CNAuthorizationStatus::Restricted => Access::Restricted,
        _ => Access::NotDetermined,
    }
}

pub(crate) fn request_access(patience: Duration) -> Access {
    let (sender, answer) = mpsc::channel();
    let sender = Mutex::new(Some(sender));
    let handler = RcBlock::new(move |granted: Bool, _error: *mut NSError| {
        if let Some(sender) = sender.lock().ok().and_then(|mut sender| sender.take()) {
            let _ = sender.send(granted.as_bool());
        }
    });
    // SAFETY: `new` makes a store owned here; the handler is a Rust block that only sends a boolean and
    // copies nothing from the error pointer, which it never reads; the framework keeps the block alive
    // until it calls it, on a queue of its own.
    unsafe {
        let store = CNContactStore::new();
        store.requestAccessForEntityType_completionHandler(CNEntityType::Contacts, &handler);
    }
    match answer.recv_timeout(patience) {
        Ok(true) => Access::Granted,
        Ok(false) | Err(_) => access(),
    }
}

pub(crate) fn contacts() -> Result<Vec<Contact>, String> {
    let found = Arc::new(Mutex::new(Vec::new()));
    let gathered = found.clone();
    let block = RcBlock::new(
        move |contact: std::ptr::NonNull<CNContact>, _stop: std::ptr::NonNull<Bool>| {
            // SAFETY: the framework hands a valid contact for the duration of the call; it is only read here.
            let contact = unsafe { contact.as_ref() };
            if let (Some(read), Ok(mut gathered)) = (read_contact(contact), gathered.lock()) {
                gathered.push(read);
            }
        },
    );
    let mut error: Option<Retained<NSError>> = None;
    // SAFETY: the keys are the framework's own constants, which outlive the program; the request is
    // built from them and owned here; the block only reads the contacts it is handed, while the call
    // lasts, and the error slot is ours.
    let enumerated = unsafe {
        let keys: Retained<NSArray<ProtocolObject<dyn CNKeyDescriptor>>> = NSArray::from_slice(&[
            ProtocolObject::from_ref(CNContactGivenNameKey),
            ProtocolObject::from_ref(CNContactFamilyNameKey),
            ProtocolObject::from_ref(CNContactOrganizationNameKey),
            ProtocolObject::from_ref(CNContactEmailAddressesKey),
        ]);
        let request =
            CNContactFetchRequest::initWithKeysToFetch(CNContactFetchRequest::alloc(), &keys);
        CNContactStore::new().enumerateContactsWithFetchRequest_error_usingBlock(
            &request,
            Some(&mut error),
            &block,
        )
    };
    if !enumerated {
        return Err(error.map_or_else(
            || "Contacts failed".to_owned(),
            |error| error.localizedDescription().to_string(),
        ));
    }
    let contacts = found.lock().map(|found| found.clone()).unwrap_or_default();
    Ok(contacts)
}

/// A contact's name and addresses, read through the keys the request fetched.
fn read_contact(contact: &CNContact) -> Option<Contact> {
    // SAFETY: the four properties read are those the request fetched; each returns a retained object.
    let (given, family, organisation, labelled) = unsafe {
        (
            contact.givenName(),
            contact.familyName(),
            contact.organizationName(),
            contact.emailAddresses(),
        )
    };
    let addresses = labelled
        .iter()
        // SAFETY: an email address's labelled value holds a string, as the property's type says.
        .map(|address| unsafe { address.value() }.to_string())
        .collect();
    contact_of(Parts {
        given: &given.to_string(),
        family: &family.to_string(),
        organisation: &organisation.to_string(),
        addresses,
    })
}
