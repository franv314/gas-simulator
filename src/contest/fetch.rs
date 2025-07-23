use std::cmp;
use std::collections::HashMap;

use chrono::TimeDelta;
use diesel::{ExpressionMethods, QueryDsl};
use rocket_db_pools::diesel::prelude::RunQueryDsl;
use rocket_db_pools::Connection;
use tracing::info;

use super::contest::{Contest, Question, Team};
use crate::contest::contest::{Jolly, QuestionStatus, Submission, TeamQuestion};
use crate::model::{self, ContestJolly, ContestSubmission};

use crate::DB;

pub async fn fetch_contest(db: &mut Connection<DB>, user_id: i32, id: i32) -> anyhow::Result<Option<Contest>> {
    use crate::schema::{contests, questions, teams};

    info!("Loading contest {}", id);

    let contest = contests::dsl::contests
        .select((
            contests::phiquadro_id,
            contests::phiquadro_sess,
            contests::contest_name,
            contests::duration,
            contests::start_time,
            contests::drift,
            contests::drift_time,
            contests::jolly_time,
            contests::teams_no,
            contests::questions_no,
            contests::active,
            contests::question_bonus,
            contests::contest_bonus,
            contests::owner_id,
        ))
        .filter(contests::id.eq(id))
        .filter(contests::active.eq(true))
        .load::<model::Contest>(db)
        .await?;

    let Some(contest) = contest.get(0) else {
        return Ok(None);
    };

    if contest.owner_id != user_id {
        return Ok(None);
    }

    let questions = questions::dsl::questions
        .select((questions::id, questions::answer))
        .filter(questions::contest_id.eq(id))
        .order(questions::position.asc())
        .load::<(i32, i32)>(db)
        .await?;

    let teams = teams::dsl::teams
        .select((
            teams::id,
            teams::team_name,
            teams::is_fake,
            teams::position,
            teams::contest_id,
        ))
        .filter(teams::contest_id.eq(id))
        .load::<model::TeamWithId>(db)
        .await?;

    let questions: Vec<Question> = questions
        .iter()
        .map(|&(id, answer)| Question {
            id,
            answer,
            score: 20,
            locked: false,
        })
        .collect();

    let teams: Vec<Team> = teams
        .iter()
        .map(|team| Team {
            id: team.id,
            name: team.team_name.clone(),
            is_fake: team.is_fake,
            score: questions.len() as i64 * 10,
            questions: vec![TeamQuestion::default(); questions.len()],
        })
        .collect();

    Ok(Some(Contest {
        id,
        name: contest.contest_name.clone(),
        phiquadro_id: contest.phiquadro_id,
        phiquadro_sess: contest.phiquadro_sess,
        duration: TimeDelta::seconds(contest.duration as i64),
        drift: contest.drift,
        start_time: contest.start_time,
        questions,
        teams,
        drift_time: TimeDelta::seconds(contest.drift_time as i64),
        jolly_time: TimeDelta::seconds(contest.jolly_time as i64),
        question_bonus: contest.question_bonus.iter().map(|&x| x.expect("Question bonus can't be null")).collect(),
        contest_bonus: contest.contest_bonus.iter().map(|&x| x.expect("Contest bonus can't be null")).collect(),
    }))
}


pub async fn fetch_submissions(db: &mut Connection<DB>, user_id: i32, id: i32) -> anyhow::Result<Option<Vec<Submission>>> {
    use crate::schema::{contests, questions, submissions, teams};

    let contest_owner = contests::dsl::contests
        .select(contests::owner_id)
        .filter(contests::id.eq(id))
        .filter(contests::active.eq(true))
        .load::<i32>(db)
        .await?;

    let Some(contest_owner) = contest_owner.get(0) else {
        return Ok(None);
    };

    if *contest_owner != user_id {
        return Ok(None);
    }

    let teams = teams::dsl::teams
        .select((
            teams::position,
            teams::team_name,
        ))
        .filter(teams::contest_id.eq(id))
        .filter(teams::is_fake.eq(false))
        .load::<(i32, String)>(db)
        .await?
        .into_iter()
        .collect::<HashMap<i32, String>>();
    
    let submissions = submissions::dsl::submissions
        .inner_join(questions::table)
        .inner_join(teams::table)
        .select((
            submissions::id,
            submissions::answer,
            submissions::sub_time,
            questions::answer,
            questions::position,
            teams::position,
            teams::is_fake,
            teams::contest_id,
        ))
        .filter(teams::contest_id.eq(id))
        .filter(teams::is_fake.eq(false))
        .order((submissions::sub_time.asc(), submissions::id.asc()))
        .load::<ContestSubmission>(db)
        .await
        .map(|subs| subs.into_iter().map(|sub| Submission {
            id: sub.id,
            given_answer: sub.given_answer,
            correct: sub.given_answer == sub.correct_answer,
            question_pos: sub.question_pos,
            team_name: teams[&sub.team_pos].clone(),
        }))?
        .collect::<Vec<_>>();

    Ok(Some(submissions))
}

