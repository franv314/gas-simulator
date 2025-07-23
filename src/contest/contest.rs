use chrono::{Duration, DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Question {
    pub id: i32,
    pub answer: i32,
    pub score: i64,
    pub locked: bool,
}

#[derive(Clone, PartialEq, Eq, Copy, Default, Serialize, Deserialize)]
pub enum QuestionStatus {
    #[default]
    NotAttempted,
    Attempted,
    JustSolved,
    Solved,
}

#[derive(Clone, Copy, Default, Serialize, Deserialize)]
pub struct TeamQuestion {
    pub score: i64,
    pub jolly: bool,
    pub status: QuestionStatus,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Submission {
    pub id: i32,
    pub given_answer: i32,
    pub question_pos: i32,
    pub correct: bool,
    pub team_name: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Jolly {
    pub id: i32,
    pub question_pos: i32,
    pub team_name: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub is_fake: bool,
    pub score: i64,
    pub questions: Vec<TeamQuestion>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Contest {
    pub id: i32,
    pub name: String,
    pub phiquadro_id: i32,
    pub phiquadro_sess: i32,
    pub questions: Vec<Question>,
    pub teams: Vec<Team>,
    pub duration: Duration,
    pub start_time: DateTime<Utc>,
    pub drift: i32,
    pub drift_time: Duration,
    pub jolly_time: Duration,
    pub question_bonus: Vec<i32>,
    pub contest_bonus: Vec<i32>,
}
