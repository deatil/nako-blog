use sea_orm::*;

use crate::app::entity::{user, user::Entity as User};

pub struct UserModel;

impl UserModel {
    pub async fn find_user_by_id(db: &DbConn, id: u32) -> Result<Option<user::Model>, DbErr> {
        User::find_by_id(id).one(db).await
    }

    pub async fn find_user_by_name(db: &DbConn, name: &str) -> Result<Option<user::Model>, DbErr> {
        User::find()
            .filter(user::Column::Username.contains(name))
            .one(db)
            .await
    }

    pub async fn find_users_count(db: &DbConn) -> Result<u64, DbErr> {
        User::find().count(db).await
    }

    pub async fn find_users_count_by_name(db: &DbConn, name: &str) -> Result<u64, DbErr> {
        User::find()
            .filter(
                Condition::any()
                    .add(user::Column::Username.like(name))
                    .add(user::Column::Nickname.like(name))
            )
            .count(db)
            .await
    }

    /// If ok, returns (user models, num pages).
    pub async fn find_users_in_page(
        db: &DbConn,
        page: u64,
        per_page: u64,
    ) -> Result<(Vec<user::Model>, u64), DbErr> {
        // Setup paginator
        let paginator = User::find()
            .order_by_asc(user::Column::Id)
            .paginate(db, per_page);
        let num_pages = paginator.num_pages().await?;

        // Fetch paginated users
        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }

    pub async fn find_users_in_page_by_name(
        db: &DbConn,
        page: u64,
        per_page: u64,
        name: &str,
    ) -> Result<(Vec<user::Model>, u64), DbErr> {
        // Setup paginator
        let paginator = User::find()
            .filter(
                Condition::any()
                    .add(user::Column::Username.like(name))
                    .add(user::Column::Nickname.like(name))
            )
            .order_by_asc(user::Column::Id)
            .paginate(db, per_page);
        let num_pages = paginator.num_pages().await?;

        // Fetch paginated users
        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }

    pub async fn create_user(
        db: &DbConn,
        form_data: user::Model,
    ) -> Result<user::ActiveModel, DbErr> {
        user::ActiveModel {
                username: Set(form_data.username.to_owned()),
                nickname: Set(form_data.nickname.to_owned()),
                sign:     Set(form_data.sign.to_owned()),
                status:   Set(form_data.status.to_owned()),
                add_time: Set(form_data.add_time.to_owned()),
                add_ip:   Set(form_data.add_ip.to_owned()),
                ..Default::default()
            }
            .save(db)
            .await
    }

    pub async fn update_user_by_id(
        db: &DbConn,
        id: u32,
        form_data: user::Model,
    ) -> Result<user::Model, DbErr> {
        let user: user::ActiveModel = User::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find user.".to_owned()))
            .map(Into::into)?;

        user::ActiveModel {
                id: user.id,
                username: Set(form_data.username.to_owned()),
                nickname: Set(form_data.nickname.to_owned()),
                sign: Set(form_data.sign.to_owned()),
                status: Set(form_data.status.to_owned()),
                ..Default::default()
            }
            .update(db)
            .await
    }

    pub async fn update_status_by_id(
        db: &DbConn,
        id: u32,
        form_data: user::Model,
    ) -> Result<user::Model, DbErr> {
        let user: user::ActiveModel = User::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find user.".to_owned()))
            .map(Into::into)?;

        user::ActiveModel {
                id: user.id,
                status: Set(form_data.status.to_owned()),
                ..Default::default()
            }
            .update(db)
            .await
    }

    pub async fn update_password_by_id(
        db: &DbConn,
        id: u32,
        form_data: user::Model,
    ) -> Result<user::Model, DbErr> {
        let user: user::ActiveModel = User::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find user.".to_owned()))
            .map(Into::into)?;

        user::ActiveModel {
                id: user.id,
                password: Set(form_data.password.to_owned()),
                ..Default::default()
            }
            .update(db)
            .await
    }

    pub async fn update_avatar_by_id(
        db: &DbConn,
        id: u32,
        form_data: user::Model,
    ) -> Result<user::Model, DbErr> {
        let user: user::ActiveModel = User::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find user.".to_owned()))
            .map(Into::into)?;

        user::ActiveModel {
                id: user.id,
                avatar: Set(form_data.avatar.to_owned()),
                ..Default::default()
            }
            .update(db)
            .await
    }

    pub async fn delete_user(db: &DbConn, id: u32) -> Result<DeleteResult, DbErr> {
        let user: user::ActiveModel = User::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::Custom("Cannot find user.".to_owned()))
            .map(Into::into)?;

        user.delete(db).await
    }

    pub async fn delete_all_users(db: &DbConn) -> Result<DeleteResult, DbErr> {
        User::delete_many().exec(db).await
    }

}
