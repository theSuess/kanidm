use crate::constants::uuids::*;
///! Constant Entries for the IDM
use crate::constants::values::*;
use crate::entry::{Entry, EntryInit, EntryInitNew, EntryNew};
use crate::value::Value;

/*
// Template acp
pub const _UUID_IDM_ACP_XX_V1: &str = "00000000-0000-0000-0000-ffffff0000XX";
pub const JSON_IDM_ACP_XX_V1: &str = r#"{
    "attrs": {
        "class": [
            "object",
            "access_control_profile",
            "access_control_search",
            "access_control_modify",
            "access_control_create",
            "access_control_delete"
        ],
        "name": ["idm_acp_xx"],
        "uuid": ["00000000-0000-0000-0000-ffffff0000XX"],
        "description": ["Builtin IDM Control for xx"],
        "acp_receiver": [
            "{\"eq\":[\"memberof\",\"00000000-0000-0000-0000-0000000000XX\"]}"
        ],
        "acp_targetscope": [
            "{\"and\": [{\"eq\": [\"attr\",\"value\"]}, {\"andnot\": {\"or\": [{\"eq\": [\"class\", \"tombstone\"]}, {\"eq\": [\"class\", \"recycled\"]}]}}]}"
        ],
        "acp_search_attr": [

        ],
        "acp_modify_removedattr": [

        ],
        "acp_modify_presentattr": [

        ],
        "acp_modify_class":  [

        ],
        "acp_create_attr": [

        ],
        "acp_create_class": [

        ]
    }
}"#;
*/

pub const JSON_IDM_ADMINS_ACP_RECYCLE_SEARCH_V1: &str = r#"{
    "attrs": {
        "class": ["object", "access_control_profile", "access_control_search"],
        "name": ["idm_admins_acp_recycle_search"],
        "uuid": ["00000000-0000-0000-0000-ffffff000002"],
        "description": ["Builtin IDM admin recycle bin search permission."],
        "acp_receiver": [],
        "acp_receiver_group": ["00000000-0000-0000-0000-000000000019"],
        "acp_targetscope": [
            "{\"eq\": [\"class\", \"recycled\"]}"
        ],
        "acp_search_attr": ["name", "class", "uuid", "last_modified_cid"]
    }
}"#;

pub const JSON_IDM_ADMINS_ACP_REVIVE_V1: &str = r#"{
    "attrs": {
        "class": ["object", "access_control_profile", "access_control_modify"],
        "name": ["idm_admins_acp_revive"],
        "uuid": ["00000000-0000-0000-0000-ffffff000003"],
        "description": ["Builtin IDM Administrators Access Controls."],
        "acp_receiver": [],
        "acp_receiver_group": ["00000000-0000-0000-0000-000000000019"],
        "acp_targetscope": [
            "{\"eq\":[\"class\",\"recycled\"]}"
        ],
        "acp_modify_removedattr": ["class"],
        "acp_modify_class": ["recycled"]
    }
}"#;

pub const JSON_IDM_SELF_ACP_READ_V1: &str = r#"{
    "attrs": {
        "class": ["object", "access_control_profile", "access_control_search"],
        "name": ["idm_self_acp_read"],
        "uuid": ["00000000-0000-0000-0000-ffffff000004"],
        "description": ["Builtin IDM Control for self read - required for whoami and many other functions."],
        "acp_receiver": [],
        "acp_receiver_group": ["00000000-0000-0000-0000-000000000036"],
        "acp_targetscope": [
            "\"self\""
        ],
        "acp_search_attr": [
            "name",
            "spn",
            "displayname",
            "legalname",
            "class",
            "memberof",
            "radius_secret",
            "gidnumber",
            "loginshell",
            "uuid",
            "account_expire",
            "account_valid_from",
            "primary_credential",
            "user_auth_token_session",
            "passkeys",
            "devicekeys"
        ]
    }
}"#;

pub const JSON_IDM_SELF_ACP_WRITE_V1: &str = r#"{
    "attrs": {
        "class": ["object", "access_control_profile", "access_control_modify"],
        "name": ["idm_self_acp_write"],
        "uuid": ["00000000-0000-0000-0000-ffffff000021"],
        "description": ["Builtin IDM Control for self write - required for people to update their own identities and credentials in line with best practices."],
        "acp_receiver": [],
        "acp_receiver_group": ["00000000-0000-0000-0000-000000000035"],
        "acp_targetscope": [
            "{\"and\": [{\"eq\": [\"class\",\"person\"]}, {\"eq\": [\"class\",\"account\"]}, \"self\"]}"
        ],
        "acp_modify_removedattr": [
            "name", "displayname", "legalname", "radius_secret", "primary_credential", "ssh_publickey", "unix_password", "passkeys", "devicekeys", "user_auth_token_session"
        ],
        "acp_modify_presentattr": [
            "name", "displayname", "legalname", "radius_secret", "primary_credential", "ssh_publickey", "unix_password", "passkeys", "devicekeys"
        ]
    }
}"#;

pub const JSON_IDM_PEOPLE_SELF_ACP_WRITE_MAIL_PRIV_V1: &str = r#"{
    "attrs": {
        "class": ["object", "access_control_profile", "access_control_modify"],
        "name": ["idm_people_self_acp_write_mail"],
        "uuid": ["00000000-0000-0000-0000-ffffff000041"],
        "description": ["Builtin IDM Control for self write of mail for people accounts."],
        "acp_receiver": [],
        "acp_receiver_group": ["00000000-0000-0000-0000-000000000033"],
        "acp_targetscope": [
            "{\"and\": [{\"eq\": [\"class\",\"person\"]}, {\"eq\": [\"class\",\"account\"]}, \"self\"]}"
        ],
        "acp_modify_removedattr": [
            "mail"
        ],
        "acp_modify_presentattr": [
            "mail"
        ]
    }
}"#;

pub const JSON_IDM_ALL_ACP_READ_V1: &str = r#"{
    "attrs": {
        "class": ["object", "access_control_profile", "access_control_search"],
        "name": ["idm_all_acp_read"],
        "uuid": ["00000000-0000-0000-0000-ffffff000006"],
        "description": ["Builtin IDM Control for all read - IE anonymous and all authenticated accounts."],
        "acp_receiver": [],
        "acp_receiver_group": ["00000000-0000-0000-0000-000000000036"],
        "acp_targetscope": [
            "{\"and\": [{\"pres\": \"class\"}, {\"andnot\": {\"or\": [{\"eq\": [\"class\", \"tombstone\"]}, {\"eq\": [\"class\", \"recycled\"]}]}}]}"
        ],
        "acp_search_attr": [
            "name",
            "spn",
            "displayname",
            "class",
            "memberof",
            "member",
            "uuid",
            "gidnumber",
            "loginshell",
            "ssh_publickey"
        ]
    }
}"#;

