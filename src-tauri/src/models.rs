use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Clone, serde::Serialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::projects, check_for_backend(diesel::sqlite::Sqlite))]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::projects)]
pub struct NewProject<'a> {
    pub name: &'a str,
    pub description: &'a str,
}

#[derive(Clone, serde::Serialize, Queryable, Selectable, Associations)]
#[diesel(table_name = crate::schema::tasks, check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Project))]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: i32,
    pub started_at: NaiveDateTime,
    pub finished_at: Option<NaiveDateTime>,
    pub description: String,
    pub project_id: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::tasks)]
pub struct NewTask<'a> {
    pub started_at: NaiveDateTime,
    pub finished_at: Option<NaiveDateTime>,
    pub description: &'a str,
    pub project_id: i32,
}

#[derive(Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskDTO {
    #[serde(flatten)]
    pub task: Task,
    pub project: Project,
}

impl From<(Task, Project)> for TaskDTO {
    fn from((task, project): (Task, Project)) -> Self {
        return Self { task, project };
    }
}
