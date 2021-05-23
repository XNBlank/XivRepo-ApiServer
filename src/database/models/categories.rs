use super::ids::*;
use super::DatabaseError;
use futures::TryStreamExt;

pub struct Loader {
    pub id: LoaderId,
    pub loader: String,
}

pub struct GameVersion {
    pub id: GameVersionId,
    pub version: String,
}

pub struct Category {
    pub id: CategoryId,
    pub category: String,
}

pub struct ReportType {
    pub id: ReportTypeId,
    pub report_type: String,
}

pub struct License {
    pub id: LicenseId,
    pub short: String,
    pub name: String,
}

pub struct DonationPlatform {
    pub id: DonationPlatformId,
    pub short: String,
    pub name: String,
}

pub struct CategoryBuilder<'a> {
    pub name: Option<&'a str>,
}

impl Category {
    pub fn builder() -> CategoryBuilder<'static> {
        CategoryBuilder { name: None }
    }

    pub async fn get_id<'a, E>(name: &str, exec: E) -> Result<Option<CategoryId>, DatabaseError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        if !name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        {
            return Err(DatabaseError::InvalidIdentifier(name.to_string()));
        }

        let result = sqlx::query!(
            "
            SELECT id FROM categories
            WHERE category = $1
            ",
            name
        )
        .fetch_optional(exec)
        .await?;

        Ok(result.map(|r| CategoryId(r.id)))
    }

    pub async fn get_name<'a, E>(id: CategoryId, exec: E) -> Result<String, DatabaseError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let result = sqlx::query!(
            "
            SELECT category FROM categories
            WHERE id = $1
            ",
            id as CategoryId
        )
        .fetch_one(exec)
        .await?;

        Ok(result.category)
    }

    pub async fn list<'a, E>(exec: E) -> Result<Vec<String>, DatabaseError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let result = sqlx::query!(
            "
            SELECT category FROM categories
            "
        )
        .fetch_many(exec)
        .try_filter_map(|e| async { Ok(e.right().map(|c| c.category)) })
        .try_collect::<Vec<String>>()
        .await?;

        Ok(result)
    }

    pub async fn remove<'a, E>(name: &str, exec: E) -> Result<Option<()>, DatabaseError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        use sqlx::Done;

        let result = sqlx::query!(
            "
            DELETE FROM categories
            WHERE category = $1
            ",
            name
        )
        .execute(exec)
        .await?;

        if result.rows_affected() == 0 {
            // Nothing was deleted
            Ok(None)
        } else {
            Ok(Some(()))
        }
    }
}

impl<'a> CategoryBuilder<'a> {
    /// The name of the category.  Must be ASCII alphanumeric or `-`/`_`
    pub fn name(self, name: &'a str) -> Result<CategoryBuilder<'a>, DatabaseError> {
        if name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        {
            Ok(Self { name: Some(name) })
        } else {
            Err(DatabaseError::InvalidIdentifier(name.to_string()))
        }
    }

    pub async fn insert<'b, E>(self, exec: E) -> Result<CategoryId, DatabaseError>
    where
        E: sqlx::Executor<'b, Database = sqlx::Postgres>,
    {
        let result = sqlx::query!(
            "
            INSERT INTO categories (category)
            VALUES ($1)
            ON CONFLICT (category) DO NOTHING
            RETURNING id
            ",
            self.name
        )
        .fetch_one(exec)
        .await?;

        Ok(CategoryId(result.id))
    }
}

pub struct LoaderBuilder<'a> {
    pub name: Option<&'a str>,
}