// 7 people read acp JSON_IDM_PEOPLE_READ_PRIV_V1
pub const JSON_IDM_ACP_PEOPLE_READ_PRIV_V1: &str = r#"{
    "attrs": {
        "class": [
            "object",
            "access_control_profile",
            "access_control_search"
        ],
        "name": ["idm_acp_people_read_priv"],
        "uuid": ["00000000-0000-0000-0000-ffffff000007"],
        "description": ["Builtin IDM Control for reading personal sensitive data."],
        "acp_receiver": [],
        "acp_receiver_group": ["00000000-0000-0000-0000-000000000002"],
        "acp_targetscope": [
            "{\"and\": [{\"eq\": [\"class\",\"person\"]}, {\"andnot\": {\"or\": [{\"eq\": [\"memberof\",\"00000000-0000-0000-0000-000000001000\"]}, {\"eq\": [\"class\", \"tombstone\"]}, {\"eq\": [\"class\", \"recycled\"]}]}}]}"
        ],
        "acp_search_attr": [
            "name", "displayname", "legalname", "mail"
        ]
    }
}"#;
// 8 people write acp JSON_IDM_PEOPLE_WRITE_PRIV_V1
pub const JSON_IDM_ACP_PEOPLE_WRITE_PRIV_V1: &str = r#"{
    "attrs": {
        "class": [
            "object",
            "access_control_profile",
            "access_control_modify"
        ],
        "name": ["idm_acp_people_write_priv"],
        "uuid": ["00000000-0000-0000-0000-ffffff000008"],
        "description": ["Builtin IDM Control for managing personal and sensitive data."],
        "acp_receiver": [],
        "acp_receiver_group": ["00000000-0000-0000-0000-000000000003"],
        "acp_targetscope": [
            "{\"and\": [{\"eq\": [\"class\",\"person\"]}, {\"andnot\": {\"or\": [{\"eq\": [\"memberof\",\"00000000-0000-0000-0000-000000001000\"]}, {\"eq\": [\"class\", \"tombstone\"]}, {\"eq\": [\"class\", \"recycled\"]}]}}]}"
        ],
        "acp_modify_removedattr": [
            "name", "displayname", "legalname", "mail"
        ],
        "acp_modify_presentattr": [
            "name", "displayname", "legalname", "mail"
        ]
    }
}"#;
// 13 user (person) account create acp  JSON_IDM_PERSON_ACCOUNT_CREATE_PRIV_V1
pub const JSON_IDM_ACP_PEOPLE_MANAGE_PRIV_V1: &str = r#"{
    "attrs": {
        "class": [
            "object",
            "access_control_profile",
            "access_control_delete",
            "access_control_create"
        ],
        "name": ["idm_acp_people_manage"],
        "uuid": ["00000000-0000-0000-0000-ffffff000013"],
        "description": ["Builtin IDM Control for creating person (user) accounts"],
        "acp_receiver": [],
        "acp_receiver_group": ["00000000-0000-0000-0000-000000000013"],
        "acp_targetscope": [
            "{\"and\": [{\"eq\": [\"class\",\"account\"]}, {\"eq\": [\"class\",\"person\"]}, {\"andnot\": {\"or\": [{\"eq\": [\"memberof\",\"00000000-0000-0000-0000-000000001000\"]}, {\"eq\": [\"class\", \"tombstone\"]}, {\"eq\": [\"class\", \"recycled\"]}]}}]}"
        ],
        "acp_create_attr": [
            "class",
            "name",
            "displayname",
            "legalname",
            "primary_credential",
            "passkeys",
            "devicekeys",
            "user_auth_token_session",
            "ssh_publickey",
            "mail"
        ],
        "acp_create_class": [
            "object", "person", "account"
        ]
    }
}"#;
// 31 - password import modification priv
// right now, create requires you to have access to every attribute in a single snapshot,
// so people will need to two step (create then import pw). Later we could add another
// acp that allows the create here too? Should it be seperate?
pub const JSON_IDM_ACP_PEOPLE_ACCOUNT_PASSWORD_IMPORT_PRIV_V1: &str = r#"{
    "attrs": {
        "class": [
            "object",
            "access_control_profile",
            "access_control_modify"
        ],
        "name": ["idm_acp_people_account_password_import_priv"],
        "uuid": ["00000000-0000-0000-0000-ffffff000031"],
        "description": ["Builtin IDM Control for allowing imports of passwords to people+account types."],
        "acp_receiver": [],
        "acp_receiver_group": ["00000000-0000-0000-0000-000000000023"],
        "acp_targetscope": [
            "{\"and\": [{\"eq\": [\"class\",\"person\"]}, {\"eq\": [\"class\",\"account\"]}, {\"andnot\": {\"or\": [{\"eq\": [\"memberof\",\"00000000-0000-0000-0000-000000001000\"]}, {\"eq\": [\"class\", \"tombstone\"]}, {\"eq\": [\"class\", \"recycled\"]}]}}]}"
        ],
        "acp_modify_removedattr": [
            "password_import"
        ],
        "acp_modify_presentattr": [
            "password_import"
        ]
    }
}"#;

//
pub const JSON_IDM_ACP_PEOPLE_EXTEND_PRIV_V1: &str = r#"{
    "attrs": {
        "class": [
            "object",
            "access_control_profile",
            "access_control_modify"
        ],
        "name": ["idm_acp_people_extend_priv"],
        "uuid": ["00000000-0000-0000-0000-ffffff000032"],
        "description": ["Builtin IDM Control for allowing person class extension"],
        "acp_receiver": [],
        "acp_receiver_group": ["00000000-0000-0000-0000-000000000024"],
        "acp_targetscope": [
            "{\"and\": [{\"eq\": [\"class\",\"account\"]}, {\"andnot\": {\"or\": [{\"eq\": [\"memberof\",\"00000000-0000-0000-0000-000000001000\"]}, {\"eq\": [\"class\", \"tombstone\"]}, {\"eq\": [\"class\", \"recycled\"]}]}}]}"
        ],
        "acp_modify_removedattr": [
            "name", "displayname", "legalname", "mail"
        ],
        "acp_modify_presentattr": [
            "class", "name", "displayname", "legalname", "mail"
        ],
        "acp_modify_class": ["person"]
    }
}"#;

// -- hp people
pub const JSON_IDM_ACP_HP_PEOPLE_READ_PRIV_V1: &str = r#"{
    "attrs": {
        "class": [
            "object",
            "access_control_profile",
            "access_control_search"
        ],
        "name": ["idm_acp_hp_people_read_priv"],
        "uuid": ["00000000-0000-0000-0000-ffffff000036"],
        "description": ["Builtin IDM Control for reading high privilege personal sensitive data."],
        "acp_receiver": [],
        "acp_receiver_group": ["00000000-0000-0000-0000-000000000028"],
        "acp_targetscope": [
            "{\"and\": [{\"eq\": [\"class\",\"person\"]}, {\"eq\": [\"memberof\",\"00000000-0000-0000-0000-000000001000\"]}, {\"andnot\": {\"or\": [{\"eq\": [\"class\", \"tombstone\"]}, {\"eq\": [\"class\", \"recycled\"]}]}}]}"
        ],
        "acp_search_attr": [
            "name", "displayname", "legalname", "mail"
        ]
    }
}"#;

