//! EPP XML to `EppObject` deserialization tests

mod response {
    use super::super::get_xml;
    use super::super::CLTRID;
    use crate::domain::check::EppDomainCheckResponse;
    use crate::domain::create::EppDomainCreateResponse;
    use crate::epp::response::ExpiryType;
    use crate::epp::response::Relative;
    use crate::epp::response::{
        EppCommandResponseError, EppGreeting, EppLoginResponse, EppLogoutResponse,
    };
    use crate::epp::xml::EppXml;
    use crate::epp::*;

    const SVTRID: &str = "RO-6879-1627224678242975";
    const SUCCESS_MSG: &str = "Command completed successfully";

    #[test]
    fn greeting() {
        let xml = get_xml("response/greeting.xml").unwrap();
        let object = EppGreeting::deserialize(xml.as_str()).unwrap();

        assert_eq!(object.data.service_id, "ISPAPI EPP Server");
        assert_eq!(object.data.service_date, "2021-07-25T14:51:17.0Z");
        assert_eq!(object.data.svc_menu.options.version, "1.0".into());
        assert_eq!(object.data.svc_menu.options.lang, "en".into());
        assert_eq!(object.data.svc_menu.services.obj_uris.len(), 4);
        assert_eq!(
            object
                .data
                .svc_menu
                .services
                .svc_ext
                .unwrap()
                .ext_uris
                .unwrap()
                .len(),
            5
        );
        assert_eq!(object.data.dcp.statement.len(), 2);
        assert_eq!(
            object.data.dcp.expiry.unwrap().ty,
            ExpiryType::Relative(Relative {
                relative: "P1M".into()
            })
        );
    }

    #[test]
    fn error() {
        let xml = get_xml("response/error.xml").unwrap();
        let object = EppCommandResponseError::deserialize(xml.as_str()).unwrap();

        assert_eq!(object.data.result.code, 2303);
        assert_eq!(object.data.result.message, "Object does not exist".into());
        assert_eq!(
            object.data.result.ext_value.unwrap().reason,
            "545 Object not found".into()
        );
        assert_eq!(object.data.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.data.tr_ids.server_tr_id, SVTRID.into());
    }

    #[test]
    fn login() {
        let xml = get_xml("response/login.xml").unwrap();
        let object = EppLoginResponse::deserialize(xml.as_str()).unwrap();

        assert_eq!(object.data.result.code, 1000);
        assert_eq!(object.data.result.message, SUCCESS_MSG.into());
        assert_eq!(object.data.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.data.tr_ids.server_tr_id, SVTRID.into());
    }

    #[test]
    fn logout() {
        let xml = get_xml("response/logout.xml").unwrap();
        let object = EppLogoutResponse::deserialize(xml.as_str()).unwrap();

        assert_eq!(object.data.result.code, 1500);
        assert_eq!(
            object.data.result.message,
            "Command completed successfully; ending session".into()
        );
        assert_eq!(object.data.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.data.tr_ids.server_tr_id, SVTRID.into());
    }

    #[test]
    fn contact_check() {
        let xml = get_xml("response/contact/check.xml").unwrap();
        let object = EppContactCheckResponse::deserialize(xml.as_str()).unwrap();

        let results = object.data.res_data().unwrap();

        assert_eq!(object.data.result.code, 1000);
        assert_eq!(object.data.result.message, SUCCESS_MSG.into());
        assert_eq!(
            results.check_data.contact_list[0].contact.id,
            "eppdev-contact-1".into()
        );
        assert_eq!(results.check_data.contact_list[0].contact.available, 0);
        assert_eq!(
            results.check_data.contact_list[1].contact.id,
            "eppdev-contact-2".into()
        );
        assert_eq!(results.check_data.contact_list[1].contact.available, 1);
        assert_eq!(object.data.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.data.tr_ids.server_tr_id, SVTRID.into());
    }