pub async fn fetch_jollies(db: &mut Connection<DB>, user_id: i32, id: i32) -> anyhow::Result<Option<Vec<Jolly>>> {
    use crate::schema::{contests, questions, jollies, teams};

    let contest_owner = contests::dsl::contests
        .select(contests::owner_id)
        .filter(contests::id.eq(id))
        .filter(contests::active.eq(true))
        .load::<i32>(db)
        .await?;

    let Some(contest_owner) = contest_owner.get(0) else {
        return Ok(None);
    };

    if *contest_owner != user_id {
        return Ok(None);
    }

    let teams = teams::dsl::teams
        .select((
            teams::position,
            teams::team_name,
        ))
        .filter(teams::contest_id.eq(id))
        .filter(teams::is_fake.eq(false))
        .load::<(i32, String)>(db)
        .await?
        .into_iter()
        .collect::<HashMap<i32, String>>();
    
    let jollies = jollies::dsl::jollies
        .inner_join(questions::table)
        .inner_join(teams::table)
        .select((
            jollies::id,
            jollies::sub_time,
            questions::position,
            teams::position,
            teams::contest_id,
        ))
        .filter(teams::contest_id.eq(id))
        .filter(teams::is_fake.eq(false))
        .order((jollies::sub_time.asc(), jollies::id.asc()))
        .load::<ContestJolly>(db)
        .await
        .map(|jollies| jollies.into_iter().map(|jolly| Jolly {
            id: jolly.id,
            question_pos: jolly.question_pos,
            team_name: teams[&jolly.team_pos].clone(),
        }))?
        .collect::<Vec<_>>();

    Ok(Some(jollies))
}

pub async fn fetch_contest_with_ranking(db: &mut Connection<DB>, user_id: i32, id: i32) -> anyhow::Result<Option<Contest>> {
    use crate::schema::{jollies, questions, submissions, teams};

    let Some(mut contest) = fetch_contest(db, user_id, id).await? else {
        return Ok(None);
    };

    let Contest {
        questions,
        teams,
        drift,
        drift_time,
        start_time,
        question_bonus,
        contest_bonus,
        ..
    } = &mut contest;

    let now = chrono::offset::Utc::now();

    let submissions = submissions::dsl::submissions
        .inner_join(questions::table)
        .inner_join(teams::table)
        .select((
            submissions::id,
            submissions::answer,
            submissions::sub_time,
            questions::answer,
            questions::position,
            teams::position,
            teams::is_fake,
            teams::contest_id,
        ))
        .filter(teams::contest_id.eq(id))
        .filter(submissions::sub_time.le(now))
        .order((submissions::sub_time.asc(), submissions::id.asc()))
        .load::<ContestSubmission>(db)
        .await?;

    let jollies = jollies::dsl::jollies
        .inner_join(questions::table)
        .inner_join(teams::table)
        .select((
            jollies::id,
            jollies::sub_time,
            questions::position,
            teams::position,
            teams::contest_id,
        ))
        .filter(teams::contest_id.eq(id))
        .filter(jollies::sub_time.le(now))
        .load::<ContestJolly>(db)
        .await?;

    let drift_no = *drift;
    let mut correct = vec![0; questions.len()];
    let mut wrong = vec![vec![false; questions.len()]; teams.len()];
    let mut drift = vec![*drift_time; questions.len()];

    for submission in &submissions {
        let q_pos = submission.question_pos as usize;
        let t_pos = submission.team_pos as usize;
        let sub_time = submission.sub_time - *start_time;

        if submission.given_answer == submission.correct_answer {
            correct[q_pos] += 1;
            if correct[q_pos] >= drift_no {
                drift[q_pos] = cmp::min(drift[q_pos], sub_time);
            }
        } else {
            if sub_time < *drift_time && submission.given_answer != submission.correct_answer {
                if correct[q_pos] == 0 && !wrong[t_pos][q_pos] {
                    questions[q_pos].score += 2;
                }
                wrong[t_pos][q_pos] = true;
            }
        }
    }

    for i in 0..questions.len() {
        questions[i].score += cmp::min(drift[i], now - *start_time).num_minutes().max(0);
        if chrono::offset::Utc::now() >= *start_time + drift[i] {
            questions[i].locked = true;
        }
    }

    let mut question_solves = vec![0; questions.len()];
    let mut team_solves = vec![0; teams.len()];
    let mut solves = 0;

    for submission in &submissions {
        let q_pos = submission.question_pos as usize;
        let t_pos = submission.team_pos as usize;

        if submission.given_answer == submission.correct_answer {
            if
                teams[t_pos].questions[q_pos].status != QuestionStatus::Solved
                && teams[t_pos].questions[q_pos].status != QuestionStatus::JustSolved
            {
                teams[t_pos].questions[q_pos].score +=
                    questions[q_pos].score + *question_bonus.get(question_solves[q_pos]).unwrap_or(&0) as i64;

                question_solves[q_pos] += 1;
                team_solves[t_pos] += 1;

                if team_solves[t_pos] == questions.len() {
                    teams[t_pos].score += *contest_bonus.get(solves).unwrap_or(&0) as i64;
                    solves += 1;
                }

                teams[t_pos].questions[q_pos].status =if submission.sub_time >= now - TimeDelta::minutes(1) {
                    QuestionStatus::JustSolved
                } else {
                    QuestionStatus::Solved
                };
            }
        } else {
            if teams[t_pos].questions[q_pos].status == QuestionStatus::NotAttempted {
                teams[t_pos].questions[q_pos].status = QuestionStatus::Attempted;
            }
            teams[t_pos].questions[q_pos].score -= 10;
        }
    }

    for jolly in jollies {
        teams[jolly.team_pos as usize].questions[jolly.question_pos as usize].score *= 2;
        teams[jolly.team_pos as usize].questions[jolly.question_pos as usize].jolly = true;
    }

    for team in teams.iter_mut() {
        team.score += team.questions.iter().map(|q| q.score).sum::<i64>();
    }

    teams.sort_unstable_by_key(|team| -team.score);

    Ok(Some(contest))
}
