use sqlx::{Database, Decode, Encode, Type, encode::IsNull, error::BoxDynError};

use crate::models::PluralKitId;

impl<DB, T> Type<DB> for PluralKitId<T>
where
    DB: Database,
    for<'a> &'a str: Type<DB>,
{
    fn type_info() -> <DB as Database>::TypeInfo {
        <&str as Type<DB>>::type_info()
    }

    fn compatible(ty: &<DB as Database>::TypeInfo) -> bool {
        <&str as Type<DB>>::compatible(ty)
    }
}

impl<'q, DB, T> Encode<'q, DB> for PluralKitId<T>
where
    DB: Database,
    String: Encode<'q, DB>,
{
    fn encode_by_ref(
        &self,
        buf: &mut <DB as Database>::ArgumentBuffer<'q>,
    ) -> Result<IsNull, BoxDynError> {
        <&String as Encode<'q, DB>>::encode(&self.value, buf)
    }
}

impl<'r, DB, T> Decode<'r, DB> for PluralKitId<T>
where
    DB: Database,
    &'r str: Decode<'r, DB>,
{
    fn decode(value: <DB as Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
        <&str as Decode<DB>>::decode(value).map(|val| Self::new(val.to_string()))
    }
}

#[cfg(test)]
mod test {
    use crate::models::{PluralKitId, marker::SystemMarker};

    #[tokio::test]
    #[ignore]
    async fn it_decodes_for_postgres() -> Result<(), Box<dyn std::error::Error>> {
        let expected = PluralKitId::<SystemMarker>::try_from("exmpl")?;
        let conn = sqlx::PgPool::connect(&std::env::var("POSTGRES_URL")?).await?;

        let result: PluralKitId<SystemMarker> =
            sqlx::query_scalar(r#"SELECT 'exmpl' AS "id: PluralKitId<SystemMarker>""#)
                .fetch_one(&conn)
                .await?;

        assert_eq!(result, expected);

        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn it_decodes_for_mysql() -> Result<(), Box<dyn std::error::Error>> {
        let expected = PluralKitId::<SystemMarker>::try_from("exmpl")?;
        let conn = sqlx::MySqlPool::connect(&std::env::var("MYSQL_URL")?).await?;

        let result: PluralKitId<SystemMarker> =
            sqlx::query_scalar(r#"SELECT "exmpl" AS 'id: PluralKitId<SystemMarker>'"#)
                .fetch_one(&conn)
                .await?;

        assert_eq!(result, expected);

        Ok(())
    }

    #[tokio::test]
    async fn it_decodes_for_sqlite() -> Result<(), Box<dyn std::error::Error>> {
        let expected = PluralKitId::<SystemMarker>::try_from("exmpl")?;
        let conn = sqlx::SqlitePool::connect(":memory:").await?;

        let result: PluralKitId<SystemMarker> =
            sqlx::query_scalar(r#"SELECT "exmpl" AS "id: PluralKitId<SystemMarker>""#)
                .fetch_one(&conn)
                .await?;

        assert_eq!(result, expected);

        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn it_encodes_for_postgres() -> Result<(), Box<dyn std::error::Error>> {
        let input = PluralKitId::<SystemMarker>::try_from("exmpl")?;
        let conn = sqlx::PgPool::connect(&std::env::var("POSTGRES_URL")?).await?;

        let mut tx = conn.begin().await?;
        sqlx::query(r#"CREATE TEMPORARY TABLE pluralkit_id (id TEXT)"#)
            .execute(&mut *tx)
            .await?;
        sqlx::query(r#"INSERT INTO pluralkit_id (id) VALUES ($1)"#)
            .bind(&input)
            .execute(&mut *tx)
            .await?;

        let result: PluralKitId<SystemMarker> =
            sqlx::query_scalar(r#"SELECT id FROM pluralkit_id"#)
                .fetch_one(&mut *tx)
                .await?;
        tx.rollback().await?;

        assert_eq!(result, input);
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn it_encodes_for_mysql() -> Result<(), Box<dyn std::error::Error>> {
        let input = PluralKitId::<SystemMarker>::try_from("exmpl")?;
        let conn = sqlx::MySqlPool::connect(&std::env::var("MYSQL_URL")?).await?;

        let mut tx = conn.begin().await?;
        sqlx::query(r#"CREATE TEMPORARY TABLE pluralkit_id (id TEXT)"#)
            .execute(&mut *tx)
            .await?;
        sqlx::query(r#"INSERT INTO pluralkit_id (id) VALUES (?)"#)
            .bind(&input)
            .execute(&mut *tx)
            .await?;

        let result: PluralKitId<SystemMarker> = sqlx::query_scalar(r#"SELECT * FROM pluralkit_id"#)
            .fetch_one(&mut *tx)
            .await?;
        tx.rollback().await?;

        assert_eq!(result, input);

        Ok(())
    }

    #[tokio::test]
    async fn it_encodes_for_sqlite() -> Result<(), Box<dyn std::error::Error>> {
        let input = PluralKitId::<SystemMarker>::try_from("exmpl")?;
        let conn = sqlx::SqlitePool::connect(":memory:").await?;

        sqlx::query(r#"CREATE TABLE pluralkit_id (id TEXT)"#)
            .execute(&conn)
            .await?;
        sqlx::query(r#" INSERT INTO pluralkit_id (id) VALUES (?)"#)
            .bind(&input)
            .execute(&conn)
            .await?;

        let result: PluralKitId<SystemMarker> = sqlx::query_scalar(r#"SELECT * FROM pluralkit_id"#)
            .fetch_one(&conn)
            .await?;

        assert_eq!(result, input);

        Ok(())
    }
}