    #[test]
    fn contact_create() {
        let xml = get_xml("response/contact/create.xml").unwrap();
        let object = EppContactCreateResponse::deserialize(xml.as_str()).unwrap();

        let results = object.data.res_data().unwrap();

        assert_eq!(object.data.result.code, 1000);
        assert_eq!(object.data.result.message, SUCCESS_MSG.into());
        assert_eq!(results.create_data.id, "eppdev-contact-4".into());
        assert_eq!(
            results.create_data.created_at,
            "2021-07-25T16:05:32.0Z".into()
        );
        assert_eq!(object.data.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.data.tr_ids.server_tr_id, SVTRID.into());
    }

    #[test]
    fn contact_delete() {
        let xml = get_xml("response/contact/delete.xml").unwrap();
        let object = EppContactDeleteResponse::deserialize(xml.as_str()).unwrap();

        assert_eq!(object.data.result.code, 1000);
        assert_eq!(object.data.result.message, SUCCESS_MSG.into());
        assert_eq!(object.data.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.data.tr_ids.server_tr_id, SVTRID.into());
    }

    #[test]
    fn contact_info() {
        let xml = get_xml("response/contact/info.xml").unwrap();
        let object = EppContactInfoResponse::deserialize(xml.as_str()).unwrap();

        let result = object.data.res_data().unwrap();
        let fax = result.info_data.fax.as_ref().unwrap();
        let voice_ext = result.info_data.voice.extension.as_ref().unwrap();
        let fax_ext = fax.extension.as_ref().unwrap();
        let auth_info = result.info_data.auth_info.as_ref().unwrap();

        assert_eq!(object.data.result.code, 1000);
        assert_eq!(object.data.result.message, SUCCESS_MSG.into());
        assert_eq!(result.info_data.id, "eppdev-contact-3".into());
        assert_eq!(result.info_data.roid, "UNDEF-ROID".into());
        assert_eq!(result.info_data.statuses[0].status, "ok".to_string());
        assert_eq!(result.info_data.postal_info.info_type, "loc".to_string());
        assert_eq!(result.info_data.postal_info.name, "John Doe".into());
        assert_eq!(
            result.info_data.postal_info.organization,
            "Acme Widgets".into()
        );
        assert_eq!(result.info_data.postal_info.address.street[0], "58".into());
        assert_eq!(
            result.info_data.postal_info.address.street[1],
            "Orchid Road".into()
        );
        assert_eq!(result.info_data.postal_info.address.city, "Paris".into());
        assert_eq!(
            result.info_data.postal_info.address.province,
            "Paris".into()
        );
        assert_eq!(
            result.info_data.postal_info.address.postal_code,
            "392374".into()
        );
        assert_eq!(
            result.info_data.postal_info.address.country_code,
            "FR".into()
        );
        assert_eq!(result.info_data.voice.number, "+33.47237942".to_string());
        assert_eq!(*voice_ext, "123".to_string());
        assert_eq!(fax.number, "+33.86698799".to_string());
        assert_eq!(*fax_ext, "243".to_string());
        assert_eq!(result.info_data.email, "contact@eppdev.net".into());
        assert_eq!(result.info_data.client_id, "eppdev".into());
        assert_eq!(result.info_data.creator_id, "SYSTEM".into());
        assert_eq!(result.info_data.created_at, "2021-07-23T13:09:09.0Z".into());
        assert_eq!(
            *(result.info_data.updater_id.as_ref().unwrap()),
            "SYSTEM".into()
        );
        assert_eq!(
            *(result.info_data.updated_at.as_ref().unwrap()),
            "2021-07-23T13:09:09.0Z".into()
        );
        assert_eq!((*auth_info).password, "eppdev-387323".into());
        assert_eq!(object.data.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.data.tr_ids.server_tr_id, SVTRID.into());
    }

    #[test]
    fn contact_update() {
        let xml = get_xml("response/contact/update.xml").unwrap();
        let object = EppContactUpdateResponse::deserialize(xml.as_str()).unwrap();

        assert_eq!(object.data.result.code, 1000);
        assert_eq!(object.data.result.message, SUCCESS_MSG.into());
        assert_eq!(object.data.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.data.tr_ids.server_tr_id, SVTRID.into());
    }