lazy_static! {
    pub static ref E_IDM_ACP_ACCOUNT_MAIL_READ_PRIV_V1: EntryInitNew = entry_init!(
        ("class", CLASS_OBJECT.clone()),
        ("class", CLASS_ACCESS_CONTROL_PROFILE.clone()),
        ("class", CLASS_ACCESS_CONTROL_SEARCH.clone()),
        (
            "name",
            Value::new_iname("idm_acp_account_mail_read_priv")
        ),
        (
            "uuid",
            Value::Uuid(UUID_IDM_ACP_ACCOUNT_MAIL_READ_PRIV_V1)
        ),
        (
            "description",
            Value::new_utf8s(
                "Builtin IDM Control for reading account mail attributes."
            )
        ),
        (
            "acp_receiver_group",
            Value::Refer(UUID_IDM_ACCOUNT_MAIL_READ_PRIV)
        ),
        (
            "acp_targetscope",
            #[allow(clippy::expect_used)]
            Value::new_json_filter_s("{\"and\": [{\"eq\": [\"class\",\"account\"]}, {\"andnot\": {\"or\": [{\"eq\": [\"class\", \"tombstone\"]}, {\"eq\": [\"class\", \"recycled\"]}]}}]}").expect("filter")
        ),
        ("acp_search_attr", Value::new_iutf8("mail"))
    );
}

pub const JSON_IDM_ACP_HP_PEOPLE_WRITE_PRIV_V1: &str = r#"{
    "attrs": {
        "class": [
            "object",
            "access_control_profile",
            "access_control_modify"
        ],
        "name": ["idm_acp_hp_people_write_priv"],
        "uuid": ["00000000-0000-0000-0000-ffffff000037"],
        "description": ["Builtin IDM Control for managing privilege personal and sensitive data."],
        "acp_receiver": [],
        "acp_receiver_group": ["00000000-0000-0000-0000-000000000029"],
        "acp_targetscope": [
            "{\"and\": [{\"eq\": [\"class\",\"person\"]}, {\"eq\": [\"memberof\",\"00000000-0000-0000-0000-000000001000\"]}, {\"andnot\": {\"or\": [{\"eq\": [\"class\", \"tombstone\"]}, {\"eq\": [\"class\", \"recycled\"]}]}}]}"
        ],
        "acp_modify_removedattr": [
            "name", "displayname", "legalname", "mail"
        ],
        "acp_modify_presentattr": [
            "name", "displayname", "legalname", "mail"
        ]
    }
}"#;

pub const JSON_IDM_ACP_HP_PEOPLE_EXTEND_PRIV_V1: &str = r#"{
    "attrs": {
        "class": [
            "object",
            "access_control_profile",
            "access_control_modify"
        ],
        "name": ["idm_acp_hp_people_extend_priv"],
        "uuid": ["00000000-0000-0000-0000-ffffff000038"],
        "description": ["Builtin IDM Control for allowing privilege person class extension"],
        "acp_receiver": [],
        "acp_receiver_group": ["00000000-0000-0000-0000-000000000030"],
        "acp_targetscope": [
            "{\"and\": [{\"eq\": [\"class\",\"account\"]}, {\"eq\": [\"memberof\",\"00000000-0000-0000-0000-000000001000\"]}, {\"andnot\": {\"or\": [{\"eq\": [\"class\", \"tombstone\"]}, {\"eq\": [\"class\", \"recycled\"]}]}}]}"
        ],
        "acp_modify_removedattr": [
            "name", "displayname", "legalname", "mail"
        ],
        "acp_modify_presentattr": [
            "class", "name", "displayname", "legalname", "mail"
        ],
        "acp_modify_class": ["person"]
    }
}"#;

// -- end people