impl Loader {
    pub fn builder() -> LoaderBuilder<'static> {
        LoaderBuilder { name: None }
    }

    pub async fn get_id<'a, E>(name: &str, exec: E) -> Result<Option<LoaderId>, DatabaseError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        if !name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        {
            return Err(DatabaseError::InvalidIdentifier(name.to_string()));
        }

        let result = sqlx::query!(
            "
            SELECT id FROM loaders
            WHERE loader = $1
            ",
            name
        )
        .fetch_optional(exec)
        .await?;

        Ok(result.map(|r| LoaderId(r.id)))
    }

    pub async fn get_name<'a, E>(id: LoaderId, exec: E) -> Result<String, DatabaseError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let result = sqlx::query!(
            "
            SELECT loader FROM loaders
            WHERE id = $1
            ",
            id as LoaderId
        )
        .fetch_one(exec)
        .await?;

        Ok(result.loader)
    }

    pub async fn list<'a, E>(exec: E) -> Result<Vec<String>, DatabaseError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let result = sqlx::query!(
            "
            SELECT loader FROM loaders
            "
        )
        .fetch_many(exec)
        .try_filter_map(|e| async { Ok(e.right().map(|c| c.loader)) })
        .try_collect::<Vec<String>>()
        .await?;

        Ok(result)
    }

    // TODO: remove loaders with mods using them
    pub async fn remove<'a, E>(name: &str, exec: E) -> Result<Option<()>, DatabaseError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        use sqlx::Done;

        let result = sqlx::query!(
            "
            DELETE FROM loaders
            WHERE loader = $1
            ",
            name
        )
        .execute(exec)
        .await?;

        if result.rows_affected() == 0 {
            // Nothing was deleted
            Ok(None)
        } else {
            Ok(Some(()))
        }
    }
}

impl<'a> LoaderBuilder<'a> {
    /// The name of the loader.  Must be ASCII alphanumeric or `-`/`_`
    pub fn name(self, name: &'a str) -> Result<LoaderBuilder<'a>, DatabaseError> {
        if name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        {
            Ok(Self { name: Some(name) })
        } else {
            Err(DatabaseError::InvalidIdentifier(name.to_string()))
        }
    }

    pub async fn insert<'b, E>(self, exec: E) -> Result<LoaderId, DatabaseError>
    where
        E: sqlx::Executor<'b, Database = sqlx::Postgres>,
    {
        let result = sqlx::query!(
            "
            INSERT INTO loaders (loader)
            VALUES ($1)
            ON CONFLICT (loader) DO NOTHING
            RETURNING id
            ",
            self.name
        )
        .fetch_one(exec)
        .await?;

        Ok(LoaderId(result.id))
    }
}

#[derive(Default)]
pub struct GameVersionBuilder<'a> {
    pub version: Option<&'a str>,
    pub version_type: Option<&'a str>,
    pub date: Option<&'a chrono::DateTime<chrono::Utc>>,
}

impl GameVersion {
    pub fn builder() -> GameVersionBuilder<'static> {
        GameVersionBuilder::default()
    }

    pub async fn get_id<'a, E>(
        version: &str,
        exec: E,
    ) -> Result<Option<GameVersionId>, DatabaseError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        if !version
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || "-_.".contains(c))
        {
            return Err(DatabaseError::InvalidIdentifier(version.to_string()));
        }

        let result = sqlx::query!(
            "
            SELECT id FROM game_versions
            WHERE version = $1
            ",
            version
        )
        .fetch_optional(exec)
        .await?;

        Ok(result.map(|r| GameVersionId(r.id)))
    }

    pub async fn get_name<'a, E>(id: GameVersionId, exec: E) -> Result<String, DatabaseError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let result = sqlx::query!(
            "
            SELECT version FROM game_versions
            WHERE id = $1
            ",
            id as GameVersionId
        )
        .fetch_one(exec)
        .await?;

        Ok(result.version)
    }

    pub async fn list<'a, E>(exec: E) -> Result<Vec<String>, DatabaseError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let result = sqlx::query!(
            "
            SELECT version FROM game_versions
            ORDER BY created DESC
            "
        )
        .fetch_many(exec)
        .try_filter_map(|e| async { Ok(e.right().map(|c| c.version)) })
        .try_collect::<Vec<String>>()
        .await?;

        Ok(result)
    }

    pub async fn list_filter<'a, E>(
        version_type_option: Option<&str>,
        major_option: Option<bool>,
        exec: E,
    ) -> Result<Vec<String>, DatabaseError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let result;

        if let Some(version_type) = version_type_option {
            if let Some(major) = major_option {
                result = sqlx::query!(
                    "
                    SELECT version FROM game_versions
                    WHERE major = $1 AND type = $2
                    ORDER BY created DESC
                    ",
                    major,
                    version_type
                )
                .fetch_many(exec)
                .try_filter_map(|e| async { Ok(e.right().map(|c| c.version)) })
                .try_collect::<Vec<String>>()
                .await?;
            } else {
                result = sqlx::query!(
                    "
                    SELECT version FROM game_versions
                    WHERE type = $1
                    ORDER BY created DESC
                    ",
                    version_type
                )
                .fetch_many(exec)
                .try_filter_map(|e| async { Ok(e.right().map(|c| c.version)) })
                .try_collect::<Vec<String>>()
                .await?;
            }
        } else if let Some(major) = major_option {
            result = sqlx::query!(
                "
                SELECT version FROM game_versions
                WHERE major = $1
                ORDER BY created DESC
                ",
                major
            )
            .fetch_many(exec)
            .try_filter_map(|e| async { Ok(e.right().map(|c| c.version)) })
            .try_collect::<Vec<String>>()
            .await?;
        } else {
            result = Vec::new();
        }

        Ok(result)
    }

    pub async fn remove<'a, E>(name: &str, exec: E) -> Result<Option<()>, DatabaseError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        use sqlx::Done;

        let result = sqlx::query!(
            "
            DELETE FROM game_versions
            WHERE version = $1
            ",
            name
        )
        .execute(exec)
        .await?;

        if result.rows_affected() == 0 {
            // Nothing was deleted
            Ok(None)
        } else {
            Ok(Some(()))
        }
    }
}