    #[test]
    fn domain_check() {
        let xml = get_xml("response/domain/check.xml").unwrap();
        let object = EppDomainCheckResponse::deserialize(xml.as_str()).unwrap();

        let result = object.data.res_data().unwrap();

        assert_eq!(object.data.result.code, 1000);
        assert_eq!(object.data.result.message, SUCCESS_MSG.into());
        assert_eq!(
            result.check_data.domain_list[0].domain.name,
            "eppdev.com".into()
        );
        assert_eq!(result.check_data.domain_list[0].domain.available, 1);
        assert_eq!(
            result.check_data.domain_list[1].domain.name,
            "eppdev.net".into()
        );
        assert_eq!(result.check_data.domain_list[1].domain.available, 0);
        assert_eq!(object.data.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.data.tr_ids.server_tr_id, SVTRID.into());
    }

    #[test]
    fn domain_create() {
        let xml = get_xml("response/domain/create.xml").unwrap();
        let object = EppDomainCreateResponse::deserialize(xml.as_str()).unwrap();

        let result = object.data.res_data().unwrap();

        assert_eq!(object.data.result.code, 1000);
        assert_eq!(object.data.result.message, SUCCESS_MSG.into());
        assert_eq!(result.create_data.name, "eppdev-2.com".into());
        assert_eq!(
            result.create_data.created_at,
            "2021-07-25T18:11:35.0Z".into()
        );
        assert_eq!(
            result.create_data.expiring_at,
            "2022-07-25T18:11:34.0Z".into()
        );
        assert_eq!(object.data.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.data.tr_ids.server_tr_id, SVTRID.into());
    }

    #[test]
    fn domain_delete() {
        let xml = get_xml("response/domain/delete.xml").unwrap();
        let object = EppDomainDeleteResponse::deserialize(xml.as_str()).unwrap();

        assert_eq!(object.data.result.code, 1000);
        assert_eq!(object.data.result.message, SUCCESS_MSG.into());
        assert_eq!(object.data.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.data.tr_ids.server_tr_id, SVTRID.into());
    }

    #[test]
    fn domain_info() {
        let xml = get_xml("response/domain/info.xml").unwrap();
        let object = EppDomainInfoResponse::deserialize(xml.as_str()).unwrap();

        let result = object.data.res_data().unwrap();
        let auth_info = result.info_data.auth_info.as_ref().unwrap();
        let ns_list = result.info_data.ns.as_ref().unwrap();
        let ns = (*ns_list).host_obj.as_ref().unwrap();
        let hosts = result.info_data.hosts.as_ref().unwrap();

        assert_eq!(object.data.result.code, 1000);
        assert_eq!(object.data.result.message, SUCCESS_MSG.into());
        assert_eq!(result.info_data.name, "eppdev-1.com".into());
        assert_eq!(result.info_data.roid, "125899511_DOMAIN_COM-VRSN".into());
        assert_eq!(result.info_data.statuses[0].status, "ok".to_string());
        assert_eq!(
            result.info_data.statuses[1].status,
            "clientTransferProhibited".to_string()
        );
        assert_eq!(result.info_data.registrant, "eppdev-contact-2".into());
        assert_eq!(
            result.info_data.contacts[0].id,
            "eppdev-contact-2".to_string()
        );
        assert_eq!(
            result.info_data.contacts[0].contact_type,
            "admin".to_string()
        );
        assert_eq!(
            result.info_data.contacts[1].id,
            "eppdev-contact-2".to_string()
        );
        assert_eq!(
            result.info_data.contacts[1].contact_type,
            "tech".to_string()
        );
        assert_eq!(
            result.info_data.contacts[2].id,
            "eppdev-contact-2".to_string()
        );
        assert_eq!(
            result.info_data.contacts[2].contact_type,
            "billing".to_string()
        );
        assert_eq!((*ns)[0], "ns1.eppdev-1.com".into());
        assert_eq!((*ns)[1], "ns2.eppdev-1.com".into());
        assert_eq!((*hosts)[0], "ns1.eppdev-1.com".into());
        assert_eq!((*hosts)[1], "ns2.eppdev-1.com".into());
        assert_eq!(result.info_data.client_id, "eppdev".into());
        assert_eq!(result.info_data.creator_id, "SYSTEM".into());
        assert_eq!(result.info_data.created_at, "2021-07-23T15:31:20.0Z".into());
        assert_eq!(result.info_data.updater_id, "SYSTEM".into());
        assert_eq!(result.info_data.updated_at, "2021-07-23T15:31:21.0Z".into());
        assert_eq!(
            result.info_data.expiring_at,
            "2023-07-23T15:31:20.0Z".into()
        );
        assert_eq!((*auth_info).password, "epP4uthd#v".into());
        assert_eq!(object.data.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.data.tr_ids.server_tr_id, SVTRID.into());
    }

