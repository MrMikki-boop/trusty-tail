use sea_orm::prelude::*;
use sea_orm::{sea_query::OnConflict, ActiveValue, ColumnTrait};
use std::error::Error;
use teloxide::prelude::*;

use super::monitoring_statuses;

pub async fn is_enabled(connection: &DatabaseConnection, chat_id: ChatId) -> bool {
    monitoring_statuses::Entity::find()
        .filter(monitoring_statuses::Column::ChatId.eq(chat_id.0))
        .one(connection)
        .await
        .ok()
        .flatten()
        .map_or(false, |x| x.enabled)
}

pub async fn set_monitoring(
    connection: &DatabaseConnection,
    chat_id: ChatId,
    status: bool,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    monitoring_statuses::Entity::insert(monitoring_statuses::ActiveModel {
        chat_id: ActiveValue::Set(chat_id.0),
        enabled: ActiveValue::Set(status),
        ..Default::default()
    })
    .on_conflict(
        OnConflict::column(monitoring_statuses::Column::ChatId)
            .update_column(monitoring_statuses::Column::Enabled)
            .to_owned(),
    )
    .exec(connection)
    .await?;

    Ok(())
}