impl<'a> GameVersionBuilder<'a> {
    /// The game version.  Spaces must be replaced with '_' for it to be valid
    pub fn version(self, version: &'a str) -> Result<GameVersionBuilder<'a>, DatabaseError> {
        if version
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || "-_.".contains(c))
        {
            Ok(Self {
                version: Some(version),
                ..self
            })
        } else {
            Err(DatabaseError::InvalidIdentifier(version.to_string()))
        }
    }

    pub fn version_type(
        self,
        version_type: &'a str,
    ) -> Result<GameVersionBuilder<'a>, DatabaseError> {
        if version_type
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || "-_.".contains(c))
        {
            Ok(Self {
                version_type: Some(version_type),
                ..self
            })
        } else {
            Err(DatabaseError::InvalidIdentifier(version_type.to_string()))
        }
    }

    pub fn created(self, created: &'a chrono::DateTime<chrono::Utc>) -> GameVersionBuilder<'a> {
        Self {
            date: Some(created),
            ..self
        }
    }

    pub async fn insert<'b, E>(self, exec: E) -> Result<GameVersionId, DatabaseError>
    where
        E: sqlx::Executor<'b, Database = sqlx::Postgres>,
    {
        // This looks like a mess, but it *should* work
        // This allows game versions to be partially updated without
        // replacing the unspecified fields with defaults.
        let result = sqlx::query!(
            "
            INSERT INTO game_versions (version, type, created)
            VALUES ($1, COALESCE($2, 'other'), COALESCE($3, timezone('utc', now())))
            ON CONFLICT (version) DO UPDATE
                SET type = COALESCE($2, game_versions.type),
                    created = COALESCE($3, game_versions.created)
            RETURNING id
            ",
            self.version,
            self.version_type,
            self.date.map(chrono::DateTime::naive_utc),
        )
        .fetch_one(exec)
        .await?;

        Ok(GameVersionId(result.id))
    }
}

#[derive(Default)]
pub struct LicenseBuilder<'a> {
    pub short: Option<&'a str>,
    pub name: Option<&'a str>,
}

