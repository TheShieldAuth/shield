use crate::models::client;
use crate::models::realm;
use crate::utils::check_locked_at_constraint;
use async_trait::async_trait;
use sea_orm::QuerySelect;
use sea_orm::{entity::prelude::*, ActiveValue};

#[async_trait]
impl ActiveModelBehavior for client::ActiveModel {
    /// Will be triggered before insert / update
    async fn before_save<C>(mut self, db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if let ActiveValue::Set(ref locked_at) = self.locked_at {
            check_locked_at_constraint(locked_at)?
        }

        // Fetch the realm information based on the realm_id in the client
        let realm_id = self.realm_id.clone().unwrap(); // Assume realm_id is set

        let realm = realm::Entity::find_by_id(realm_id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Realm not found".to_owned()))?;

        // Check max_concurrent_sessions
        if let Some(realm_max_sessions) = realm.max_concurrent_sessions {
            // Calculate the total max_concurrent_sessions for all clients in this realm (excluding the current client if it's an update)
            let mut total_sessions = client::Entity::find()
                .filter(client::Column::RealmId.eq(realm_id))
                .filter(client::Column::Id.ne(self.id.clone().unwrap())) // Exclude current client during update
                .select_only()
                .column_as(client::Column::MaxConcurrentSessions.sum(), "total")
                .into_tuple::<i32>()
                .one(db)
                .await?
                .unwrap_or(0);

            // Add the new/updated client's max_concurrent_sessions to the total
            if let Some(client_max_sessions) = self.max_concurrent_sessions.as_ref() {
                total_sessions += client_max_sessions;
            }

            // Check if total exceeds the realm's max_concurrent_sessions
            if total_sessions > realm_max_sessions {
                return Err(DbErr::Custom(format!(
                    "Total max_concurrent_sessions for all clients in this realm ({}) exceeds the realm's limit ({})",
                    total_sessions, realm_max_sessions
                )));
            }
        }

        // Check session_lifetime
        match self.session_lifetime {
            ActiveValue::Set(session_lifetime) => {
                if session_lifetime > realm.session_lifetime {
                    return Err(DbErr::Custom(format!(
                        "Client session_lifetime ({}) exceeds the realm's limit ({})",
                        self.session_lifetime.as_ref(),
                        &realm.session_lifetime
                    )));
                }
            }
            _ => {}
        }

        // // Check refresh_token_lifetime
        match self.refresh_token_lifetime {
            ActiveValue::Set(refresh_token_lifetime) => {
                if refresh_token_lifetime > realm.refresh_token_lifetime {
                    return Err(DbErr::Custom(format!(
                        "Client refresh_token_lifetime ({}) exceeds the realm's limit ({})",
                        self.refresh_token_lifetime.as_ref(),
                        &realm.refresh_token_lifetime
                    )));
                }
            }
            _ => {}
        }

        // // Check refresh_token_reuse_limit
        match self.refresh_token_reuse_limit {
            ActiveValue::Set(refresh_token_reuse_limit) => {
                if refresh_token_reuse_limit > realm.refresh_token_reuse_limit {
                    return Err(DbErr::Custom(format!(
                        "Client refresh_token_reuse_limit ({}) exceeds the realm's limit ({})",
                        self.refresh_token_reuse_limit.as_ref(),
                        &realm.refresh_token_reuse_limit
                    )));
                }
            }
            _ => {}
        }

        Ok(self)
    }
}
