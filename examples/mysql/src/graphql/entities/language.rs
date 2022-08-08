use sea_orm::prelude::*;
pub fn filter_recursive(root_filter: Option<Filter>) -> sea_orm::Condition {
    let mut condition = sea_orm::Condition::all();
    if let Some(current_filter) = root_filter {
        if let Some(or_filters) = current_filter.or {
            let or_condition = or_filters
                .into_iter()
                .fold(sea_orm::Condition::any(), |fold_condition, filter| {
                    fold_condition.add(filter_recursive(Some(*filter)))
                });
            condition = condition.add(or_condition);
        }
        if let Some(and_filters) = current_filter.and {
            let and_condition = and_filters
                .into_iter()
                .fold(sea_orm::Condition::all(), |fold_condition, filter| {
                    fold_condition.add(filter_recursive(Some(*filter)))
                });
            condition = condition.add(and_condition);
        }
        if let Some(language_id) = current_filter.language_id {
            if let Some(eq_value) = language_id.eq {
                condition = condition.add(Column::LanguageId.eq(eq_value))
            }
            if let Some(ne_value) = language_id.ne {
                condition = condition.add(Column::LanguageId.ne(ne_value))
            }
            if let Some(gt_value) = language_id.gt {
                condition = condition.add(Column::LanguageId.gt(gt_value))
            }
            if let Some(gte_value) = language_id.gte {
                condition = condition.add(Column::LanguageId.gte(gte_value))
            }
            if let Some(lt_value) = language_id.lt {
                condition = condition.add(Column::LanguageId.lt(lt_value))
            }
            if let Some(lte_value) = language_id.lte {
                condition = condition.add(Column::LanguageId.lte(lte_value))
            }
            if let Some(is_in_value) = language_id.is_in {
                condition = condition.add(Column::LanguageId.is_in(is_in_value))
            }
            if let Some(is_not_in_value) = language_id.is_not_in {
                condition = condition.add(Column::LanguageId.is_not_in(is_not_in_value))
            }
            if let Some(is_null_value) = language_id.is_null {
                if is_null_value {
                    condition = condition.add(Column::LanguageId.is_null())
                }
            }
        }
        if let Some(name) = current_filter.name {
            if let Some(eq_value) = name.eq {
                condition = condition.add(Column::Name.eq(eq_value))
            }
            if let Some(ne_value) = name.ne {
                condition = condition.add(Column::Name.ne(ne_value))
            }
            if let Some(gt_value) = name.gt {
                condition = condition.add(Column::Name.gt(gt_value))
            }
            if let Some(gte_value) = name.gte {
                condition = condition.add(Column::Name.gte(gte_value))
            }
            if let Some(lt_value) = name.lt {
                condition = condition.add(Column::Name.lt(lt_value))
            }
            if let Some(lte_value) = name.lte {
                condition = condition.add(Column::Name.lte(lte_value))
            }
            if let Some(is_in_value) = name.is_in {
                condition = condition.add(Column::Name.is_in(is_in_value))
            }
            if let Some(is_not_in_value) = name.is_not_in {
                condition = condition.add(Column::Name.is_not_in(is_not_in_value))
            }
            if let Some(is_null_value) = name.is_null {
                if is_null_value {
                    condition = condition.add(Column::Name.is_null())
                }
            }
        }
        if let Some(last_update) = current_filter.last_update {
            if let Some(eq_value) = last_update.eq {
                condition = condition.add(Column::LastUpdate.eq(eq_value))
            }
            if let Some(ne_value) = last_update.ne {
                condition = condition.add(Column::LastUpdate.ne(ne_value))
            }
            if let Some(gt_value) = last_update.gt {
                condition = condition.add(Column::LastUpdate.gt(gt_value))
            }
            if let Some(gte_value) = last_update.gte {
                condition = condition.add(Column::LastUpdate.gte(gte_value))
            }
            if let Some(lt_value) = last_update.lt {
                condition = condition.add(Column::LastUpdate.lt(lt_value))
            }
            if let Some(lte_value) = last_update.lte {
                condition = condition.add(Column::LastUpdate.lte(lte_value))
            }
            if let Some(is_in_value) = last_update.is_in {
                condition = condition.add(Column::LastUpdate.is_in(is_in_value))
            }
            if let Some(is_not_in_value) = last_update.is_not_in {
                condition = condition.add(Column::LastUpdate.is_not_in(is_not_in_value))
            }
            if let Some(is_null_value) = last_update.is_null {
                if is_null_value {
                    condition = condition.add(Column::LastUpdate.is_null())
                }
            }
        }
    }
    condition
}
use crate::graphql::*;
pub use crate::orm::language::*;
#[async_graphql::Object(name = "Language")]
impl Model {
    pub async fn language_id(&self) -> &u8 {
        &self.language_id
    }
    pub async fn name(&self) -> &String {
        &self.name
    }
    pub async fn last_update(&self) -> &DateTimeUtc {
        &self.last_update
    }
    pub async fn language_language_film<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> Vec<crate::orm::film::Model> {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = LanguageFilmFK(self.language_id.clone().try_into().unwrap());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap_or(vec![])
    }
}
#[derive(async_graphql :: InputObject, Debug)]
#[graphql(name = "LanguageFilter")]
pub struct Filter {
    pub or: Option<Vec<Box<Filter>>>,
    pub and: Option<Vec<Box<Filter>>>,
    pub language_id: Option<TypeFilter<u8>>,
    pub name: Option<TypeFilter<String>>,
    pub last_update: Option<TypeFilter<DateTimeUtc>>,
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct LanguageFilmFK(u8);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<LanguageFilmFK> for OrmDataloader {
    type Value = Vec<crate::orm::film::Model>;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[LanguageFilmFK],
    ) -> Result<std::collections::HashMap<LanguageFilmFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(crate::orm::film::Column::LanguageId.as_column_ref())
                    .into_simple_expr(),
            ])),
            sea_orm::sea_query::BinOper::In,
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(
                keys.iter()
                    .map(|tuple| {
                        sea_orm::sea_query::SimpleExpr::Values(vec![tuple.0.clone().into()])
                    })
                    .collect(),
            )),
        ));
        use itertools::Itertools;
        Ok(crate::orm::film::Entity::find()
            .filter(filter)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|model| {
                let key = LanguageFilmFK(model.language_id.clone().try_into().unwrap());
                (key, model)
            })
            .into_group_map())
    }
}