// 9 group write acp JSON_IDM_GROUP_WRITE_PRIV_V1
pub const JSON_IDM_ACP_GROUP_WRITE_PRIV_V1: &str = r#"{
    "attrs": {
        "class": [
            "object",
            "access_control_profile",
            "access_control_search",
            "access_control_modify"
        ],
        "name": ["idm_acp_group_write_priv"],
        "uuid": ["00000000-0000-0000-0000-ffffff000009"],
        "description": ["Builtin IDM Control for managing groups"],
        "acp_receiver": [],
        "acp_receiver_group": ["00000000-0000-0000-0000-000000000004"],
        "acp_targetscope": [
            "{\"and\": [{\"eq\": [\"class\",\"group\"]}, {\"andnot\": {\"or\": [{\"eq\": [\"memberof\",\"00000000-0000-0000-0000-000000001000\"]}, {\"eq\": [\"class\", \"tombstone\"]}, {\"eq\": [\"class\", \"recycled\"]}]}}]}"
        ],
        "acp_search_attr": [
            "class", "name", "spn", "uuid", "description", "member"
        ],
        "acp_modify_removedattr": [
            "name", "description", "member"
        ],
        "acp_modify_presentattr": [
            "name", "description", "member"
        ]
    }
}"#;
// 10 account read acp JSON_IDM_ACCOUNT_READ_PRIV_V1
pub const JSON_IDM_ACP_ACCOUNT_READ_PRIV_V1: &str = r#"{
    "attrs": {
        "class": [
            "object",
            "access_control_profile",
            "access_control_search"
        ],
        "name": ["idm_acp_account_read_priv"],
        "uuid": ["00000000-0000-0000-0000-ffffff000010"],
        "description": ["Builtin IDM Control for accounts."],
        "acp_receiver": [],
        "acp_receiver_group": ["00000000-0000-0000-0000-000000000005"],
        "acp_targetscope": [
            "{\"and\": [{\"eq\": [\"class\",\"account\"]}, {\"andnot\": {\"or\": [{\"eq\": [\"memberof\",\"00000000-0000-0000-0000-000000001000\"]}, {\"eq\": [\"class\", \"tombstone\"]}, {\"eq\": [\"class\", \"recycled\"]}]}}]}"
        ],
        "acp_search_attr": [
            "class", "name", "spn", "uuid", "displayname", "ssh_publickey", "primary_credential", "memberof", "mail", "gidnumber", "account_expire", "account_valid_from", "passkeys", "devicekeys", "api_token_session", "user_auth_token_session"
        ]
    }
}"#;
// 11 account write acp JSON_IDM_ACCOUNT_WRITE_PRIV_V1
pub const JSON_IDM_ACP_ACCOUNT_WRITE_PRIV_V1: &str = r#"{
    "attrs": {
        "class": [
            "object",
            "access_control_profile",
            "access_control_modify"
        ],
        "name": ["idm_acp_account_write_priv"],
        "uuid": ["00000000-0000-0000-0000-ffffff000011"],
        "description": ["Builtin IDM Control for managing all accounts (both person and service)."],
        "acp_receiver": [],
        "acp_receiver_group": ["00000000-0000-0000-0000-000000000006"],
        "acp_targetscope": [
            "{\"and\": [{\"eq\": [\"class\",\"account\"]}, {\"andnot\": {\"or\": [{\"eq\": [\"memberof\",\"00000000-0000-0000-0000-000000001000\"]}, {\"eq\": [\"class\", \"tombstone\"]}, {\"eq\": [\"class\", \"recycled\"]}]}}]}"
        ],
        "acp_modify_removedattr": [
            "name", "displayname", "ssh_publickey", "primary_credential", "mail", "account_expire", "account_valid_from", "passkeys", "devicekeys", "api_token_session", "user_auth_token_session"
        ],
        "acp_modify_presentattr": [
            "name", "displayname", "ssh_publickey", "primary_credential", "mail", "account_expire", "account_valid_from", "passkeys", "devicekeys", "api_token_session"
        ]
    }
}"#;
// 12 service account create acp (only admins?)  JSON_IDM_SERVICE_ACCOUNT_CREATE_PRIV_V1
pub const JSON_IDM_ACP_ACCOUNT_MANAGE_PRIV_V1: &str = r#"{
    "attrs": {
        "class": [
            "object",
            "access_control_profile",
            "access_control_delete",
            "access_control_create"
        ],
        "name": ["idm_acp_account_manage"],
        "uuid": ["00000000-0000-0000-0000-ffffff000012"],
        "description": ["Builtin IDM Control for creating and deleting (service) accounts"],
        "acp_receiver": [],
        "acp_receiver_group": ["00000000-0000-0000-0000-000000000014"],
        "acp_targetscope": [
            "{\"and\": [{\"eq\": [\"class\",\"account\"]}, {\"andnot\": {\"or\": [{\"eq\": [\"memberof\",\"00000000-0000-0000-0000-000000001000\"]}, {\"eq\": [\"class\", \"tombstone\"]}, {\"eq\": [\"class\", \"recycled\"]}]}}]}"
        ],
        "acp_create_attr": [
            "class",
            "name",
            "displayname",
            "description",
            "primary_credential",
            "ssh_publickey",
            "mail",
            "account_expire",
            "account_valid_from",
            "passkeys",
            "devicekeys"
        ],
        "acp_create_class": [
            "object", "account", "service_account"
        ]
    }
}"#;

// 14 radius read acp JSON_IDM_RADIUS_SERVERS_V1
// The targetscope of this could change later to a "radius access" group or similar so we can add/remove
//  users from having radius access easier.

pub const JSON_IDM_ACP_RADIUS_SECRET_READ_PRIV_V1: &str = r#"{
    "attrs": {
        "class": [
            "object",
            "access_control_profile",
            "access_control_search"
        ],
        "name": ["idm_acp_radius_secret_read_priv"],
        "uuid": ["00000000-0000-0000-0000-ffffff000039"],
        "description": ["Builtin IDM Control for reading radius secrets of accounts."],
        "acp_receiver": [],
        "acp_receiver_group": ["00000000-0000-0000-0000-000000000032"],
        "acp_targetscope": [
            "{\"and\": [{\"eq\": [\"class\",\"account\"]}, {\"andnot\": {\"or\": [{\"eq\": [\"memberof\",\"00000000-0000-0000-0000-000000001000\"]}, {\"eq\": [\"class\", \"tombstone\"]}, {\"eq\": [\"class\", \"recycled\"]}]}}]}"
        ],
        "acp_search_attr": [
            "radius_secret"
        ]
    }
}"#;

pub const JSON_IDM_ACP_RADIUS_SECRET_WRITE_PRIV_V1: &str = r#"{
    "attrs": {
        "class": [
            "object",
            "access_control_profile",
            "access_control_modify"
        ],
        "name": ["idm_acp_radius_secret_write_priv"],
        "uuid": ["00000000-0000-0000-0000-ffffff000040"],
        "description": ["Builtin IDM Control allowing writes to user radius secrets."],
        "acp_receiver": [],
        "acp_receiver_group": ["00000000-0000-0000-0000-000000000031"],
        "acp_targetscope": [
            "{\"and\": [{\"eq\": [\"class\",\"account\"]}, {\"andnot\": {\"or\": [{\"eq\": [\"memberof\",\"00000000-0000-0000-0000-000000001000\"]}, {\"eq\": [\"class\", \"tombstone\"]}, {\"eq\": [\"class\", \"recycled\"]}]}}]}"
        ],
        "acp_modify_removedattr": [
            "radius_secret"
        ],
        "acp_modify_presentattr": [
            "radius_secret"
        ]
    }
}"#;

pub const JSON_IDM_ACP_RADIUS_SERVERS_V1: &str = r#"{
    "attrs": {
        "class": [
            "object",
            "access_control_profile",
            "access_control_search"
        ],
        "name": ["idm_acp_radius_servers"],
        "uuid": ["00000000-0000-0000-0000-ffffff000014"],
        "description": ["Builtin IDM Control for RADIUS servers to read credentials and other needed details."],
        "acp_receiver": [],
        "acp_receiver_group": ["00000000-0000-0000-0000-000000000007"],
        "acp_targetscope": [
            "{\"and\": [{\"pres\": \"class\"}, {\"andnot\": {\"or\": [{\"eq\": [\"class\", \"tombstone\"]}, {\"eq\": [\"class\", \"recycled\"]}]}}]}"
        ],
        "acp_search_attr": [
            "name", "spn", "uuid", "radius_secret"
        ]
    }
}"#;

