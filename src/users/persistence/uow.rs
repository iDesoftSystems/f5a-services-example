use crate::users::persistence::repository::UserRepository;
use sea_orm::{DatabaseConnection, DatabaseTransaction, DbErr, TransactionTrait};
use std::sync::Arc;

pub trait UnitOfWork: Send + Sync {
    fn user_repository(&self) -> UserRepository<'_, DatabaseTransaction>;

    fn commit(self) -> impl Future<Output = Result<(), DbErr>>;

    fn rollback(self) -> impl Future<Output = Result<(), DbErr>>;
}

pub struct UnitOfWorkImpl {
    tx: DatabaseTransaction,
}

impl UnitOfWork for UnitOfWorkImpl {
    fn user_repository(&self) -> UserRepository<'_, DatabaseTransaction> {
        UserRepository::new(&self.tx)
    }

    async fn commit(self) -> Result<(), DbErr> {
        self.tx.commit().await
    }

    async fn rollback(self) -> Result<(), DbErr> {
        self.tx.rollback().await
    }
}

pub struct UnitOfWorkFactory {
    conn: Arc<DatabaseConnection>,
}

impl UnitOfWorkFactory {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }

    pub async fn begin(&self) -> Result<UnitOfWorkImpl, DbErr> {
        let tx = self.conn.begin().await?;
        Ok(UnitOfWorkImpl { tx })
    }
}
