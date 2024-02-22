use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "profiles")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub chat_id: i64,
    pub username: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::statuses::Entity")]
    MonitoringStatuses,
}

impl Related<super::statuses::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MonitoringStatuses.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