// 15 high priv account read JSON_IDM_HP_ACCOUNT_READ_PRIV_V1
pub const JSON_IDM_ACP_HP_ACCOUNT_READ_PRIV_V1: &str = r#"{
    "attrs": {
        "class": [
            "object",
            "access_control_profile",
            "access_control_search"
        ],
        "name": ["idm_acp_hp_account_read_priv"],
        "uuid": ["00000000-0000-0000-0000-ffffff000015"],
        "description": ["Builtin IDM Control for reading high privilege accounts."],
        "acp_receiver": [],
        "acp_receiver_group": ["00000000-0000-0000-0000-000000000009"],
        "acp_targetscope": [
            "{\"and\": [{\"eq\": [\"class\",\"account\"]}, {\"eq\": [\"memberof\",\"00000000-0000-0000-0000-000000001000\"]}, {\"andnot\": {\"or\": [{\"eq\": [\"class\", \"tombstone\"]}, {\"eq\": [\"class\", \"recycled\"]}]}}]}"
        ],
        "acp_search_attr": [
            "class", "name", "spn", "uuid", "displayname", "ssh_publickey", "primary_credential", "memberof", "account_expire", "account_valid_from", "passkeys", "devicekeys", "api_token_session", "user_auth_token_session"
        ]
    }
}"#;
// 16 high priv account write JSON_IDM_HP_ACCOUNT_WRITE_PRIV_V1
pub const JSON_IDM_ACP_HP_ACCOUNT_WRITE_PRIV_V1: &str = r#"{
    "attrs": {
        "class": [
            "object",
            "access_control_profile",
            "access_control_modify"
        ],
        "name": ["idm_acp_hp_account_write_priv"],
        "uuid": ["00000000-0000-0000-0000-ffffff000016"],
        "description": ["Builtin IDM Control for managing high privilege accounts (both person and service)."],
        "acp_receiver": [],
        "acp_receiver_group": ["00000000-0000-0000-0000-000000000009"],
        "acp_targetscope": [
            "{\"and\": [{\"eq\": [\"class\",\"account\"]}, {\"eq\": [\"memberof\",\"00000000-0000-0000-0000-000000001000\"]}, {\"andnot\": {\"or\": [{\"eq\": [\"class\", \"tombstone\"]}, {\"eq\": [\"class\", \"recycled\"]}]}}]}"
        ],
        "acp_modify_removedattr": [
            "name", "displayname", "ssh_publickey", "primary_credential", "account_expire", "account_valid_from", "passkeys", "devicekeys", "api_token_session", "user_auth_token_session"
        ],
        "acp_modify_presentattr": [
            "name", "displayname", "ssh_publickey", "primary_credential", "account_expire", "account_valid_from", "passkeys", "devicekeys", "api_token_session"
        ]
    }
}"#;

// 17 high priv group write --> JSON_IDM_HP_GROUP_WRITE_PRIV_V1 (12)
pub const JSON_IDM_ACP_HP_GROUP_WRITE_PRIV_V1: &str = r#"{
    "attrs": {
        "class": [
            "object",
            "access_control_profile",
            "access_control_search",
            "access_control_modify"
        ],
        "name": ["idm_acp_hp_group_write_priv"],
        "uuid": ["00000000-0000-0000-0000-ffffff000017"],
        "description": ["Builtin IDM Control for managing high privilege groups"],
        "acp_receiver": [],
        "acp_receiver_group": ["00000000-0000-0000-0000-000000000012"],
        "acp_targetscope": [
            "{\"and\": [{\"eq\": [\"class\",\"group\"]}, {\"eq\": [\"memberof\",\"00000000-0000-0000-0000-000000001000\"]}, {\"andnot\": {\"or\": [{\"eq\": [\"class\", \"tombstone\"]}, {\"eq\": [\"class\", \"recycled\"]}]}}]}"
        ],
        "acp_search_attr": [
            "class", "name", "uuid", "description", "member"
        ],
        "acp_modify_removedattr": [
            "name", "description", "member"
        ],
        "acp_modify_presentattr": [
            "name", "description", "member"
        ]
    }
}"#;

// 18 schema write JSON_IDM_SCHEMA_WRITE_PRIV_V1
pub const JSON_IDM_ACP_SCHEMA_WRITE_ATTRS_PRIV_V1: &str = r#"{
    "attrs": {
        "class": [
            "object",
            "access_control_profile",
            "access_control_search",
            "access_control_modify",
            "access_control_create"
        ],
        "name": ["idm_acp_schema_write_attrs_priv"],
        "uuid": ["00000000-0000-0000-0000-ffffff000018"],
        "description": ["Builtin IDM Control for management of schema attributes."],
        "acp_receiver": [],
        "acp_receiver_group": ["00000000-0000-0000-0000-000000000010"],
        "acp_targetscope": [
            "{\"and\": [{\"eq\": [\"class\",\"attributetype\"]}, {\"andnot\": {\"or\": [{\"eq\": [\"class\", \"tombstone\"]}, {\"eq\": [\"class\", \"recycled\"]}]}}]}"
        ],
        "acp_search_attr": [
            "class",
            "description",
            "index",
            "unique",
            "multivalue",
            "attributename",
            "syntax",
            "uuid"
        ],
        "acp_modify_removedattr": [
            "description",
            "index",
            "unique",
            "multivalue",
            "syntax"
        ],
        "acp_modify_presentattr": [
            "description",
            "index",
            "unique",
            "multivalue",
            "syntax"
        ],
        "acp_modify_class":  [],
        "acp_create_attr": [
            "class",
            "description",
            "index",
            "unique",
            "multivalue",
            "attributename",
            "syntax",
            "uuid"
        ],
        "acp_create_class": [
            "object", "attributetype"
        ]
    }
}"#;

// 19 acp read/write
pub const JSON_IDM_ACP_ACP_MANAGE_PRIV_V1: &str = r#"{
    "attrs": {
        "class": [
            "object",
            "access_control_profile",
            "access_control_search",
            "access_control_modify",
            "access_control_create",
            "access_control_delete"
        ],
        "name": ["idm_acp_acp_manage_priv"],
        "uuid": ["00000000-0000-0000-0000-ffffff000019"],
        "description": ["Builtin IDM Control for access profiles management."],
        "acp_receiver": [],
        "acp_receiver_group": ["00000000-0000-0000-0000-000000000011"],
        "acp_targetscope": [
            "{\"and\": [{\"eq\": [\"class\",\"access_control_profile\"]}, {\"andnot\": {\"or\": [{\"eq\": [\"class\", \"tombstone\"]}, {\"eq\": [\"class\", \"recycled\"]}]}}]}"
        ],
        "acp_search_attr": [
            "name",
            "class",
            "description",
            "acp_enable",
            "acp_receiver_group",
            "acp_targetscope",
            "acp_search_attr",
            "acp_modify_removedattr",
            "acp_modify_presentattr",
            "acp_modify_class",
            "acp_create_class",
            "acp_create_attr"
        ],
        "acp_modify_removedattr": [
            "name",
            "class",
            "description",
            "acp_enable",
            "acp_receiver_group",
            "acp_targetscope",
            "acp_search_attr",
            "acp_modify_removedattr",
            "acp_modify_presentattr",
            "acp_modify_class",
            "acp_create_class",
            "acp_create_attr"
        ],
        "acp_modify_presentattr": [
            "name",
            "class",
            "description",
            "acp_enable",
            "acp_receiver_group",
            "acp_targetscope",
            "acp_search_attr",
            "acp_modify_removedattr",
            "acp_modify_presentattr",
            "acp_modify_class",
            "acp_create_class",
            "acp_create_attr"
        ],
        "acp_modify_class":  [
            "access_control_profile",
            "access_control_search",
            "access_control_modify",
            "access_control_create",
            "access_control_delete"
        ],
        "acp_create_attr": [
            "name",
            "class",
            "description",
            "acp_enable",
            "acp_receiver_group",
            "acp_targetscope",
            "acp_search_attr",
            "acp_modify_removedattr",
            "acp_modify_presentattr",
            "acp_modify_class",
            "acp_create_class",
            "acp_create_attr"
        ],
        "acp_create_class": [
            "access_control_profile",
            "access_control_search",
            "access_control_modify",
            "access_control_create",
            "access_control_delete"
        ]
    }
}"#;

