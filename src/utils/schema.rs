use sqlx::PgPool;

pub async fn ensure_situacao_enum(pool: &PgPool) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    sqlx::query("SELECT pg_advisory_xact_lock(hashtext('situacao_registro_migration'))")
        .execute(&mut *tx)
        .await?;

    sqlx::query(
        "DO $$
        BEGIN
            IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'situacao_registro') THEN
                CREATE TYPE situacao_registro AS ENUM ('Ativo', 'Inativo');
            END IF;
        END
        $$;",
    )
    .execute(&mut *tx)
    .await?;

    let pedagio_tipo = sqlx::query_scalar::<_, String>(
        "SELECT udt_name
         FROM information_schema.columns
         WHERE table_schema = 'public'
           AND table_name = 'pedagio'
           AND column_name = 'situacao'",
    )
    .fetch_one(&mut *tx)
    .await?;

    if pedagio_tipo != "situacao_registro" {
        sqlx::query(
            "ALTER TABLE pedagio
             ALTER COLUMN situacao TYPE situacao_registro
             USING situacao::situacao_registro",
        )
        .execute(&mut *tx)
        .await?;
    }

    let tarifas_tipo = sqlx::query_scalar::<_, String>(
        "SELECT udt_name
         FROM information_schema.columns
         WHERE table_schema = 'public'
           AND table_name = 'tarifas'
           AND column_name = 'situacao'",
    )
    .fetch_one(&mut *tx)
    .await?;

    if tarifas_tipo != "situacao_registro" {
        sqlx::query(
            "ALTER TABLE tarifas
             ALTER COLUMN situacao TYPE situacao_registro
             USING situacao::situacao_registro",
        )
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    Ok(())
}
