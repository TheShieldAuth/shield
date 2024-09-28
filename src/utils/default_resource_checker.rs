use sea_orm::prelude::Uuid;

use crate::packages::settings::SETTINGS;

pub fn is_default_realm(realm_id: Uuid) -> bool {
    realm_id == SETTINGS.read().default_cred.realm_id
}

pub fn is_default_client(client_id: Uuid) -> bool {
    client_id == SETTINGS.read().default_cred.client_id
}
