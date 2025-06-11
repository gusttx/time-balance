use crate::{
    database::{models::Activity, Db},
    types::{ActivityError, CommandError},
};
use tauri::State;

#[tauri::command]
pub async fn get_all_activities(db: State<'_, Db>) -> Result<Vec<Activity>, CommandError> {
    log::debug!("Getting all activities");
    let activities = Activity::get_all(&db).await?;
    log::debug!("Got {} activities successfully", activities.len());

    Ok(activities)
}

#[tauri::command]
pub async fn create_activity(
    db: State<'_, Db>,
    name: String,
    points_per_second: i8,
) -> Result<Activity, CommandError> {
    let name = name.trim().chars().take(30).collect::<String>();
    if name.is_empty() {
        return Err(ActivityError::EmptyName.into());
    }

    let points_per_second = points_per_second.clamp(-10, 10);

    log::debug!("Creating activity: {}", name);
    let activity = Activity::create(&db, name, points_per_second).await?;
    log::debug!("Created activity '{}' successfully", activity.name);

    Ok(activity)
}

#[tauri::command]
pub async fn delete_activity(db: State<'_, Db>, id: i64) -> Result<(), CommandError> {
    log::debug!("Deleting activity: {}", id);
    Activity::delete(&db, id).await?;
    log::debug!("Deleted activity '{}' successfully", id);

    Ok(())
}

#[tauri::command]
pub async fn edit_activity(
    db: State<'_, Db>,
    id: i64,
    name: String,
    points_per_second: i8,
) -> Result<Activity, CommandError> {
    let name = name.trim().chars().take(30).collect::<String>();
    if name.is_empty() {
        return Err(ActivityError::EmptyName.into());
    }

    let points_per_second = points_per_second.clamp(-10, 10);

    log::debug!("Editing activity: {}", id);
    let activity = Activity::edit(&db, id, name, points_per_second).await?;
    log::debug!("Edited activity '{}' successfully", id);

    Ok(activity)
}
