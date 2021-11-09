mod request {
    use super::super::super::super::get_xml;
    use super::super::super::super::CLTRID;
    use crate::epp::ext::neustar::object::data::ContactExtension;
    use crate::epp::object::data::ContactStatus;
    use crate::epp::object::data::{Address, Phone, PostalInfo};
    use crate::epp::xml::EppXml;
    use crate::epp::*;

    #[test]
    fn contact_create() {
        let xml = get_xml("ext/neustar/request/contact/create.xml").unwrap();

        let street = vec!["58", "Orchid Road"];
        let address = Address::new(street, "Paris", "Paris", "392374", "FR");
        let postal_info = PostalInfo::new("int", "John Doe", "Acme Widgets", address);
        let mut voice = Phone::new("+33.47237942");
        voice.set_extension("123");
        let mut fax = Phone::new("+33.86698799");
        fax.set_extension("677");

        let ext = ContactExtension {
            ext_contact: None,
            app_purpose: Some("P2".to_string()),
            nexus_category: Some("C31/DE".to_string()),
        };

        let mut object = EppNeustarContactCreate::new(
            "eppdev-contact-3",
            "contact@eppdev.net",
            postal_info,
            voice,
            "eppdev-387323",
            ext,
            CLTRID,
        );
        object.set_fax(fax);

        let serialized = object.serialize().unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn contact_update() {
        let xml = get_xml("ext/neustar/request/contact/update.xml").unwrap();

        let mut object = EppNeustarContactUpdate::new("eppdev-contact-3", CLTRID);

        let street = vec!["58", "Orchid Road"];
        let address = Address::new(street, "Paris", "Paris", "392374", "FR");
        let postal_info = PostalInfo::new("loc", "John Doe", "Acme Widgets", address);
        let voice = Phone::new("+33.47237942");

        object.set_info("newemail@eppdev.net", postal_info, voice, "eppdev-387323");
        let add_statuses = vec![ContactStatus {
            status: "clientTransferProhibited".to_string(),
        }];
        object.add(add_statuses);
        let remove_statuses = vec![ContactStatus {
            status: "clientDeleteProhibited".to_string(),
        }];
        object.remove(remove_statuses);

        let ext = ContactExtension {
            ext_contact: None,
            app_purpose: None,
            nexus_category: Some("C31/DE".to_string()),
        };

        object.set_extension(ext);

        let serialized = object.serialize().unwrap();

        assert_eq!(xml, serialized);
    }
}
