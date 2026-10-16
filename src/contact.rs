//! A contact as the application sees it: what it is called, and its email addresses.

/// A contact that has at least one email address.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Contact {
    /// Its given and family names, or its organisation's; `None` when it has none.
    pub name: Option<String>,
    /// Each once, in the contact's order, trimmed.
    pub addresses: Vec<String>,
}

/// The parts of a contact as Contacts gives them.
pub(crate) struct Parts<'a> {
    pub given: &'a str,
    pub family: &'a str,
    pub organisation: &'a str,
    pub addresses: Vec<String>,
}

/// The contact the parts make; `None` without an address — nothing to write to.
pub(crate) fn contact_of(parts: Parts<'_>) -> Option<Contact> {
    let mut addresses: Vec<String> = Vec::new();
    for address in parts.addresses {
        let address = address.trim().to_owned();
        let known = addresses
            .iter()
            .any(|kept| kept.eq_ignore_ascii_case(&address));
        if address.contains('@') && !known {
            addresses.push(address);
        }
    }
    if addresses.is_empty() {
        return None;
    }
    let person = [parts.given.trim(), parts.family.trim()]
        .into_iter()
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    let name = Some(person)
        .filter(|person| !person.is_empty())
        .or_else(|| {
            Some(parts.organisation.trim().to_owned())
                .filter(|organisation| !organisation.is_empty())
        });
    Some(Contact { name, addresses })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parts<'a>(
        given: &'a str,
        family: &'a str,
        organisation: &'a str,
        addresses: &[&str],
    ) -> Parts<'a> {
        Parts {
            given,
            family,
            organisation,
            addresses: addresses
                .iter()
                .map(|&address| address.to_owned())
                .collect(),
        }
    }

    #[test]
    fn names_a_person_by_given_and_family_name() {
        let contact = contact_of(parts(
            " Ada ",
            "Lovelace",
            "Analytical",
            &["ada@example.org"],
        ))
        .unwrap();

        assert_eq!(contact.name.as_deref(), Some("Ada Lovelace"));
    }

    #[test]
    fn names_an_organisation_without_a_person() {
        let contact = contact_of(parts("", " ", "La Gazette", &["news@gazette.example"])).unwrap();

        assert_eq!(contact.name.as_deref(), Some("La Gazette"));
    }

    #[test]
    fn keeps_each_address_once_and_drops_what_is_not_one() {
        let contact = contact_of(parts(
            "Ada",
            "",
            "",
            &[
                " ada@example.org",
                "ADA@example.org",
                "not an address",
                "ada@work.example",
            ],
        ))
        .unwrap();

        assert_eq!(contact.addresses, ["ada@example.org", "ada@work.example"]);
    }

    #[test]
    fn leaves_out_a_contact_without_an_address() {
        assert_eq!(contact_of(parts("Charles", "Babbage", "", &[])), None);
        assert_eq!(
            contact_of(parts("", "", "", &["ada@example.org"]))
                .unwrap()
                .name,
            None
        );
    }
}
