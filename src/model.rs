use chrono::{DateTime, TimeDelta, Utc};
use diesel::{data_types::PgInterval, Insertable, Queryable, Selectable};
use serde::Serialize;

#[derive(Debug, Clone, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::contests)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Contest {
    pub phiquadro_id: i32,
    pub phiquadro_sess: i32,
    pub contest_name: String,
    pub duration: i32,
    pub start_time: DateTime<Utc>,
    pub drift: i32,
    pub drift_time: i32,
    pub jolly_time: i32,
    pub teams_no: i32,
    pub questions_no: i32,
    pub active: bool,
    pub question_bonus: Vec<Option<i32>>,
    pub contest_bonus: Vec<Option<i32>>,
    pub owner_id: i32,
}

#[derive(Debug, Clone, Copy, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::questions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Question {
    pub answer: i32,
    pub position: i32,
    pub contest_id: i32,
}

#[derive(Debug, Clone, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::teams)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Team {
    pub team_name: String,
    pub is_fake: bool,
    pub position: i32,
    pub contest_id: i32,
}

#[derive(Debug, Clone, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::submissions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Submission {
    pub answer: i32,
    pub sub_time: DateTime<Utc>,
    pub team_id: i32,
    pub question_id: i32,
}

#[derive(Debug, Clone, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::jollies)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Jolly {
    pub sub_time: DateTime<Utc>,
    pub team_id: i32,
    pub question_id: i32,
}

#[derive(Debug, Clone, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub username: String,
    pub password_hash: Vec<u8>,
    pub salt: Vec<u8>,
}

#[derive(Debug, Clone, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Token {
    pub user_id: i32,
    pub token: String,
    pub expires: DateTime<Utc>,
}

#[derive(Queryable, Clone, Copy)]
pub struct ContestSubmission {
    pub id: i32,
    pub given_answer: i32,
    pub sub_time: DateTime<Utc>,
    pub correct_answer: i32,
    pub question_pos: i32,
    pub team_pos: i32,
    pub is_fake: bool,
    pub contest_id: i32,
}

#[derive(Queryable, Clone, Copy)]
pub struct ContestJolly {
    pub id: i32,
    pub sub_time: DateTime<Utc>,
    pub question_pos: i32,
    pub team_pos: i32,
    pub contest_id: i32,
}

#[derive(Queryable, Serialize, Clone)]
pub struct ContestWithId {
    pub id: i32,
    pub phiquadro_id: i32,
    pub phiquadro_sess: i32,
    pub contest_name: String,
    pub duration: i32,
    pub start_time: DateTime<Utc>,
    pub drift: i32,
    pub drift_time: i32,
    pub jolly_time: i32,
    pub teams_no: i32,
    pub questions_no: i32,
    pub bool: bool,
}

#[derive(Queryable)]
pub struct TeamWithId {
    pub id: i32,
    pub team_name: String,
    pub is_fake: bool,
    pub position: i32,
    pub contest_id: i32,
}

pub fn timedelta_to_pg_interval(delta: TimeDelta) -> PgInterval {
    const MILLISECONDS_IN_DAYS: i64 = 1_000 * 86_400;

    let millisecond = delta.num_milliseconds();
    PgInterval {
        microseconds: (millisecond % MILLISECONDS_IN_DAYS) * 1_000,
        days: ((millisecond / MILLISECONDS_IN_DAYS) % 30) as i32,
        months: ((millisecond / MILLISECONDS_IN_DAYS) / 30) as i32,
    }
}
