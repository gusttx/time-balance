use tauri::State;

use crate::{database::{models::{Activity, Entry}, Db}, types::{CommandError, EntryError}};

#[tauri::command]
pub async fn create_entry(db: State<'_, Db>, duration: i64, activity_id: i64) -> Result<Entry, CommandError> {
    if duration < 0 {
        return Err(EntryError::NegativeDuration.into());
    }

    let activity = Activity::get(&db, activity_id)
        .await?
        .ok_or(EntryError::ActivityNotFound)?;

    log::debug!("Creating entry with activity '{}' and duration '{}'", activity.id, duration);
    let entry = Entry::create(&db, duration, activity).await?;
    log::debug!("Created entry '{}' successfully", entry.id);
    
    Ok(entry)
}

#[tauri::command]
pub async fn get_entry_page(db: State<'_, Db>, page: i64) -> Result<(Vec<Entry>, i64), CommandError> {
    if page <= 0 {
        return Err(EntryError::ZeroOrNegativePage.into());
    }
    
    log::debug!("Getting entry page: {}", page);
    let result = Entry::get_page(&db, page - 1).await?;
    log::debug!("Successfully got entry page {} with {} entries, total entries: {}", page, result.0.len(), result.1);
    Ok(result)
}

#[tauri::command]
pub async fn delete_entry(db: State<'_, Db>, id: i64) -> Result<(), CommandError> {
    log::debug!("Deleting entry '{}'", id);
    Entry::delete(&db, id).await?;
    log::debug!("Deleted entry '{}' successfully", id);
    Ok(())
}