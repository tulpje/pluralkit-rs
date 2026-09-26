use sqlx::{
    Database, Decode, Encode, Type,
    encode::IsNull,
    error::BoxDynError,
    postgres::{PgHasArrayType, PgTypeInfo},
};
use uuid::Uuid;

use crate::models::PluralKitUuid;

impl<DB, T> Type<DB> for PluralKitUuid<T>
where
    DB: Database,
    Uuid: sqlx::Type<DB>,
{
    fn type_info() -> <DB as Database>::TypeInfo {
        <Uuid as Type<DB>>::type_info()
    }

    fn compatible(ty: &<DB as Database>::TypeInfo) -> bool {
        <Uuid as Type<DB>>::compatible(ty)
    }
}

impl<'q, DB, T> Encode<'q, DB> for PluralKitUuid<T>
where
    DB: Database,
    Uuid: Encode<'q, DB>,
{
    fn encode_by_ref(
        &self,
        buf: &mut <DB as Database>::ArgumentBuffer<'q>,
    ) -> Result<IsNull, BoxDynError> {
        <Uuid as Encode<'q, DB>>::encode(self.value, buf)
    }
}

impl<'r, DB, T> Decode<'r, DB> for PluralKitUuid<T>
where
    DB: Database,
    Uuid: Decode<'r, DB>,
{
    fn decode(value: <DB as Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
        <Uuid as Decode<DB>>::decode(value).map(Self::new)
    }
}

impl<T> PgHasArrayType for PluralKitUuid<T> {
    fn array_type_info() -> PgTypeInfo {
        <Uuid as PgHasArrayType>::array_type_info()
    }

    fn array_compatible(ty: &PgTypeInfo) -> bool {
        <Uuid as PgHasArrayType>::array_compatible(ty)
    }
}

#[cfg(test)]
mod test {
    use crate::models::{PluralKitUuid, marker::SystemMarker};

    #[tokio::test]
    #[ignore]
    async fn it_decodes_for_postgres() -> Result<(), Box<dyn std::error::Error>> {
        let expected =
            PluralKitUuid::<SystemMarker>::try_from("aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa")?;
        let conn = sqlx::PgPool::connect(&std::env::var("POSTGRES_URL")?).await?;

        let result: PluralKitUuid<SystemMarker> = sqlx::query_scalar(
            r#"SELECT 'aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa'::uuid AS "id: PluralKitUuid<SystemMarker>""#,
        )
        .fetch_one(&conn)
        .await?;

        assert_eq!(result, expected);

        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn it_decodes_for_mysql() -> Result<(), Box<dyn std::error::Error>> {
        let expected =
            PluralKitUuid::<SystemMarker>::try_from("aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa")?;
        let conn = sqlx::MySqlPool::connect(&std::env::var("MYSQL_URL")?).await?;

        let result: PluralKitUuid<SystemMarker> = sqlx::query_scalar(
            r#"SELECT UNHEX('aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa') AS 'id: PluralKitUuid<SystemMarker>'"#,
        )
        .fetch_one(&conn)
        .await?;

        assert_eq!(result, expected);

        Ok(())
    }

    #[tokio::test]
    async fn it_decodes_for_sqlite() -> Result<(), Box<dyn std::error::Error>> {
        let expected =
            PluralKitUuid::<SystemMarker>::try_from("aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa")?;
        let conn = sqlx::SqlitePool::connect(":memory:").await?;

        let result: PluralKitUuid<SystemMarker> = sqlx::query_scalar(
            r#"SELECT x'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa' AS "uuid: PluralKitUuid<SystemMarker>""#,
        )
        .fetch_one(&conn)
        .await?;

        assert_eq!(result, expected);

        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn it_encodes_for_postgres() -> Result<(), Box<dyn std::error::Error>> {
        let input =
            PluralKitUuid::<SystemMarker>::try_from("aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa")?;
        let conn = sqlx::PgPool::connect(&std::env::var("POSTGRES_URL")?).await?;

        let mut tx = conn.begin().await?;
        sqlx::query(r#"CREATE TEMPORARY TABLE pluralkit_uuid (uuid UUID)"#)
            .execute(&mut *tx)
            .await?;
        sqlx::query(r#"INSERT INTO pluralkit_uuid (uuid) VALUES ($1)"#)
            .bind(input)
            .execute(&mut *tx)
            .await?;

        let result: PluralKitUuid<SystemMarker> =
            sqlx::query_scalar(r#"SELECT uuid FROM pluralkit_uuid"#)
                .fetch_one(&mut *tx)
                .await?;
        tx.rollback().await?;

        assert_eq!(result, input);
        Ok(())
    }

    #[tokio::test]
    #[ignore]
    async fn it_encodes_for_mysql() -> Result<(), Box<dyn std::error::Error>> {
        let input =
            PluralKitUuid::<SystemMarker>::try_from("aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa")?;
        let conn = sqlx::MySqlPool::connect(&std::env::var("MYSQL_URL")?).await?;

        let mut tx = conn.begin().await?;
        sqlx::query(r#"CREATE TEMPORARY TABLE pluralkit_uuid (uuid BINARY(16))"#)
            .execute(&mut *tx)
            .await?;
        sqlx::query(r#"INSERT INTO pluralkit_uuid (uuid) VALUES (?)"#)
            .bind(input)
            .execute(&mut *tx)
            .await?;

        let result: PluralKitUuid<SystemMarker> =
            sqlx::query_scalar(r#"SELECT uuid FROM pluralkit_uuid"#)
                .fetch_one(&mut *tx)
                .await?;
        tx.rollback().await?;

        assert_eq!(result, input);

        Ok(())
    }

    #[tokio::test]
    async fn it_encodes_for_sqlite() -> Result<(), Box<dyn std::error::Error>> {
        let input =
            PluralKitUuid::<SystemMarker>::try_from("aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa")?;
        let conn = sqlx::SqlitePool::connect(":memory:").await?;

        sqlx::query(r#"CREATE TABLE pluralkit_uuid (uuid TEXT)"#)
            .execute(&conn)
            .await?;
        sqlx::query(r#" INSERT INTO pluralkit_uuid (uuid) VALUES (?)"#)
            .bind(input)
            .execute(&conn)
            .await?;

        let result: PluralKitUuid<SystemMarker> =
            sqlx::query_scalar(r#"SELECT uuid FROM pluralkit_uuid"#)
                .fetch_one(&conn)
                .await?;

        assert_eq!(result, input);

        Ok(())
    }
}