pub const JSON_IDM_ACP_SCHEMA_WRITE_CLASSES_PRIV_V1: &str = r#"{
    "attrs": {
        "class": [
            "object",
            "access_control_profile",
            "access_control_search",
            "access_control_modify",
            "access_control_create"
        ],
        "name": ["idm_acp_schema_write_classes_priv"],
        "uuid": ["00000000-0000-0000-0000-ffffff000020"],
        "description": ["Builtin IDM Control for management of schema classes."],
        "acp_receiver": [],
        "acp_receiver_group": ["00000000-0000-0000-0000-000000000010"],
        "acp_targetscope": [
            "{\"and\": [{\"eq\": [\"class\",\"classtype\"]}, {\"andnot\": {\"or\": [{\"eq\": [\"class\", \"tombstone\"]}, {\"eq\": [\"class\", \"recycled\"]}]}}]}"
        ],
        "acp_search_attr": [
            "class",
            "description",
            "classname",
            "systemmay",
            "may",
            "systemmust",
            "must",
            "uuid"
        ],
        "acp_modify_removedattr": [
            "class",
            "description",
            "may",
            "must"
        ],
        "acp_modify_presentattr": [
            "class",
            "description",
            "may",
            "must"
        ],
        "acp_modify_class":  [],
        "acp_create_attr": [
            "class",
            "description",
            "classname",
            "may",
            "must",
            "uuid"
        ],
        "acp_create_class": [
            "object", "classtype"
        ]
    }
}"#;

// 21 - anonymous / everyone schema read.

// 22 - group create right
pub const JSON_IDM_ACP_GROUP_MANAGE_PRIV_V1: &str = r#"{
    "attrs": {
        "class": [
            "object",
            "access_control_profile",
            "access_control_delete",
            "access_control_create"
        ],
        "name": ["idm_acp_group_manage"],
        "uuid": ["00000000-0000-0000-0000-ffffff000022"],
        "description": ["Builtin IDM Control for creating and deleting groups in the directory"],
        "acp_receiver": [],
        "acp_receiver_group": ["00000000-0000-0000-0000-000000000015"],
        "acp_targetscope": [
            "{\"and\": [{\"eq\": [\"class\",\"group\"]}, {\"andnot\": {\"or\": [{\"eq\": [\"memberof\",\"00000000-0000-0000-0000-000000001000\"]}, {\"eq\": [\"class\", \"tombstone\"]}, {\"eq\": [\"class\", \"recycled\"]}]}}]}"
        ],
        "acp_create_attr": [
            "class",
            "name",
            "description",
            "member"
        ],
        "acp_create_class": [
            "object", "group"
        ]
    }
}"#;

// 23 - HP account manage
pub const JSON_IDM_ACP_HP_ACCOUNT_MANAGE_PRIV_V1: &str = r#"{
    "attrs": {
        "class": [
            "object",
            "access_control_profile",
            "access_control_delete",
            "access_control_create"
        ],
        "name": ["idm_acp_hp_account_manage"],
        "uuid": ["00000000-0000-0000-0000-ffffff000023"],
        "description": ["Builtin IDM Control for creating and deleting hp and regular (service) accounts"],
        "acp_receiver": [],
        "acp_receiver_group": ["00000000-0000-0000-0000-000000000016"],
        "acp_targetscope": [
            "{\"and\": [{\"eq\": [\"class\",\"account\"]}, {\"andnot\": {\"or\": [{\"eq\": [\"class\", \"tombstone\"]}, {\"eq\": [\"class\", \"recycled\"]}]}}]}"
        ],
        "acp_create_attr": [
            "class",
            "name",
            "displayname",
            "description",
            "primary_credential",
            "ssh_publickey",
            "account_expire",
            "account_valid_from",
            "passkeys",
            "devicekeys"
        ],
        "acp_create_class": [
            "object", "account", "service_account"
        ]
    }
}"#;

// 24 - hp group manage
pub const JSON_IDM_ACP_HP_GROUP_MANAGE_PRIV_V1: &str = r#"{
    "attrs": {
        "class": [
            "object",
            "access_control_profile",
            "access_control_delete",
            "access_control_create"
        ],
        "name": ["idm_acp_hp_group_manage"],
        "uuid": ["00000000-0000-0000-0000-ffffff000024"],
        "description": ["Builtin IDM Control for creating and deleting hp and regular groups in the directory"],
        "acp_receiver": [],
        "acp_receiver_group": ["00000000-0000-0000-0000-000000000017"],
        "acp_targetscope": [
            "{\"and\": [{\"eq\": [\"class\",\"group\"]}, {\"andnot\": {\"or\": [{\"eq\": [\"class\", \"tombstone\"]}, {\"eq\": [\"class\", \"recycled\"]}]}}]}"
        ],
        "acp_create_attr": [
            "class",
            "name",
            "description",
            "member"
        ],
        "acp_create_class": [
            "object", "group"
        ]
    }
}"#;

// 28 - domain admins acp
pub const JSON_IDM_ACP_DOMAIN_ADMIN_PRIV_V1: &str = r#"{
    "attrs": {
        "class": [
            "object",
            "access_control_profile",
            "access_control_search",
            "access_control_modify"
        ],
        "name": ["idm_acp_domain_admin_priv"],
        "uuid": ["00000000-0000-0000-0000-ffffff000026"],
        "description": ["Builtin IDM Control for granting domain info administration locally"],
        "acp_receiver": [],
        "acp_receiver_group": ["00000000-0000-0000-0000-000000000020"],
        "acp_targetscope": [
            "{\"and\": [{\"eq\": [\"uuid\",\"00000000-0000-0000-0000-ffffff000025\"]}, {\"andnot\": {\"or\": [{\"eq\": [\"class\", \"tombstone\"]}, {\"eq\": [\"class\", \"recycled\"]}]}}]}"
        ],
        "acp_search_attr": [
            "domain_display_name",
            "domain_name",
            "domain_ssid",
            "domain_uuid",
            "es256_private_key_der",
            "fernet_private_key_str",
            "name",
            "uuid"
        ],
        "acp_modify_removedattr": [
            "domain_display_name",
            "domain_ssid",
            "es256_private_key_der",
            "fernet_private_key_str"
        ],
        "acp_modify_presentattr": [
            "domain_display_name",
            "domain_ssid"
        ]
    }
}"#;

