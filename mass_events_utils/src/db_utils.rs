use std::{future::Future, ptr};

use sqlx::{Pool, Postgres, Transaction};

pub async fn execute_transaction(sql: &str, pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    let sql_command = sql.split(';');
    let mut tx = pool.begin().await?;

    for command in sql_command {
        if let Err(err) = sqlx::query(command).execute(tx.as_mut()).await {
            tx.rollback().await?;
            return Err(err);
        }
    }

    tx.commit().await?;

    Ok(())
}

#[derive(Clone, Copy)]
pub struct TransactionObject {
    transaction: *mut Transaction<'static, Postgres>,
}

unsafe impl Send for TransactionObject {}

impl TransactionObject {
    pub fn new(transaction: *mut Transaction<'static, Postgres>) -> Self { Self { transaction } }

    // pub async fn commit(self) -> Result<(), sqlx::Error> {
    //     Ok(unsafe { ptr::read(self.transaction).commit() }.await?)
    // }
    // pub async fn rollback(self) -> Result<(), sqlx::Error> {
    //     Ok(unsafe { ptr::read(self.transaction).rollback() }.await?)
    // }
    pub fn get_connection_ref(&self) -> &mut Transaction<'static, Postgres> {
        unsafe { &mut *self.transaction }
    }
}

pub async fn in_transaction<F, Fut, R>(pool: &Pool<Postgres>, closure: F) -> Result<R, sqlx::Error>
where
    F: FnOnce(TransactionObject) -> Fut,
    Fut: Future<Output = Result<R, sqlx::Error>>,
{
    let mut tx = pool.begin().await?;
    let tx_ptr = ptr::addr_of_mut!(tx);
    let txo = TransactionObject::new(tx_ptr);
    let rtn = closure(txo).await;

    if rtn.is_ok() {
        tx.commit().await?;
    } else {
        tx.rollback().await?;
    }
    rtn
}
