//! `EppObject` to EPP XML serialization tests

mod request {
    use super::super::get_xml;
    use super::super::CLTRID;
    use crate::epp::object::data::{
        Address, AuthInfo, ContactStatus, DomainContact, DomainStatus, Host, HostAddr, HostAttr,
        HostStatus, Phone, PostalInfo,
    };
    use crate::epp::object::StringValueTrait;
    use crate::epp::request::{EppHello, EppLogin, EppLogout};
    use crate::epp::xml::EppXml;
    use crate::epp::*;
    use chrono::NaiveDate;

    #[test]
    fn hello() {
        let xml = get_xml("request/hello.xml").unwrap();
        let object = EppHello::new();
        let serialized = object.serialize().unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn login() {
        let ext_uris = Some(vec![
            "http://schema.ispapi.net/epp/xml/keyvalue-1.0".to_string()
        ]);

        let xml = get_xml("request/login.xml").unwrap();
        let object = EppLogin::new("username", "password", &ext_uris, CLTRID);
        let serialized = object.serialize().unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn logout() {
        let xml = get_xml("request/logout.xml").unwrap();
        let object = EppLogout::new(CLTRID);
        let serialized = object.serialize().unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn contact_check() {
        let xml = get_xml("request/contact/check.xml").unwrap();
        let object = EppContactCheck::new(vec!["eppdev-contact-1", "eppdev-contact-2"], CLTRID);
        let serialized = object.serialize().unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn contact_create() {
        let xml = get_xml("request/contact/create.xml").unwrap();

        let street = vec!["58", "Orchid Road"];
        let address = Address::new(street, "Paris", "Paris", "392374", "FR");
        let postal_info = PostalInfo::new("int", "John Doe", "Acme Widgets", address);
        let mut voice = Phone::new("+33.47237942");
        voice.set_extension("123");
        let mut fax = Phone::new("+33.86698799");
        fax.set_extension("677");

        let mut object = EppContactCreate::new(
            "eppdev-contact-3",
            "contact@eppdev.net",
            postal_info,
            voice,
            "eppdev-387323",
            CLTRID,
        );
        object.set_fax(fax);

        let serialized = object.serialize().unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn contact_info() {
        let xml = get_xml("request/contact/info.xml").unwrap();

        let object = EppContactInfo::new("eppdev-contact-3", "eppdev-387323", CLTRID);

        let serialized = object.serialize().unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn contact_update() {
        let xml = get_xml("request/contact/update.xml").unwrap();

        let mut object = EppContactUpdate::new("eppdev-contact-3", CLTRID);

        let street = vec!["58", "Orchid Road"];
        let address = Address::new(street, "Paris", "Paris", "392374", "FR");
        let postal_info = PostalInfo::new("loc", "John Doe", "Acme Widgets", address);
        let voice = Phone::new("+33.47237942");

        object.set_info("newemail@eppdev.net", postal_info, voice, "eppdev-387323");
        let add_statuses = vec![ContactStatus {
            status: "clientTransferProhibited".to_string(),
        }];
        object.add_statuses(add_statuses);
        let remove_statuses = vec![ContactStatus {
            status: "clientDeleteProhibited".to_string(),
        }];
        object.remove_statuses(remove_statuses);

        let serialized = object.serialize().unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn contact_delete() {
        let xml = get_xml("request/contact/delete.xml").unwrap();

        let object = EppContactDelete::new("eppdev-contact-3", CLTRID);

        let serialized = object.serialize().unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn domain_check() {
        let xml = get_xml("request/domain/check.xml").unwrap();

        let object = EppDomainCheck::new(vec!["eppdev.com", "eppdev.net"], CLTRID);

        let serialized = object.serialize().unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn domain_create() {
        let xml = get_xml("request/domain/create.xml").unwrap();

        let contacts = vec![
            DomainContact {
                contact_type: "admin".to_string(),
                id: "eppdev-contact-3".to_string(),
            },
            DomainContact {
                contact_type: "tech".to_string(),
                id: "eppdev-contact-3".to_string(),
            },
            DomainContact {
                contact_type: "billing".to_string(),
                id: "eppdev-contact-3".to_string(),
            },
        ];

        let object = EppDomainCreate::new(
            "eppdev-1.com",
            1,
            "eppdev-contact-3",
            "epP4uthd#v",
            contacts,
            CLTRID,
        );

        let serialized = object.serialize().unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn domain_create_with_host_obj() {
        let xml = get_xml("request/domain/create_with_host_obj.xml").unwrap();

        let contacts = vec![
            DomainContact {
                contact_type: "admin".to_string(),
                id: "eppdev-contact-3".to_string(),
            },
            DomainContact {
                contact_type: "tech".to_string(),
                id: "eppdev-contact-3".to_string(),
            },
            DomainContact {
                contact_type: "billing".to_string(),
                id: "eppdev-contact-3".to_string(),
            },
        ];

        let object = EppDomainCreate::new_with_ns(
            "eppdev-1.com",
            1,
            vec!["ns1.test.com", "ns2.test.com"],
            "eppdev-contact-3",
            "epP4uthd#v",
            contacts,
            CLTRID,
        );

        let serialized = object.serialize().unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn domain_create_with_host_attr() {
        let xml = get_xml("request/domain/create_with_host_attr.xml").unwrap();

        let contacts = vec![
            DomainContact {
                contact_type: "admin".to_string(),
                id: "eppdev-contact-3".to_string(),
            },
            DomainContact {
                contact_type: "tech".to_string(),
                id: "eppdev-contact-3".to_string(),
            },
            DomainContact {
                contact_type: "billing".to_string(),
                id: "eppdev-contact-3".to_string(),
            },
        ];

        let host_attr = vec![
            HostAttr {
                name: "ns1.eppdev-1.com".to_string_value(),
                addresses: None,
            },
            HostAttr {
                name: "ns2.eppdev-1.com".to_string_value(),
                addresses: Some(vec![
                    HostAddr::new_v4("177.232.12.58"),
                    HostAddr::new_v6("2404:6800:4001:801::200e"),
                ]),
            },
        ];

        let object = EppDomainCreate::new_with_host_attr(
            "eppdev-2.com",
            1,
            host_attr,
            "eppdev-contact-3",
            "epP4uthd#v",
            contacts,
            CLTRID,
        );

        let serialized = object.serialize().unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn domain_info() {
        let xml = get_xml("request/domain/info.xml").unwrap();

        let object = EppDomainInfo::new("eppdev.com", CLTRID);

        let serialized = object.serialize().unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn domain_update() {
        let xml = get_xml("request/domain/update.xml").unwrap();

        let mut object = EppDomainUpdate::new("eppdev.com", CLTRID);

        let add = DomainAddRemove {
            ns: None,
            contacts: None,
            statuses: Some(vec![DomainStatus {
                status: "clientDeleteProhibited".to_string(),
            }]),
        };

        let remove = DomainAddRemove {
            ns: None,
            contacts: Some(vec![DomainContact {
                contact_type: "billing".to_string(),
                id: "eppdev-contact-2".to_string(),
            }]),
            statuses: None,
        };

        let change_info = DomainChangeInfo {
            registrant: None,
            auth_info: Some(AuthInfo::new("epP5uthd#v")),
        };

        object.add(add);
        object.remove(remove);
        object.info(change_info);

        let serialized = object.serialize().unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn domain_delete() {
        let xml = get_xml("request/domain/delete.xml").unwrap();

        let object = EppDomainDelete::new("eppdev.com", CLTRID);

        let serialized = object.serialize().unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn domain_renew() {
        let xml = get_xml("request/domain/renew.xml").unwrap();

        let exp_date = NaiveDate::from_ymd(2022, 7, 23);
        let object = EppDomainRenew::new("eppdev.com", exp_date, 1, CLTRID);

        let serialized = object.serialize().unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn domain_transfer_request() {
        let xml = get_xml("request/domain/transfer_request.xml").unwrap();

        let object = EppDomainTransferRequest::request("testing.com", 1, "epP4uthd#v", CLTRID);

        let serialized = object.serialize().unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn domain_transfer_approve() {
        let xml = get_xml("request/domain/transfer_approve.xml").unwrap();

        let object = EppDomainTransferApprove::approve("testing.com", CLTRID);

        let serialized = object.serialize().unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn domain_transfer_reject() {
        let xml = get_xml("request/domain/transfer_reject.xml").unwrap();

        let object = EppDomainTransferReject::reject("testing.com", CLTRID);

        let serialized = object.serialize().unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn domain_transfer_cancel() {
        let xml = get_xml("request/domain/transfer_cancel.xml").unwrap();

        let object = EppDomainTransferCancel::cancel("testing.com", CLTRID);

        let serialized = object.serialize().unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn domain_transfer_query() {
        let xml = get_xml("request/domain/transfer_query.xml").unwrap();

        let object = EppDomainTransferQuery::query("testing.com", "epP4uthd#v", CLTRID);

        let serialized = object.serialize().unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn host_check() {
        let xml = get_xml("request/host/check.xml").unwrap();

        let object = EppHostCheck::new(vec!["ns1.eppdev-1.com", "host1.eppdev-1.com"], CLTRID);

        let serialized = object.serialize().unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn host_create() {
        let xml = get_xml("request/host/create.xml").unwrap();

        let host = Host {
            name: "host1.eppdev-1.com".to_string_value(),
            addresses: Some(vec![
                HostAddr::new("v4", "29.245.122.14"),
                HostAddr::new("v6", "2404:6800:4001:801::200e"),
            ]),
        };

        let object = EppHostCreate::new(host, CLTRID);

        let serialized = object.serialize().unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn host_info() {
        let xml = get_xml("request/host/info.xml").unwrap();

        let object = EppHostInfo::new("ns1.eppdev-1.com", CLTRID);

        let serialized = object.serialize().unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn host_update() {
        let xml = get_xml("request/host/update.xml").unwrap();

        let addr = vec![HostAddr::new("v6", "2404:6800:4001:801::200e")];

        let add = HostAddRemove {
            addresses: Some(addr),
            statuses: None,
        };

        let remove = HostAddRemove {
            addresses: None,
            statuses: Some(vec![HostStatus {
                status: "clientDeleteProhibited".to_string(),
            }]),
        };

        let mut object = EppHostUpdate::new("host1.eppdev-1.com", CLTRID);

        object.add(add);
        object.remove(remove);
        object.info(HostChangeInfo {
            name: "host2.eppdev-1.com".to_string_value(),
        });

        let serialized = object.serialize().unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn host_delete() {
        let xml = get_xml("request/host/delete.xml").unwrap();

        let object = EppHostDelete::new("ns1.eppdev-1.com", CLTRID);

        let serialized = object.serialize().unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn message_poll() {
        let xml = get_xml("request/message/poll.xml").unwrap();

        let object = EppMessagePoll::new(CLTRID);

        let serialized = object.serialize().unwrap();

        assert_eq!(xml, serialized);
    }

    #[test]
    fn message_ack() {
        let xml = get_xml("request/message/ack.xml").unwrap();

        let object = EppMessageAck::new(12345, CLTRID);

        let serialized = object.serialize().unwrap();

        assert_eq!(xml, serialized);
    }
}