    #[test]
    fn domain_renew() {
        let xml = get_xml("response/domain/renew.xml").unwrap();
        let object = EppDomainRenewResponse::deserialize(xml.as_str()).unwrap();

        let result = object.data.res_data().unwrap();

        assert_eq!(object.data.result.code, 1000);
        assert_eq!(object.data.result.message, SUCCESS_MSG.into());
        assert_eq!(result.renew_data.name, "eppdev-1.com".into());
        assert_eq!(
            result.renew_data.expiring_at,
            "2024-07-23T15:31:20.0Z".into()
        );
        assert_eq!(object.data.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.data.tr_ids.server_tr_id, SVTRID.into());
    }

    #[test]
    fn domain_transfer_request() {
        let xml = get_xml("response/domain/transfer_request.xml").unwrap();
        let object = EppDomainTransferRequestResponse::deserialize(xml.as_str()).unwrap();

        let result = object.data.res_data().unwrap();

        assert_eq!(object.data.result.code, 1001);
        assert_eq!(
            object.data.result.message,
            "Command completed successfully; action pending".into()
        );
        assert_eq!(result.transfer_data.name, "eppdev-transfer.com".into());
        assert_eq!(result.transfer_data.transfer_status, "pending".into());
        assert_eq!(result.transfer_data.requester_id, "eppdev".into());
        assert_eq!(
            result.transfer_data.requested_at,
            "2021-07-23T15:31:21.0Z".into()
        );
        assert_eq!(result.transfer_data.ack_id, "ClientY".into());
        assert_eq!(result.transfer_data.ack_by, "2021-07-28T15:31:21.0Z".into());
        assert_eq!(
            result.transfer_data.expiring_at,
            "2022-07-02T14:53:19.0Z".into()
        );
        assert_eq!(object.data.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.data.tr_ids.server_tr_id, SVTRID.into());
    }

    #[test]
    fn domain_transfer_approve() {
        let xml = get_xml("response/domain/transfer_approve.xml").unwrap();
        let object = EppDomainTransferApproveResponse::deserialize(xml.as_str()).unwrap();

        assert_eq!(object.data.result.code, 1000);
        assert_eq!(object.data.result.message, SUCCESS_MSG.into());
        assert_eq!(object.data.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.data.tr_ids.server_tr_id, SVTRID.into());
    }

    #[test]
    fn domain_transfer_reject() {
        let xml = get_xml("response/domain/transfer_reject.xml").unwrap();
        let object = EppDomainTransferRejectResponse::deserialize(xml.as_str()).unwrap();

        assert_eq!(object.data.result.code, 1000);
        assert_eq!(object.data.result.message, SUCCESS_MSG.into());
        assert_eq!(object.data.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.data.tr_ids.server_tr_id, SVTRID.into());
    }

    #[test]
    fn domain_transfer_cancel() {
        let xml = get_xml("response/domain/transfer_cancel.xml").unwrap();
        let object = EppDomainTransferCancelResponse::deserialize(xml.as_str()).unwrap();

        assert_eq!(object.data.result.code, 1000);
        assert_eq!(object.data.result.message, SUCCESS_MSG.into());
        assert_eq!(object.data.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.data.tr_ids.server_tr_id, SVTRID.into());
    }

