use anyhow::anyhow;
use diesel::{ExpressionMethods, QueryDsl};
use rocket::http::Status;
use rocket::Route;
use rocket_db_pools::diesel::prelude::RunQueryDsl;
use rocket_db_pools::Connection;
use rocket_dyn_templates::context;
use rocket_dyn_templates::Template;

use super::fetch::{fetch_contest, fetch_contest_with_ranking};
use crate::api::ApiUser;
use crate::error::IntoStatusResult;
use crate::{model, DB};

#[get("/create")]
async fn create_contest(_api_user: ApiUser) -> Template {
    Template::render("create", context! {})
}

#[get("/create", rank = 2)]
async fn create_contest_unauthorized() -> Status {
    Status::Unauthorized
}

#[get("/contest/<id>")]
pub async fn show_contest(id: i32, mut db: Connection<DB>) -> Result<Template, Status> {
    match fetch_contest_with_ranking(&mut db, id)
        .await
        .attach_info(Status::InternalServerError, "")?
    {
        Some(contest) => Ok(Template::render("ranking", contest)),
        None => Err(Status::NotFound),
    }
}

#[get("/settings/<id>")]
async fn contest_settings(id: i32, mut db: Connection<DB>) -> Result<Template, Status> {
    match fetch_contest(&mut db, id)
        .await
        .attach_info(Status::InternalServerError, "")?
    {
        Some(contest) => Ok(Template::render("settings", contest)),
        None => Err(Status::NotFound),
    }
}

#[get("/")]
async fn show_contest_list(mut db: Connection<DB>, user: Option<ApiUser>) -> Result<Template, Status> {
    use crate::schema::contests;

    let filter = match &user {
        Some(user) => contests::owner_id.eq(user.user_id),
        None => contests::owner_id.eq(-1),
    };

    let contests = contests::dsl::contests
        .select((
            contests::id,
            contests::phiquadro_id,
            contests::phiquadro_sess,
            contests::contest_name,
            contests::duration,
            contests::start_time,
            contests::drift,
            contests::drift_time,
            contests::teams_no,
            contests::questions_no,
            contests::active,
        ))
        .filter(contests::active.eq(true))
        .filter(filter)
        .order(contests::id.desc())
        .limit(10)
        .load::<model::ContestWithId>(&mut **db)
        .await
        .map_err(|error| anyhow!("Failed to fetch contests: {}", error))
        .attach_info(Status::InternalServerError, "")?;

    Ok(Template::render("contests", context! { contests, user }))
}

pub fn routes() -> Vec<Route> {
    routes![create_contest, create_contest_unauthorized, show_contest, contest_settings, show_contest_list,]
}
