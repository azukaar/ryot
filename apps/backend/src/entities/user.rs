//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.2

use async_graphql::SimpleObject;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::migrator::UserLot;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, SimpleObject)]
#[graphql(name = "User")]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    #[graphql(skip)]
    pub password: String,
    pub lot: UserLot,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::seen::Entity")]
    Seen,
    #[sea_orm(has_many = "super::summary::Entity")]
    Summary,
    #[sea_orm(has_many = "super::token::Entity")]
    Token,
}

impl Related<super::seen::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Seen.def()
    }
}

impl Related<super::summary::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Summary.def()
    }
}

impl Related<super::token::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Token.def()
    }
}

impl Related<super::metadata::Entity> for Entity {
    fn to() -> RelationDef {
        super::user_to_metadata::Relation::Metadata.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::user_to_metadata::Relation::User.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