    #[test]
    fn domain_transfer_query() {
        let xml = get_xml("response/domain/transfer_query.xml").unwrap();
        let object = EppDomainTransferQueryResponse::deserialize(xml.as_str()).unwrap();

        let result = object.data.res_data().unwrap();

        assert_eq!(object.data.result.code, 1000);
        assert_eq!(object.data.result.message, SUCCESS_MSG.into());
        assert_eq!(result.transfer_data.name, "eppdev-transfer.com".into());
        assert_eq!(result.transfer_data.transfer_status, "pending".into());
        assert_eq!(result.transfer_data.requester_id, "eppdev".into());
        assert_eq!(
            result.transfer_data.requested_at,
            "2021-07-23T15:31:21.0Z".into()
        );
        assert_eq!(result.transfer_data.ack_id, "ClientY".into());
        assert_eq!(result.transfer_data.ack_by, "2021-07-28T15:31:21.0Z".into());
        assert_eq!(
            result.transfer_data.expiring_at,
            "2022-07-02T14:53:19.0Z".into()
        );
        assert_eq!(object.data.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.data.tr_ids.server_tr_id, SVTRID.into());
    }

    #[test]
    fn domain_update() {
        let xml = get_xml("response/domain/update.xml").unwrap();
        let object = EppDomainUpdateResponse::deserialize(xml.as_str()).unwrap();

        assert_eq!(object.data.result.code, 1000);
        assert_eq!(object.data.result.message, SUCCESS_MSG.into());
        assert_eq!(object.data.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.data.tr_ids.server_tr_id, SVTRID.into());
    }

    #[test]
    fn host_check() {
        let xml = get_xml("response/host/check.xml").unwrap();
        let object = EppHostCheckResponse::deserialize(xml.as_str()).unwrap();

        let result = object.data.res_data().unwrap();

        assert_eq!(object.data.result.code, 1000);
        assert_eq!(object.data.result.message, SUCCESS_MSG.into());
        assert_eq!(
            result.check_data.host_list[0].host.name,
            "host1.eppdev-1.com".into()
        );
        assert_eq!(result.check_data.host_list[0].host.available, 1);
        assert_eq!(
            result.check_data.host_list[1].host.name,
            "ns1.testing.com".into()
        );
        assert_eq!(result.check_data.host_list[1].host.available, 0);
        assert_eq!(object.data.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.data.tr_ids.server_tr_id, SVTRID.into());
    }

    #[test]
    fn host_create() {
        let xml = get_xml("response/host/create.xml").unwrap();
        let object = EppHostCreateResponse::deserialize(xml.as_str()).unwrap();

        let result = object.data.res_data().unwrap();

        assert_eq!(object.data.result.code, 1000);
        assert_eq!(object.data.result.message, SUCCESS_MSG.into());
        assert_eq!(result.create_data.name, "host2.eppdev-1.com".into());
        assert_eq!(
            result.create_data.created_at,
            "2021-07-26T05:28:55.0Z".into()
        );
        assert_eq!(object.data.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.data.tr_ids.server_tr_id, SVTRID.into());
    }

    #[test]
    fn host_info() {
        let xml = get_xml("response/host/info.xml").unwrap();
        let object = EppHostInfoResponse::deserialize(xml.as_str()).unwrap();

        let result = object.data.res_data().unwrap();

        assert_eq!(object.data.result.code, 1000);
        assert_eq!(object.data.result.message, SUCCESS_MSG.into());
        assert_eq!(result.info_data.name, "host2.eppdev-1.com".into());
        assert_eq!(result.info_data.roid, "UNDEF-ROID".into());
        assert_eq!(result.info_data.statuses[0].status, "ok".to_string());
        assert_eq!(
            *(result.info_data.addresses[0].ip_version.as_ref().unwrap()),
            "v4".to_string()
        );
        assert_eq!(
            result.info_data.addresses[0].address,
            "29.245.122.14".to_string()
        );
        assert_eq!(
            *(result.info_data.addresses[1].ip_version.as_ref().unwrap()),
            "v6".to_string()
        );
        assert_eq!(
            result.info_data.addresses[1].address,
            "2404:6800:4001:0801:0000:0000:0000:200e".to_string()
        );
        assert_eq!(result.info_data.client_id, "eppdev".into());
        assert_eq!(result.info_data.creator_id, "creator".into());
        assert_eq!(result.info_data.created_at, "2021-07-26T05:28:55.0Z".into());
        assert_eq!(
            *(result.info_data.updater_id.as_ref().unwrap()),
            "creator".into()
        );
        assert_eq!(
            *(result.info_data.updated_at.as_ref().unwrap()),
            "2021-07-26T05:28:55.0Z".into()
        );
        assert_eq!(object.data.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.data.tr_ids.server_tr_id, SVTRID.into());
    }