// 28 - system config
pub const JSON_IDM_ACP_SYSTEM_CONFIG_PRIV_V1: &str = r#"{
    "attrs": {
        "class": [
            "object",
            "access_control_profile",
            "access_control_search",
            "access_control_modify"
        ],
        "name": ["idm_acp_system_config_priv"],
        "uuid": ["00000000-0000-0000-0000-ffffff000028"],
        "description": ["Builtin IDM Control for granting system configuration rights"],
        "acp_receiver": [],
        "acp_receiver_group": ["00000000-0000-0000-0000-000000000019"],
        "acp_targetscope": [
            "{\"and\": [{\"eq\": [\"uuid\",\"00000000-0000-0000-0000-ffffff000027\"]}, {\"andnot\": {\"or\": [{\"eq\": [\"class\", \"tombstone\"]}, {\"eq\": [\"class\", \"recycled\"]}]}}]}"
        ],
        "acp_search_attr": [
            "name",
            "uuid",
            "description",
            "badlist_password"
        ],
        "acp_modify_removedattr": [
            "badlist_password"
        ],
        "acp_modify_presentattr": [
            "badlist_password"
        ]
    }
}"#;

// 29 account unix extend
pub const JSON_IDM_ACP_ACCOUNT_UNIX_EXTEND_PRIV_V1: &str = r#"{
    "attrs": {
        "class": [
            "object",
            "access_control_search",
            "access_control_profile",
            "access_control_modify"
        ],
        "name": ["idm_acp_account_unix_extend_priv"],
        "uuid": ["00000000-0000-0000-0000-ffffff000029"],
        "description": ["Builtin IDM Control for managing accounts."],
        "acp_receiver": [],
        "acp_receiver_group": ["00000000-0000-0000-0000-000000000021"],
        "acp_targetscope": [
            "{\"and\": [{\"eq\": [\"class\",\"account\"]}, {\"andnot\": {\"or\": [{\"eq\": [\"memberof\",\"00000000-0000-0000-0000-000000001000\"]}, {\"eq\": [\"class\", \"tombstone\"]}, {\"eq\": [\"class\", \"recycled\"]}]}}]}"
        ],
        "acp_search_attr": [
            "class", "name", "spn", "uuid", "description", "gidnumber", "loginshell", "unix_password"
        ],
        "acp_modify_removedattr": [
            "loginshell", "gidnumber", "unix_password"
        ],
        "acp_modify_presentattr": [
            "class", "loginshell", "gidnumber", "unix_password"
        ],
        "acp_modify_class": ["posixaccount"]
    }
}"#;
// 30 group unix extend
pub const JSON_IDM_ACP_GROUP_UNIX_EXTEND_PRIV_V1: &str = r#"{
    "attrs": {
        "class": [
            "object",
            "access_control_profile",
            "access_control_search",
            "access_control_modify"
        ],
        "name": ["idm_acp_group_unix_extend_priv"],
        "uuid": ["00000000-0000-0000-0000-ffffff000030"],
        "description": ["Builtin IDM Control for managing and extending unix groups"],
        "acp_receiver": [],
        "acp_receiver_group": ["00000000-0000-0000-0000-000000000022"],
        "acp_targetscope": [
            "{\"and\": [{\"eq\": [\"class\",\"group\"]}, {\"andnot\": {\"or\": [{\"eq\": [\"memberof\",\"00000000-0000-0000-0000-000000001000\"]}, {\"eq\": [\"class\", \"tombstone\"]}, {\"eq\": [\"class\", \"recycled\"]}]}}]}"
        ],
        "acp_search_attr": [
            "class", "name", "spn", "uuid", "description", "member", "gidnumber"
        ],
        "acp_modify_removedattr": [
            "gidnumber"
        ],
        "acp_modify_presentattr": [
            "class", "gidnumber"
        ],
        "acp_modify_class": ["posixgroup"]
    }
}"#;

// 33 hp account unix extend
pub const JSON_IDM_HP_ACP_ACCOUNT_UNIX_EXTEND_PRIV_V1: &str = r#"{
    "attrs": {
        "class": [
            "object",
            "access_control_search",
            "access_control_profile",
            "access_control_modify"
        ],
        "name": ["idm_acp_hp_account_unix_extend_priv"],
        "uuid": ["00000000-0000-0000-0000-ffffff000033"],
        "description": ["Builtin IDM Control for managing and extending unix high privilege accounts."],
        "acp_receiver": [],
        "acp_receiver_group": ["00000000-0000-0000-0000-000000000025"],
        "acp_targetscope": [
            "{\"and\": [{\"eq\": [\"class\",\"account\"]}, {\"eq\": [\"memberof\",\"00000000-0000-0000-0000-000000001000\"]}, {\"andnot\": {\"or\": [{\"eq\": [\"class\", \"tombstone\"]}, {\"eq\": [\"class\", \"recycled\"]}]}}]}"
        ],
        "acp_search_attr": [
            "class", "name", "spn", "uuid", "description", "gidnumber", "loginshell", "unix_password"
        ],
        "acp_modify_removedattr": [
            "loginshell", "gidnumber", "unix_password"
        ],
        "acp_modify_presentattr": [
            "class", "loginshell", "gidnumber", "unix_password"
        ],
        "acp_modify_class": ["posixaccount"]
    }
}"#;
// 34 hp group unix extend
pub const JSON_IDM_HP_ACP_GROUP_UNIX_EXTEND_PRIV_V1: &str = r#"{
    "attrs": {
        "class": [
            "object",
            "access_control_profile",
            "access_control_search",
            "access_control_modify"
        ],
        "name": ["idm_acp_hp_group_unix_extend_priv"],
        "uuid": ["00000000-0000-0000-0000-ffffff000034"],
        "description": ["Builtin IDM Control for managing and extending unix high privilege groups"],
        "acp_receiver": [],
        "acp_receiver_group": ["00000000-0000-0000-0000-000000000026"],
        "acp_targetscope": [
            "{\"and\": [{\"eq\": [\"class\",\"group\"]}, {\"eq\": [\"memberof\",\"00000000-0000-0000-0000-000000001000\"]}, {\"andnot\": {\"or\": [{\"eq\": [\"class\", \"tombstone\"]}, {\"eq\": [\"class\", \"recycled\"]}]}}]}"
        ],
        "acp_search_attr": [
            "class", "name", "spn", "uuid", "description", "member", "gidnumber"
        ],
        "acp_modify_removedattr": [
            "gidnumber"
        ],
        "acp_modify_presentattr": [
            "class", "gidnumber"
        ],
        "acp_modify_class": ["posixgroup"]
    }
}"#;

