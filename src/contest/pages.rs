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
use crate::contest::fetch::fetch_jollies;
use crate::contest::fetch::fetch_submissions;
use crate::error::IntoStatusResult;
use crate::{model, DB};

#[get("/create")]
async fn create_contest(api_user: Option<ApiUser>) -> Result<Template, Status> {
    match api_user {
        Some(user) => Ok(Template::render("create", context! { user })),
        None => Err(Status::Unauthorized),
    }
}

#[get("/contest/<id>")]
pub async fn show_contest(id: i32, user: Option<ApiUser>, mut db: Connection<DB>) -> Result<Template, Status> {
    let Some(user) = user else {
        return Err(Status::Unauthorized)
    };

    match fetch_contest_with_ranking(&mut db, user.user_id, id)
        .await
        .attach_info(Status::InternalServerError, "")?
    {
        Some(contest) => Ok(Template::render("ranking", context! { contest, user })),
        None => Err(Status::NotFound),
    }
}

#[get("/settings/<id>")]
async fn contest_settings(id: i32, user: Option<ApiUser>, mut db: Connection<DB>) -> Result<Template, Status> {
    let Some(user) = user else {
        return Err(Status::Unauthorized)
    };

    match fetch_contest(&mut db, user.user_id, id)
        .await
        .attach_info(Status::InternalServerError, "")?
    {
        Some(contest) => Ok(Template::render("settings", context! { contest, user })),
        None => Err(Status::NotFound),
    }
}

#[get("/submissions/<id>")]
async fn contest_submissions(id: i32, user: Option<ApiUser>, mut db: Connection<DB>) -> Result<Template, Status> {
    let Some(user) = user else {
        return Err(Status::Unauthorized)
    };

    let Some(contest) = fetch_contest(&mut db, user.user_id, id)
        .await
        .attach_info(Status::InternalServerError, "")?
    else {
        return Err(Status::NotFound);
    };

    let Some(subs) = fetch_submissions(&mut db, user.user_id, id)
        .await
        .attach_info(Status::InternalServerError, "")?
    else {
        return Err(Status::NotFound);
    };

    let Some(jollies) = fetch_jollies(&mut db, user.user_id, id)
        .await
        .attach_info(Status::InternalServerError, "")?
    else {
        return Err(Status::NotFound);
    };

    Ok(Template::render("submissions", context! { contest, subs, jollies, user }))
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
            contests::jolly_time,
            contests::teams_no,
            contests::questions_no,
            contests::active,
        ))
        .filter(contests::active.eq(true))
        .filter(filter)
        .order(contests::id.desc())
        .load::<model::ContestWithId>(&mut **db)
        .await
        .map_err(|error| anyhow!("Failed to fetch contests: {}", error))
        .attach_info(Status::InternalServerError, "")?;

    Ok(Template::render("contests", context! { contests, user }))
}

pub fn routes() -> Vec<Route> {
    routes![create_contest, show_contest, contest_settings, show_contest_list, contest_submissions]
}
