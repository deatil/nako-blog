use sea_orm::*;

use crate::app::entity::{
    friendlink, 
    friendlink::Entity as Friendlink,
};

/// 条件
#[derive(Clone)]
pub struct FriendlinkWhere {
    pub title: Option<String>,
    pub url: Option<String>,
    pub target: Option<String>,
    pub status: Option<i32>,
}

impl FriendlinkWhere {
    /// 格式化
    pub fn format(&self) -> Self {
        let mut title = None;
        if self.title != Some("".to_string()) {
            title = self.title.clone();
        }
    
        let mut url = None;
        if self.url != Some("".to_string()) {
            url = self.url.clone();
        }
       
        let mut target = None;
        if self.target != Some("".to_string()) {
            target = self.target.clone();
        }

        let mut status = None;
        if self.status == Some(1) || self.status == Some(0) {
            status = self.status;
        }
    
        Self {
            title: title,
            url: url,
            target: target,
            status: status,
        }
    }
}

pub struct FriendlinkModel;

impl FriendlinkModel {
    pub async fn find_by_id(db: &DbConn, id: u32) -> Result<Option<friendlink::Model>, DbErr> {
        Friendlink::find_by_id(id).one(db).await
    }

    pub async fn find_count(db: &DbConn) -> Result<u64, DbErr> {
        Friendlink::find().count(db).await
    }

    pub async fn find_in_page(
        db: &DbConn,
        page: u64,
        per_page: u64,
    ) -> Result<(Vec<friendlink::Model>, u64), DbErr> {
        let paginator = Friendlink::find()
            .order_by_desc(friendlink::Column::Sort)
            .paginate(db, per_page);
        let num_pages = paginator.num_pages().await?;

        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }

    // 搜索
    pub async fn search_count(
        db: &DbConn,
        wheres: FriendlinkWhere,
    ) -> Result<u64, DbErr> {
        Friendlink::find()
            .apply_if(wheres.title, |query, v| {
                query.filter(friendlink::Column::Title.contains(format!("%{}%", v).as_str()))
            })
            .apply_if(wheres.url, |query, v| {
                query.filter(friendlink::Column::Url.contains(format!("%{}%", v).as_str()))
            })
            .apply_if(wheres.target, |query, v| {
                query.filter(friendlink::Column::Target.eq(v))
            })
            .apply_if(wheres.status, |query, v| {
                query.filter(friendlink::Column::Status.eq(v))
            })
            .count(db)
            .await
    }

    pub async fn search_in_page(
        db: &DbConn,
        page: u64,
        per_page: u64,
        wheres: FriendlinkWhere,
    ) -> Result<(Vec<friendlink::Model>, u64), DbErr> {
        let paginator = Friendlink::find()
            .apply_if(wheres.title, |query, v| {
                query.filter(friendlink::Column::Title.contains(format!("%{}%", v).as_str()))
            })
            .apply_if(wheres.url, |query, v| {
                query.filter(friendlink::Column::Url.contains(format!("%{}%", v).as_str()))
            })
            .apply_if(wheres.target, |query, v| {
                query.filter(friendlink::Column::Target.eq(v))
            })
            .apply_if(wheres.status, |query, v| {
                query.filter(friendlink::Column::Status.eq(v))
            })
            .order_by_desc(friendlink::Column::Sort)
            .paginate(db, per_page);
        let num_pages = paginator.num_pages().await?;

        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }

    pub async fn list_open(
        db: &DbConn,
    ) -> Result<Vec<friendlink::Model>, DbErr> {
        Friendlink::find()
            .filter(friendlink::Column::Status.eq(1))
            .order_by_desc(friendlink::Column::Sort)
            .all(db)
            .await
    }

    pub async fn create(
        db: &DbConn,
        form_data: friendlink::Model,
    ) -> Result<friendlink::ActiveModel, DbErr> {
        friendlink::ActiveModel {
                title:    Set(form_data.title.to_owned()),
                url:      Set(form_data.url.to_owned()),
                target:   Set(form_data.target.to_owned()),
                icon:     Set(form_data.icon.to_owned()),
                sort:     Set(form_data.sort.to_owned()),
                status:   Set(form_data.status.to_owned()),
                add_time: Set(form_data.add_time.to_owned()),
                add_ip:   Set(form_data.add_ip.to_owned()),
                ..Default::default()
            }
            .save(db)
            .await
    }

    pub async fn update_by_id(
        db: &DbConn,
        id: u32,
        form_data: friendlink::Model,
    ) -> Result<friendlink::Model, DbErr> {
        let friendlink: friendlink::ActiveModel = Friendlink::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find friendlink.".to_owned()))
            .map(Into::into)?;

        friendlink::ActiveModel {
                id:     friendlink.id,
                title:  Set(form_data.title.to_owned()),
                url:    Set(form_data.url.to_owned()),
                target: Set(form_data.target.to_owned()),
                icon:   Set(form_data.icon.to_owned()),
                sort:   Set(form_data.sort.to_owned()),
                status: Set(form_data.status.to_owned()),
                ..Default::default()
            }
            .update(db)
            .await
    }
    
    pub async fn update_status_by_id(
        db: &DbConn,
        id: u32,
        form_data: friendlink::Model,
    ) -> Result<friendlink::Model, DbErr> {
        let friendlink: friendlink::ActiveModel = Friendlink::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find friendlink.".to_owned()))
            .map(Into::into)?;

        friendlink::ActiveModel {
                id: friendlink.id,
                status: Set(form_data.status.to_owned()),
                ..Default::default()
            }
            .update(db)
            .await
    }

    pub async fn delete(db: &DbConn, id: u32) -> Result<DeleteResult, DbErr> {
        let friendlink: friendlink::ActiveModel = Friendlink::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find friendlink.".to_owned()))
            .map(Into::into)?;

        friendlink.delete(db).await
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, DbErr> {
        Friendlink::delete_many().exec(db).await
    }

}
