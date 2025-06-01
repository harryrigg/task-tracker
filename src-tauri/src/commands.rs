use diesel::prelude::*;
use tauri::State;

use crate::{error::CommandError, models, schema, App};

type Result<T> = std::result::Result<T, CommandError>;

#[tauri::command]
pub fn list_projects(app: State<'_, App>) -> Result<Vec<models::Project>> {
    use schema::projects::dsl;

    let results = dsl::projects
        .select(models::Project::as_select())
        .load(&mut *app.db())?;

    Ok(results)
}

#[tauri::command]
pub fn create_project(
    app: State<'_, App>,
    name: String,
    description: String,
) -> Result<models::Project> {
    use schema::projects;

    let new_project = models::NewProject {
        name: &name,
        description: &description,
    };

    let result = diesel::insert_into(projects::table)
        .values(&new_project)
        .returning(models::Project::as_returning())
        .get_result(&mut *app.db())?;

    Ok(result)
}

#[tauri::command]
pub fn update_project(
    app: State<'_, App>,
    id: i32,
    name: String,
    description: String,
) -> Result<()> {
    use schema::projects::dsl;

    diesel::update(dsl::projects)
        .filter(dsl::id.eq(id))
        .set((dsl::name.eq(name), dsl::description.eq(description)))
        .execute(&mut *app.db())?;

    Ok(())
}

#[tauri::command]
pub fn delete_project(app: State<'_, App>, id: i32) -> Result<()> {
    use schema::projects::dsl;

    diesel::delete(dsl::projects)
        .filter(dsl::id.eq(id))
        .execute(&mut *app.db())?;

    Ok(())
}

#[tauri::command]
pub fn start_task(app: State<'_, App>, project_id: i32, description: String) -> Result<()> {
    let new_task = models::NewTask {
        started_at: chrono::Utc::now().naive_utc(),
        finished_at: None,
        description: &description,
        project_id,
    };

    diesel::insert_into(schema::tasks::table)
        .values(&new_task)
        .execute(&mut *app.db())?;

    Ok(())
}

#[tauri::command]
pub fn stop_task(app: State<'_, App>) -> Result<()> {
    use crate::schema::tasks::dsl;

    let current_task = dsl::tasks
        .select(models::Task::as_select())
        .filter(dsl::finished_at.is_null())
        .limit(1)
        .load(&mut *app.db())?
        .into_iter()
        .next()
        .ok_or_else(|| CommandError::IllegalState)?;

    diesel::update(dsl::tasks)
        .filter(dsl::id.eq(current_task.id))
        .set(dsl::finished_at.eq(chrono::Utc::now().naive_utc()))
        .execute(&mut *app.db())?;

    Ok(())
}

#[tauri::command]
pub fn current_task(app: State<'_, App>) -> Result<models::TaskDTO> {
    use schema::tasks::dsl;

    let results = dsl::tasks
        .inner_join(schema::projects::table)
        .select((models::Task::as_select(), models::Project::as_select()))
        .filter(dsl::finished_at.is_null())
        .limit(1)
        .load::<(models::Task, models::Project)>(&mut *app.db())?;

    let result = results
        .into_iter()
        .next()
        .map(|v| v.into())
        .ok_or_else(|| CommandError::NoValue)?;

    Ok(result)
}

#[tauri::command]
pub fn update_task(
    app: State<'_, App>,
    id: i32,
    project_id: i32,
    description: String,
    started_at: chrono::DateTime<chrono::Utc>,
    finished_at: Option<chrono::DateTime<chrono::Utc>>,
) -> Result<()> {
    use schema::tasks::dsl;

    diesel::update(dsl::tasks)
        .filter(dsl::id.eq(id))
        .set((
            dsl::project_id.eq(project_id),
            dsl::description.eq(description),
            dsl::started_at.eq(started_at.naive_utc()),
            dsl::finished_at.eq(finished_at.map(|v| v.naive_utc())),
        ))
        .execute(&mut *app.db())?;

    Ok(())
}

#[tauri::command]
pub fn delete_task(app: State<'_, App>, id: i32) -> Result<()> {
    use schema::tasks::dsl;

    diesel::delete(dsl::tasks)
        .filter(dsl::id.eq(id))
        .execute(&mut *app.db())?;

    Ok(())
}

#[tauri::command]
pub fn list_tasks_between(
    app: State<'_, App>,
    start: chrono::DateTime<chrono::Utc>,
    end: chrono::DateTime<chrono::Utc>,
) -> Result<Vec<models::TaskDTO>> {
    use schema::tasks::dsl;

    let results = dsl::tasks
        .inner_join(schema::projects::table)
        .select((models::Task::as_select(), models::Project::as_select()))
        .order(dsl::finished_at.desc())
        .filter(dsl::started_at.ge(start.naive_utc()))
        .filter(dsl::finished_at.lt(end.naive_utc()))
        .load::<(models::Task, models::Project)>(&mut *app.db())?
        .into_iter()
        .map(|v| v.into())
        .collect();

    Ok(results)
}