    #[test]
    fn host_update() {
        let xml = get_xml("response/host/update.xml").unwrap();
        let object = EppHostUpdateResponse::deserialize(xml.as_str()).unwrap();

        assert_eq!(object.data.result.code, 1000);
        assert_eq!(object.data.result.message, SUCCESS_MSG.into());
        assert_eq!(object.data.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.data.tr_ids.server_tr_id, SVTRID.into());
    }

    #[test]
    fn host_delete() {
        let xml = get_xml("response/host/delete.xml").unwrap();
        let object = EppHostDeleteResponse::deserialize(xml.as_str()).unwrap();

        assert_eq!(object.data.result.code, 1000);
        assert_eq!(object.data.result.message, SUCCESS_MSG.into());
        assert_eq!(object.data.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.data.tr_ids.server_tr_id, SVTRID.into());
    }

    #[test]
    fn message_poll() {
        let xml = get_xml("response/message/poll.xml").unwrap();
        let object = EppMessagePollResponse::deserialize(xml.as_str()).unwrap();

        let result = object.data.res_data().unwrap();
        let msg = object.data.message_queue().unwrap();

        assert_eq!(object.data.result.code, 1301);
        assert_eq!(
            object.data.result.message,
            "Command completed successfully; ack to dequeue".into()
        );
        assert_eq!(msg.count, 5);
        assert_eq!(msg.id, "12345".to_string());
        assert_eq!(
            *(msg.date.as_ref().unwrap()),
            "2021-07-23T19:12:43.0Z".into()
        );
        assert_eq!(
            *(msg.message.as_ref().unwrap()),
            "Transfer requested.".into()
        );
        assert_eq!(result.message_data.name, "eppdev-transfer.com".into());
        assert_eq!(result.message_data.transfer_status, "pending".into());
        assert_eq!(result.message_data.requester_id, "eppdev".into());
        assert_eq!(
            result.message_data.requested_at,
            "2021-07-23T15:31:21.0Z".into()
        );
        assert_eq!(result.message_data.ack_id, "ClientY".into());
        assert_eq!(result.message_data.ack_by, "2021-07-28T15:31:21.0Z".into());
        assert_eq!(
            result.message_data.expiring_at,
            "2022-07-02T14:53:19.0Z".into()
        );
        assert_eq!(object.data.tr_ids.client_tr_id.unwrap(), CLTRID.into());
        assert_eq!(object.data.tr_ids.server_tr_id, SVTRID.into());
    }

    #[test]
    fn message_ack() {
        let xml = get_xml("response/message/ack.xml").unwrap();
        let object = EppMessageAckResponse::deserialize(xml.as_str()).unwrap();

        let msg = object.data.message_queue().unwrap();

        assert_eq!(object.data.result.code, 1000);
        assert_eq!(object.data.result.message, SUCCESS_MSG.into());
        assert_eq!(msg.count, 4);
        assert_eq!(msg.id, "12345".to_string());
        assert_eq!(object.data.tr_ids.server_tr_id, SVTRID.into());
    }

    #[test]
    fn rgp_restore_response() {
        let xml = get_xml("response/domain/rgp_restore.xml").unwrap();
        let object = EppDomainRgpRestoreRequestResponse::deserialize(xml.as_str()).unwrap();

        let ext = object.data.extension.unwrap();

        assert_eq!(object.data.result.code, 1000);
        assert_eq!(object.data.result.message, SUCCESS_MSG.into());
        assert_eq!(ext.data.rgp_status.status, "pendingRestore".to_string());
        assert_eq!(object.data.tr_ids.server_tr_id, SVTRID.into());
    }
}