impl License {
    pub fn builder() -> LicenseBuilder<'static> {
        LicenseBuilder::default()
    }

    pub async fn get_id<'a, E>(id: &str, exec: E) -> Result<Option<LicenseId>, DatabaseError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let result = sqlx::query!(
            "
            SELECT id FROM licenses
            WHERE short = $1
            ",
            id
        )
        .fetch_optional(exec)
        .await?;

        Ok(result.map(|r| LicenseId(r.id)))
    }

    pub async fn get<'a, E>(id: LicenseId, exec: E) -> Result<License, DatabaseError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let result = sqlx::query!(
            "
            SELECT short, name FROM licenses
            WHERE id = $1
            ",
            id as LicenseId
        )
        .fetch_one(exec)
        .await?;

        Ok(License {
            id,
            short: result.short,
            name: result.name,
        })
    }

    pub async fn list<'a, E>(exec: E) -> Result<Vec<License>, DatabaseError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let result = sqlx::query!(
            "
            SELECT id, short, name FROM licenses
            "
        )
        .fetch_many(exec)
        .try_filter_map(|e| async {
            Ok(e.right().map(|c| License {
                id: LicenseId(c.id),
                short: c.short,
                name: c.name,
            }))
        })
        .try_collect::<Vec<License>>()
        .await?;

        Ok(result)
    }

    pub async fn remove<'a, E>(short: &str, exec: E) -> Result<Option<()>, DatabaseError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        use sqlx::Done;

        let result = sqlx::query!(
            "
            DELETE FROM licenses
            WHERE short = $1
            ",
            short
        )
        .execute(exec)
        .await?;

        if result.rows_affected() == 0 {
            // Nothing was deleted
            Ok(None)
        } else {
            Ok(Some(()))
        }
    }
}

impl<'a> LicenseBuilder<'a> {
    /// The license's short name/abbreviation.  Spaces must be replaced with '_' for it to be valid
    pub fn short(self, short: &'a str) -> Result<LicenseBuilder<'a>, DatabaseError> {
        if short
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || "-_.".contains(c))
        {
            Ok(Self {
                short: Some(short),
                ..self
            })
        } else {
            Err(DatabaseError::InvalidIdentifier(short.to_string()))
        }
    }

    /// The license's long name
    pub fn name(self, name: &'a str) -> Result<LicenseBuilder<'a>, DatabaseError> {
        Ok(Self {
            name: Some(name),
            ..self
        })
    }

    pub async fn insert<'b, E>(self, exec: E) -> Result<LicenseId, DatabaseError>
    where
        E: sqlx::Executor<'b, Database = sqlx::Postgres>,
    {
        let result = sqlx::query!(
            "
            INSERT INTO licenses (short, name)
            VALUES ($1, $2)
            ON CONFLICT (short) DO NOTHING
            RETURNING id
            ",
            self.short,
            self.name,
        )
        .fetch_one(exec)
        .await?;

        Ok(LicenseId(result.id))
    }
}

#[derive(Default)]
pub struct DonationPlatformBuilder<'a> {
    pub short: Option<&'a str>,
    pub name: Option<&'a str>,
}

impl DonationPlatform {
    pub fn builder() -> DonationPlatformBuilder<'static> {
        DonationPlatformBuilder::default()
    }

    pub async fn get_id<'a, E>(
        id: &str,
        exec: E,
    ) -> Result<Option<DonationPlatformId>, DatabaseError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let result = sqlx::query!(
            "
            SELECT id FROM donation_platforms
            WHERE short = $1
            ",
            id
        )
        .fetch_optional(exec)
        .await?;

        Ok(result.map(|r| DonationPlatformId(r.id)))
    }

    pub async fn get<'a, E>(
        id: DonationPlatformId,
        exec: E,
    ) -> Result<DonationPlatform, DatabaseError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let result = sqlx::query!(
            "
            SELECT short, name FROM donation_platforms
            WHERE id = $1
            ",
            id as DonationPlatformId
        )
        .fetch_one(exec)
        .await?;

        Ok(DonationPlatform {
            id,
            short: result.short,
            name: result.name,
        })
    }

    pub async fn list<'a, E>(exec: E) -> Result<Vec<DonationPlatform>, DatabaseError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let result = sqlx::query!(
            "
            SELECT id, short, name FROM donation_platforms
            "
        )
        .fetch_many(exec)
        .try_filter_map(|e| async {
            Ok(e.right().map(|c| DonationPlatform {
                id: DonationPlatformId(c.id),
                short: c.short,
                name: c.name,
            }))
        })
        .try_collect::<Vec<DonationPlatform>>()
        .await?;

        Ok(result)
    }

    pub async fn remove<'a, E>(short: &str, exec: E) -> Result<Option<()>, DatabaseError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        use sqlx::Done;

        let result = sqlx::query!(
            "
            DELETE FROM donation_platforms
            WHERE short = $1
            ",
            short
        )
        .execute(exec)
        .await?;

        if result.rows_affected() == 0 {
            // Nothing was deleted
            Ok(None)
        } else {
            Ok(Some(()))
        }
    }
}

