mod response {
    use super::super::get_xml;
    use super::super::CLTRID;
    use crate::epp::object::StringValueTrait;
    use crate::epp::response::{
        EppCommandResponse, EppCommandResponseError, EppGreeting, EppLoginResponse,
        EppLogoutResponse,
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
        assert_eq!(
            object.data.svc_menu.options.version,
            "1.0".to_string_value()
        );
        assert_eq!(object.data.svc_menu.options.lang, "en".to_string_value());
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
    }

    #[test]
    fn error() {
        let xml = get_xml("response/error.xml").unwrap();
        let object = EppCommandResponseError::deserialize(xml.as_str()).unwrap();

        assert_eq!(object.data.result.code, 2303);
        assert_eq!(
            object.data.result.message,
            "Object does not exist".to_string_value()
        );
        assert_eq!(
            object.data.result.ext_value.unwrap().reason,
            "545 Object not found".to_string_value()
        );
        assert_eq!(
            object.data.tr_ids.client_tr_id.unwrap(),
            CLTRID.to_string_value()
        );
        assert_eq!(object.data.tr_ids.server_tr_id, SVTRID.to_string_value());
    }

    #[test]
    fn login() {
        let xml = get_xml("response/login.xml").unwrap();
        let object = EppLoginResponse::deserialize(xml.as_str()).unwrap();

        assert_eq!(object.data.result.code, 1000);
        assert_eq!(object.data.result.message, SUCCESS_MSG.to_string_value());
        assert_eq!(
            object.data.tr_ids.client_tr_id.unwrap(),
            CLTRID.to_string_value()
        );
        assert_eq!(object.data.tr_ids.server_tr_id, SVTRID.to_string_value());
    }

    #[test]
    fn logout() {
        let xml = get_xml("response/logout.xml").unwrap();
        let object = EppLogoutResponse::deserialize(xml.as_str()).unwrap();

        assert_eq!(object.data.result.code, 1500);
        assert_eq!(
            object.data.result.message,
            "Command completed successfully; ending session".to_string_value()
        );
        assert_eq!(
            object.data.tr_ids.client_tr_id.unwrap(),
            CLTRID.to_string_value()
        );
        assert_eq!(object.data.tr_ids.server_tr_id, SVTRID.to_string_value());
    }

    #[test]
    fn contact_check() {
        let xml = get_xml("response/contact/check.xml").unwrap();
        let object = EppContactCheckResponse::deserialize(xml.as_str()).unwrap();

        let results = object.data.results().unwrap();

        assert_eq!(object.data.result.code, 1000);
        assert_eq!(object.data.result.message, SUCCESS_MSG.to_string_value());
        assert_eq!(
            results.check_data.contact_list[0].contact.id,
            "eppdev-contact-1".to_string_value()
        );
        assert_eq!(results.check_data.contact_list[0].contact.available, 0);
        assert_eq!(
            results.check_data.contact_list[1].contact.id,
            "eppdev-contact-2".to_string_value()
        );
        assert_eq!(results.check_data.contact_list[1].contact.available, 1);
        assert_eq!(
            object.data.tr_ids.client_tr_id.unwrap(),
            CLTRID.to_string_value()
        );
        assert_eq!(object.data.tr_ids.server_tr_id, SVTRID.to_string_value());
    }

    #[test]
    fn contact_create() {
        let xml = get_xml("response/contact/create.xml").unwrap();
        let object = EppContactCreateResponse::deserialize(xml.as_str()).unwrap();

        let results = object.data.results().unwrap();

        assert_eq!(object.data.result.code, 1000);
        assert_eq!(object.data.result.message, SUCCESS_MSG.to_string_value());
        assert_eq!(results.create_data.id, "eppdev-contact-4".to_string_value());
        assert_eq!(
            results.create_data.created_at,
            "2021-07-25T16:05:32.0Z".to_string_value()
        );
        assert_eq!(
            object.data.tr_ids.client_tr_id.unwrap(),
            CLTRID.to_string_value()
        );
        assert_eq!(object.data.tr_ids.server_tr_id, SVTRID.to_string_value());
    }

    #[test]
    fn contact_delete() {
        let xml = get_xml("response/contact/delete.xml").unwrap();
        let object = EppContactDeleteResponse::deserialize(xml.as_str()).unwrap();

        assert_eq!(object.data.result.code, 1000);
        assert_eq!(object.data.result.message, SUCCESS_MSG.to_string_value());
        assert_eq!(
            object.data.tr_ids.client_tr_id.unwrap(),
            CLTRID.to_string_value()
        );
        assert_eq!(object.data.tr_ids.server_tr_id, SVTRID.to_string_value());
    }

