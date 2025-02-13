// src/db/operations.rs
use diesel::prelude::*;
use diesel::result::Error as DieselError;

use crate::db::models::{ShortUrl, NewShortUrl};
use crate::db::schema::short_urls::dsl::*;

use crate::utils::crypto::encrypt_url;

// short urls
pub fn find_by_short_code(conn: &mut PgConnection, code: &str) -> Result<ShortUrl, DieselError> {
    short_urls
        .filter(short_code.eq(code))
        .select(ShortUrl::as_select())
        .first(conn)
}

pub fn find_by_original_url(conn: &mut PgConnection, url: &str) -> Result<ShortUrl, DieselError> {
    short_urls
        .filter(original_url.eq(url))
        .select(ShortUrl::as_select())
        .first(conn)
}

pub fn insert_short_url(
    conn: &mut PgConnection,
    new_short: &mut NewShortUrl
) -> Result<ShortUrl, DieselError> {
    // If the caller wants encryption
    if new_short.encrypted {
        if let Ok(cipher_b64) = encrypt_url(new_short.original_url) {
            new_short.original_url = Box::leak(cipher_b64.into_boxed_str());
        }
    }
    diesel::insert_into(short_urls)
        .values(&*new_short)
        .on_conflict(original_url)
        .do_nothing()
        .get_result::<ShortUrl>(conn)
}

pub fn mark_as_expired(conn: &mut PgConnection, short_url_id: i32) {
    use crate::db::schema::short_urls::dsl::*;
    let _ = diesel::update(short_urls.filter(id.eq(short_url_id)))
        .set(expired.eq(true))
        .execute(conn);
}

pub fn mark_as_expired_if_paid(
    conn: &mut PgConnection,
    short_url_id: i32,
) -> Result<ShortUrl, DieselError> {
    use crate::db::schema::short_urls::dsl::*;
    
    // 1) Fetch the existing row
    let existing = short_urls
        .filter(id.eq(short_url_id))
        .first::<ShortUrl>(conn)?;

    // 2) Check if transaction_hash is Some(...)
    if existing.transaction_hash.is_none() {
        // No payment => We do NOT set expired
        // Return an error or do something else
        return Err(DieselError::RollbackTransaction);
        // or DieselError::NotFound or your custom logic
    }

    // 3) If user paid, set expired = true
    let updated = diesel::update(short_urls.filter(id.eq(short_url_id)))
        .set(expired.eq(true))
        .get_result::<ShortUrl>(conn)?;

    Ok(updated)
}

pub fn get_all_short_urls(
    conn: &mut PgConnection
) -> Result<Vec<ShortUrl>, DieselError> {
    short_urls
        .select(ShortUrl::as_select())
        .load(conn)
}