// @generated automatically by Diesel CLI.

diesel::table! {
    projects (id) {
        id -> Integer,
        name -> Text,
        description -> Text,
    }
}

diesel::table! {
    tasks (id) {
        id -> Integer,
        started_at -> Timestamp,
        finished_at -> Nullable<Timestamp>,
        description -> Text,
        project_id -> Integer,
    }
}

diesel::joinable!(tasks -> projects (project_id));

diesel::allow_tables_to_appear_in_same_query!(
    projects,
    tasks,
);