// 35 oauth2 manage
pub const JSON_IDM_HP_ACP_OAUTH2_MANAGE_PRIV_V1: &str = r#"{
    "attrs": {
        "class": [
            "object",
            "access_control_profile",
            "access_control_search",
            "access_control_modify",
            "access_control_delete",
            "access_control_create"
        ],
        "name": ["idm_acp_hp_oauth2_manage_priv"],
        "uuid": ["00000000-0000-0000-0000-ffffff000035"],
        "description": ["Builtin IDM Control for managing oauth2 resource server integrations."],
        "acp_receiver": [],
        "acp_receiver_group": ["00000000-0000-0000-0000-000000000027"],
        "acp_targetscope": [
            "{\"and\": [{\"eq\": [\"class\",\"oauth2_resource_server\"]},{\"andnot\": {\"or\": [{\"eq\": [\"class\", \"tombstone\"]}, {\"eq\": [\"class\", \"recycled\"]}]}}]}"
        ],
        "acp_search_attr": [
            "class",
            "description",
            "displayname",
            "oauth2_rs_name",
            "oauth2_rs_origin",
            "oauth2_rs_origin_landing",
            "oauth2_rs_scope_map",
            "oauth2_rs_sup_scope_map",
            "oauth2_rs_basic_secret",
            "oauth2_rs_token_key",
            "es256_private_key_der",
            "oauth2_allow_insecure_client_disable_pkce",
            "rs256_private_key_der",
            "oauth2_jwt_legacy_crypto_enable",
            "oauth2_prefer_short_username"
        ],
        "acp_modify_removedattr": [
            "description",
            "displayname",
            "oauth2_rs_name",
            "oauth2_rs_origin",
            "oauth2_rs_origin_landing",
            "oauth2_rs_scope_map",
            "oauth2_rs_sup_scope_map",
            "oauth2_rs_basic_secret",
            "oauth2_rs_token_key",
            "es256_private_key_der",
            "oauth2_allow_insecure_client_disable_pkce",
            "rs256_private_key_der",
            "oauth2_jwt_legacy_crypto_enable",
            "oauth2_prefer_short_username"
        ],
        "acp_modify_presentattr": [
            "description",
            "displayname",
            "oauth2_rs_name",
            "oauth2_rs_origin",
            "oauth2_rs_origin_landing",
            "oauth2_rs_sup_scope_map",
            "oauth2_rs_scope_map",
            "oauth2_allow_insecure_client_disable_pkce",
            "oauth2_jwt_legacy_crypto_enable",
            "oauth2_prefer_short_username"
        ],
        "acp_modify_class": [],
        "acp_create_attr": [
            "class",
            "description",
            "displayname",
            "oauth2_rs_name",
            "oauth2_rs_origin",
            "oauth2_rs_origin_landing",
            "oauth2_rs_sup_scope_map",
            "oauth2_rs_scope_map",
            "oauth2_allow_insecure_client_disable_pkce",
            "oauth2_jwt_legacy_crypto_enable",
            "oauth2_prefer_short_username"
        ],
        "acp_create_class": ["oauth2_resource_server", "oauth2_resource_server_basic", "object"]
    }
}"#;

pub const JSON_IDM_HP_ACP_SERVICE_ACCOUNT_INTO_PERSON_MIGRATE_V1: &str = r#"{
    "attrs": {
        "class": [
            "object",
            "access_control_search",
            "access_control_profile",
            "access_control_modify"
        ],
        "name": ["idm_hp_acp_service_account_into_person_migrate"],
        "uuid": ["00000000-0000-0000-0000-ffffff000042"],
        "description": ["Builtin IDM Control allowing service accounts to be migrated into persons"],
        "acp_receiver": [],
        "acp_receiver_group": ["00000000-0000-0000-0000-000000000034"],
        "acp_targetscope": [
            "{\"and\": [{\"eq\": [\"class\",\"account\"]}, {\"andnot\": {\"or\": [{\"eq\": [\"class\", \"tombstone\"]}, {\"eq\": [\"class\", \"recycled\"]}]}}]}"
        ],
        "acp_search_attr": [
            "class", "name", "uuid"
        ],
        "acp_modify_removedattr": [
            "class"
        ],
        "acp_modify_presentattr": [
            "class"
        ],
        "acp_modify_class": ["service_account", "person"]
    }
}"#;

pub const JSON_IDM_ACP_OAUTH2_READ_PRIV_V1: &str = r#"{
    "attrs": {
        "class": [
            "object",
            "access_control_profile",
            "access_control_search"
        ],
        "name": ["idm_acp_oauth2_read_priv"],
        "uuid": ["00000000-0000-0000-0000-ffffff000043"],
        "description": ["Builtin IDM Control allowing persons to view oauth2 applications they can access"],
        "acp_receiver": [],
        "acp_receiver_group": ["00000000-0000-0000-0000-000000000035"],
        "acp_targetscope": [
            "{\"and\": [{\"eq\": [\"class\",\"oauth2_resource_server\"]},{\"andnot\": {\"or\": [{\"eq\": [\"class\", \"tombstone\"]}, {\"eq\": [\"class\", \"recycled\"]}]}}]}"
        ],
        "acp_search_attr": [
            "class",
            "displayname",
            "oauth2_rs_name",
            "oauth2_rs_origin",
            "oauth2_rs_origin_landing"
        ]
    }
}"#;

pub const JSON_IDM_HP_ACP_SYNC_ACCOUNT_MANAGE_PRIV_V1: &str = r#"{
    "attrs": {
        "class": [
            "object",
            "access_control_profile",
            "access_control_search",
            "access_control_modify",
            "access_control_delete",
            "access_control_create"
        ],
        "name": ["idm_acp_hp_sync_account_manage_priv"],
        "uuid": ["00000000-0000-0000-0000-ffffff000044"],
        "description": ["Builtin IDM Control for managing IDM synchronisation accounts / connections"],
        "acp_receiver": [],
        "acp_receiver_group": ["00000000-0000-0000-0000-000000000037"],
        "acp_targetscope": [
            "{\"and\": [{\"eq\": [\"class\",\"sync_account\"]},{\"andnot\": {\"or\": [{\"eq\": [\"class\", \"tombstone\"]}, {\"eq\": [\"class\", \"recycled\"]}]}}]}"
        ],
        "acp_search_attr": [
            "class",
            "name",
            "description",
            "jws_es256_private_key",
            "sync_token_session",
            "sync_cookie"
        ],
        "acp_modify_removedattr": [
            "name",
            "description",
            "jws_es256_private_key",
            "sync_token_session",
            "sync_cookie"
        ],
        "acp_modify_presentattr": [
            "name",
            "description",
            "sync_token_session"
        ],
        "acp_modify_class": [],
        "acp_create_attr": [
            "class",
            "name",
            "description"
        ],
        "acp_create_class": ["sync_account", "object"]
    }
}"#;
