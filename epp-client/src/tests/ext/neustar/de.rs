//! EPP XML to `EppObject` deserialization tests for Neustar EPP XML

mod response {
    use super::super::super::super::get_xml;
    use super::super::super::super::{CLTRID, SUCCESS_MSG, SVTRID};
    use crate::epp::object::StringValueTrait;
    use crate::epp::xml::EppXml;
    use crate::epp::*;

    #[test]
    fn contact_info() {
        let xml = get_xml("ext/neustar/response/contact/info.xml").unwrap();
        let object = EppNeustarContactInfoResponse::deserialize(xml.as_str()).unwrap();

        let result = object.data.res_data().unwrap();
        let fax = result.info_data.fax.as_ref().unwrap();
        let voice_ext = result.info_data.voice.extension.as_ref().unwrap();
        let fax_ext = fax.extension.as_ref().unwrap();
        let auth_info = result.info_data.auth_info.as_ref().unwrap();
        let ext = object.data.extension.as_ref().unwrap();

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

        assert_eq!(
            (*ext).data.unspec,
            "appPurpose=P2 nexusCategory=C31/DE".to_string_value()
        );

        assert_eq!(object.data.tr_ids.server_tr_id, SVTRID.to_string_value());
    }
}