    #[test]
    fn contact_info() {
        let xml = get_xml("response/contact/info.xml").unwrap();
        let object = EppContactInfoResponse::deserialize(xml.as_str()).unwrap();

        let result = object.data.results().unwrap();
        let fax = result.info_data.fax.as_ref().unwrap();
        let voice_ext = result.info_data.voice.extension.as_ref().unwrap();
        let fax_ext = fax.extension.as_ref().unwrap();
        let auth_info = result.info_data.auth_info.as_ref().unwrap();

        assert_eq!(object.data.result.code, 1000);
        assert_eq!(object.data.result.message, SUCCESS_MSG.to_string_value());
        assert_eq!(result.info_data.id, "eppdev-contact-3".to_string_value());
        assert_eq!(result.info_data.roid, "UNDEF-ROID".to_string_value());
        assert_eq!(result.info_data.statuses[0].status, "ok".to_string());
        assert_eq!(result.info_data.postal_info.info_type, "loc".to_string());
        assert_eq!(
            result.info_data.postal_info.name,
            "John Doe".to_string_value()
        );
        assert_eq!(
            result.info_data.postal_info.organization,
            "Acme Widgets".to_string_value()
        );
        assert_eq!(
            result.info_data.postal_info.address.street[0],
            "58".to_string_value()
        );
        assert_eq!(
            result.info_data.postal_info.address.street[1],
            "Orchid Road".to_string_value()
        );
        assert_eq!(
            result.info_data.postal_info.address.city,
            "Paris".to_string_value()
        );
        assert_eq!(
            result.info_data.postal_info.address.province,
            "Paris".to_string_value()
        );
        assert_eq!(
            result.info_data.postal_info.address.postal_code,
            "392374".to_string_value()
        );
        assert_eq!(
            result.info_data.postal_info.address.country_code,
            "FR".to_string_value()
        );
        assert_eq!(result.info_data.voice.number, "+33.47237942".to_string());
        assert_eq!(*voice_ext, "123".to_string());
        assert_eq!(fax.number, "+33.86698799".to_string());
        assert_eq!(*fax_ext, "243".to_string());
        assert_eq!(
            result.info_data.email,
            "contact@eppdev.net".to_string_value()
        );
        assert_eq!(result.info_data.client_id, "eppdev".to_string_value());
        assert_eq!(result.info_data.creator_id, "SYSTEM".to_string_value());
        assert_eq!(
            result.info_data.created_at,
            "2021-07-23T13:09:09.0Z".to_string_value()
        );
        assert_eq!(
            *(result.info_data.updater_id.as_ref().unwrap()),
            "SYSTEM".to_string_value()
        );
        assert_eq!(
            *(result.info_data.updated_at.as_ref().unwrap()),
            "2021-07-23T13:09:09.0Z".to_string_value()
        );
        assert_eq!((*auth_info).password, "eppdev-387323".to_string_value());
        assert_eq!(
            object.data.tr_ids.client_tr_id.unwrap(),
            CLTRID.to_string_value()
        );
        assert_eq!(object.data.tr_ids.server_tr_id, SVTRID.to_string_value());
    }

    #[test]
    fn contact_update() {
        let xml = get_xml("response/contact/update.xml").unwrap();
        let object = EppContactUpdateResponse::deserialize(xml.as_str()).unwrap();

        assert_eq!(object.data.result.code, 1000);
        assert_eq!(object.data.result.message, SUCCESS_MSG.to_string_value());
        assert_eq!(
            object.data.tr_ids.client_tr_id.unwrap(),
            CLTRID.to_string_value()
        );
        assert_eq!(object.data.tr_ids.server_tr_id, SVTRID.to_string_value());
    }

    #[test]
    fn domain_check() {
        let xml = get_xml("response/domain/check.xml").unwrap();
        let object = EppDomainCheckResponse::deserialize(xml.as_str()).unwrap();

        let result = object.data.results().unwrap();

        assert_eq!(object.data.result.code, 1000);
        assert_eq!(object.data.result.message, SUCCESS_MSG.to_string_value());
        assert_eq!(
            result.check_data.domain_list[0].domain.name,
            "eppdev.com".to_string_value()
        );
        assert_eq!(result.check_data.domain_list[0].domain.available, 1);
        assert_eq!(
            result.check_data.domain_list[1].domain.name,
            "eppdev.net".to_string_value()
        );
        assert_eq!(result.check_data.domain_list[1].domain.available, 0);
        assert_eq!(
            object.data.tr_ids.client_tr_id.unwrap(),
            CLTRID.to_string_value()
        );
        assert_eq!(object.data.tr_ids.server_tr_id, SVTRID.to_string_value());
    }

    #[test]
    fn domain_create() {
        let xml = get_xml("response/domain/create.xml").unwrap();
        let object = EppDomainCreateResponse::deserialize(xml.as_str()).unwrap();

        let result = object.data.results().unwrap();

        assert_eq!(object.data.result.code, 1000);
        assert_eq!(object.data.result.message, SUCCESS_MSG.to_string_value());
        assert_eq!(result.create_data.name, "eppdev-2.com".to_string_value());
        assert_eq!(
            result.create_data.created_at,
            "2021-07-25T18:11:35.0Z".to_string_value()
        );
        assert_eq!(
            result.create_data.expiry_date,
            "2022-07-25T18:11:34.0Z".to_string_value()
        );
        assert_eq!(
            object.data.tr_ids.client_tr_id.unwrap(),
            CLTRID.to_string_value()
        );
        assert_eq!(object.data.tr_ids.server_tr_id, SVTRID.to_string_value());
    }

    #[test]
    fn domain_delete() {
        let xml = get_xml("response/domain/delete.xml").unwrap();
        let object = EppDomainDeleteResponse::deserialize(xml.as_str()).unwrap();

        assert_eq!(object.data.result.code, 1000);
        assert_eq!(object.data.result.message, SUCCESS_MSG.to_string_value());
        assert_eq!(
            object.data.tr_ids.client_tr_id.unwrap(),
            CLTRID.to_string_value()
        );
        assert_eq!(object.data.tr_ids.server_tr_id, SVTRID.to_string_value());
    }
}
