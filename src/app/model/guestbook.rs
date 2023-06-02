use sea_orm::*;

use crate::app::entity::{
    guestbook, 
    guestbook::Entity as Guestbook,
};

/// 条件
#[derive(Clone)]
pub struct GuestbookWhere {
    pub name: Option<String>,
    pub message: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub qq: Option<String>,
    pub weixin: Option<String>,
    pub status: Option<i32>,
}

impl GuestbookWhere {
    /// 格式化
    pub fn format(&self) -> Self {
        let mut name = None;
        if self.name != Some("".to_string()) {
            name = self.name.clone();
        }
    
        let mut message = None;
        if self.message != Some("".to_string()) {
            message = self.message.clone();
        }
       
        let mut phone = None;
        if self.phone != Some("".to_string()) {
            phone = self.phone.clone();
        }
            
        let mut email = None;
        if self.email != Some("".to_string()) {
            email = self.email.clone();
        }
            
        let mut qq = None;
        if self.qq != Some("".to_string()) {
            qq = self.qq.clone();
        }
            
        let mut weixin = None;
        if self.weixin != Some("".to_string()) {
            weixin = self.weixin.clone();
        }

        let mut status = None;
        if self.status == Some(1) || self.status == Some(0) {
            status = self.status;
        }
    
        Self {
            name: name,
            message: message,
            phone: phone,
            email: email,
            qq: qq,
            weixin: weixin,
            status: status,
        }
    }
}

pub struct GuestbookModel;

impl GuestbookModel {
    pub async fn find_by_id(db: &DbConn, id: u32) -> Result<Option<guestbook::Model>, DbErr> {
        Guestbook::find_by_id(id).one(db).await
    }

    pub async fn find_count(db: &DbConn) -> Result<u64, DbErr> {
        Guestbook::find().count(db).await
    }

    pub async fn find_in_page(
        db: &DbConn,
        page: u64,
        per_page: u64,
    ) -> Result<(Vec<guestbook::Model>, u64), DbErr> {
        let paginator = Guestbook::find()
            .order_by_desc(guestbook::Column::AddTime)
            .paginate(db, per_page);
        let num_pages = paginator.num_pages().await?;

        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }

    // 搜索
    pub async fn search_count(
        db: &DbConn,
        wheres: GuestbookWhere,
    ) -> Result<u64, DbErr> {
        Guestbook::find()
            .apply_if(wheres.name, |query, v| {
                query.filter(guestbook::Column::Name.contains(format!("%{}%", v).as_str()))
            })
            .apply_if(wheres.message, |query, v| {
                query.filter(guestbook::Column::Message.contains(format!("%{}%", v).as_str()))
            })
            .apply_if(wheres.phone, |query, v| {
                query.filter(guestbook::Column::Phone.contains(format!("%{}%", v).as_str()))
            })
            .apply_if(wheres.email, |query, v| {
                query.filter(guestbook::Column::Email.contains(format!("%{}%", v).as_str()))
            })
            .apply_if(wheres.qq, |query, v| {
                query.filter(guestbook::Column::Qq.contains(format!("%{}%", v).as_str()))
            })
            .apply_if(wheres.weixin, |query, v| {
                query.filter(guestbook::Column::Weixin.contains(format!("%{}%", v).as_str()))
            })
            .apply_if(wheres.status, |query, v| {
                query.filter(guestbook::Column::Status.eq(v))
            })
            .count(db)
            .await
    }

    pub async fn search_in_page(
        db: &DbConn,
        page: u64,
        per_page: u64,
        wheres: GuestbookWhere,
    ) -> Result<(Vec<guestbook::Model>, u64), DbErr> {
        let paginator = Guestbook::find()
            .apply_if(wheres.name, |query, v| {
                query.filter(guestbook::Column::Name.contains(format!("%{}%", v).as_str()))
            })
            .apply_if(wheres.message, |query, v| {
                query.filter(guestbook::Column::Message.contains(format!("%{}%", v).as_str()))
            })
            .apply_if(wheres.phone, |query, v| {
                query.filter(guestbook::Column::Phone.contains(format!("%{}%", v).as_str()))
            })
            .apply_if(wheres.email, |query, v| {
                query.filter(guestbook::Column::Email.contains(format!("%{}%", v).as_str()))
            })
            .apply_if(wheres.qq, |query, v| {
                query.filter(guestbook::Column::Qq.contains(format!("%{}%", v).as_str()))
            })
            .apply_if(wheres.weixin, |query, v| {
                query.filter(guestbook::Column::Weixin.contains(format!("%{}%", v).as_str()))
            })
            .apply_if(wheres.status, |query, v| {
                query.filter(guestbook::Column::Status.eq(v))
            })
            .order_by_desc(guestbook::Column::AddTime)
            .paginate(db, per_page);
        let num_pages = paginator.num_pages().await?;

        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }

    pub async fn create(
        db: &DbConn,
        form_data: guestbook::Model,
    ) -> Result<guestbook::ActiveModel, DbErr> {
        guestbook::ActiveModel {
                name:     Set(form_data.name.to_owned()),
                message:  Set(form_data.message.to_owned()),
                phone:    Set(form_data.phone.to_owned()),
                email:    Set(form_data.email.to_owned()),
                qq:       Set(form_data.qq.to_owned()),
                weixin:   Set(form_data.weixin.to_owned()),
                status:   Set(form_data.status.to_owned()),
                add_time: Set(form_data.add_time.to_owned()),
                add_ip:   Set(form_data.add_ip.to_owned()),
                ..Default::default()
            }
            .save(db)
            .await
    }

    pub async fn update_status_by_id(
        db: &DbConn,
        id: u32,
        form_data: guestbook::Model,
    ) -> Result<guestbook::Model, DbErr> {
        let guestbook: guestbook::ActiveModel = Guestbook::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find guestbook.".to_owned()))
            .map(Into::into)?;

        guestbook::ActiveModel {
                id: guestbook.id,
                status: Set(form_data.status.to_owned()),
                ..Default::default()
            }
            .update(db)
            .await
    }

    pub async fn delete(db: &DbConn, id: u32) -> Result<DeleteResult, DbErr> {
        let guestbook: guestbook::ActiveModel = Guestbook::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find guestbook.".to_owned()))
            .map(Into::into)?;

        guestbook.delete(db).await
    }

    pub async fn delete_all(db: &DbConn) -> Result<DeleteResult, DbErr> {
        Guestbook::delete_many().exec(db).await
    }

}