impl<'a> DonationPlatformBuilder<'a> {
    /// The donation platform short name.  Spaces must be replaced with '_' for it to be valid
    pub fn short(self, short: &'a str) -> Result<DonationPlatformBuilder<'a>, DatabaseError> {
        if short
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || "-_.".contains(c))
        {
            Ok(Self {
                short: Some(short),
                ..self
            })
        } else {
            Err(DatabaseError::InvalidIdentifier(short.to_string()))
        }
    }

    /// The donation platform long name
    pub fn name(self, name: &'a str) -> Result<DonationPlatformBuilder<'a>, DatabaseError> {
        Ok(Self {
            name: Some(name),
            ..self
        })
    }

    pub async fn insert<'b, E>(self, exec: E) -> Result<DonationPlatformId, DatabaseError>
    where
        E: sqlx::Executor<'b, Database = sqlx::Postgres>,
    {
        let result = sqlx::query!(
            "
            INSERT INTO donation_platforms (short, name)
            VALUES ($1, $2)
            ON CONFLICT (short) DO NOTHING
            RETURNING id
            ",
            self.short,
            self.name,
        )
        .fetch_one(exec)
        .await?;

        Ok(DonationPlatformId(result.id))
    }
}

pub struct ReportTypeBuilder<'a> {
    pub name: Option<&'a str>,
}

impl ReportType {
    pub fn builder() -> ReportTypeBuilder<'static> {
        ReportTypeBuilder { name: None }
    }

    pub async fn get_id<'a, E>(name: &str, exec: E) -> Result<Option<ReportTypeId>, DatabaseError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        if !name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        {
            return Err(DatabaseError::InvalidIdentifier(name.to_string()));
        }

        let result = sqlx::query!(
            "
            SELECT id FROM report_types
            WHERE name = $1
            ",
            name
        )
        .fetch_optional(exec)
        .await?;

        Ok(result.map(|r| ReportTypeId(r.id)))
    }

    pub async fn get_name<'a, E>(id: ReportTypeId, exec: E) -> Result<String, DatabaseError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let result = sqlx::query!(
            "
            SELECT name FROM report_types
            WHERE id = $1
            ",
            id as ReportTypeId
        )
        .fetch_one(exec)
        .await?;

        Ok(result.name)
    }

    pub async fn list<'a, E>(exec: E) -> Result<Vec<String>, DatabaseError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        let result = sqlx::query!(
            "
            SELECT name FROM report_types
            "
        )
        .fetch_many(exec)
        .try_filter_map(|e| async { Ok(e.right().map(|c| c.name)) })
        .try_collect::<Vec<String>>()
        .await?;

        Ok(result)
    }

    // TODO: remove loaders with mods using them
    pub async fn remove<'a, E>(name: &str, exec: E) -> Result<Option<()>, DatabaseError>
    where
        E: sqlx::Executor<'a, Database = sqlx::Postgres>,
    {
        use sqlx::Done;

        let result = sqlx::query!(
            "
            DELETE FROM report_types
            WHERE name = $1
            ",
            name
        )
        .execute(exec)
        .await?;

        if result.rows_affected() == 0 {
            // Nothing was deleted
            Ok(None)
        } else {
            Ok(Some(()))
        }
    }
}

impl<'a> ReportTypeBuilder<'a> {
    /// The name of the report type.  Must be ASCII alphanumeric or `-`/`_`
    pub fn name(self, name: &'a str) -> Result<ReportTypeBuilder<'a>, DatabaseError> {
        if name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        {
            Ok(Self { name: Some(name) })
        } else {
            Err(DatabaseError::InvalidIdentifier(name.to_string()))
        }
    }

    pub async fn insert<'b, E>(self, exec: E) -> Result<ReportTypeId, DatabaseError>
    where
        E: sqlx::Executor<'b, Database = sqlx::Postgres>,
    {
        let result = sqlx::query!(
            "
            INSERT INTO report_types (name)
            VALUES ($1)
            ON CONFLICT (name) DO NOTHING
            RETURNING id
            ",
            self.name
        )
        .fetch_one(exec)
        .await?;

        Ok(ReportTypeId(result.id))
    }
}
