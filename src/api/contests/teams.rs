use diesel::dsl::max;
use diesel::{ExpressionMethods, QueryDsl};
use diesel::prelude::Queryable;
use reqwest::header;
use rocket::http::{Header, HeaderMap, Status};
use rocket_db_pools::{diesel::prelude::RunQueryDsl, Connection};
use serde::{Deserialize, Serialize};

use crate::api::ApiUser;
use crate::model::Team;
use crate::DB;
use crate::error::IntoStatusResult;
use super::{ApiError, ApiInputResult, ApiResponse};

#[derive(Queryable, Serialize)]
pub struct TeamGetResponse {
    id: i32,
    team_name: String,
    is_fake: bool,
}

#[derive(Deserialize)]
pub struct TeamPostData<'r> {
    team_name: &'r str,
}

#[derive(Serialize)]
pub struct TeamPostResponse {
    team_id: i32,
}

#[get("/contests/<id>/teams")]
pub async fn get_teams<'r>(
    id: i32,
    mut db: Connection<DB>
) -> Result<ApiResponse<'r, Vec<TeamGetResponse>>, ApiResponse<'r, ApiError>> {
    use crate::schema::{contests, teams};

    let exists: i64 = contests::dsl::contests
        .filter(contests::dsl::id.eq(id))
        .filter(contests::active.eq(true))
        .count()
        .get_result(&mut **db)
        .await
        .attach_info(Status::InternalServerError, "Errore riscontrato durante l'operazione")?;

    if exists == 0 {
        return Err(ApiResponse {
            status: Status::NotFound,
            body: ApiError { error: "".to_string(), },
            headers: HeaderMap::new(),
        });
    }

    let teams = teams::dsl::teams
        .select((teams::id, teams::team_name, teams::is_fake))
        .filter(teams::contest_id.eq(id))
        .order(teams::position.asc())
        .load::<TeamGetResponse>(&mut **db)
        .await
        .attach_info(Status::InternalServerError, "Errore riscontrato durante l'operazione")?;

    Ok(ApiResponse {
        status: Status::Ok,
        body: teams,
        headers: HeaderMap::new(),
    })
}

#[post("/contests/<id>/teams", format = "application/json", data = "<team>")]
pub async fn post_team<'r>(
    id: i32,
    team: ApiInputResult<'r, TeamPostData<'r>>,
    mut db: Connection<DB>,
    user: ApiUser,
) -> Result<ApiResponse<'r, TeamPostResponse>, ApiResponse<'r, ApiError>> {
    use crate::schema::teams;

    let Ok(team) = team else {
        return Err(ApiResponse {
            status: Status::BadRequest,
            body: ApiError { error: "Richiesta malformata".to_string() },
            headers: HeaderMap::new(),
        });
    };

    let team_no: i64 = teams::dsl::teams
        .filter(teams::contest_id.eq(id))
        .count()
        .get_result(&mut **db)
        .await
        .attach_info(Status::InternalServerError, "Errore incontrato durante la creazione della squadra")?;

    let team_id = diesel::insert_into(teams::dsl::teams)
        .values(Team {
            team_name: team.team_name.to_string(),
            contest_id: id,
            is_fake: false,
            position: team_no as i32,
            owner_id: Some(user.user_id),
        })
        .returning(teams::id)
        .get_result(&mut **db)
        .await
        .attach_info(Status::InternalServerError, "Errore incontrato durante la creazione della squadra")?;

    let mut headers = HeaderMap::new();
    headers.add(Header::new(header::LOCATION.as_str(), format!("/contest/{id}/teams/{team_id}")));

    Ok(ApiResponse {
        status: Status::Created,
        body: TeamPostResponse { team_id },
        headers,
    })
}

#[get("/contests/<id>/teams/<team_id>")]
pub async fn get_team<'r>(
    id: i32,
    team_id: i32,
    mut db: Connection<DB>
) -> Result<ApiResponse<'r, TeamGetResponse>, ApiResponse<'r, ApiError>> {
    use crate::schema::teams;

    let teams = teams::dsl::teams
        .select((teams::id, teams::team_name, teams::is_fake))
        .filter(teams::id.eq(team_id))
        .filter(teams::contest_id.eq(id))
        .order(teams::position.asc())
        .load::<TeamGetResponse>(&mut **db)
        .await
        .attach_info(Status::InternalServerError, "Errore riscontrato durante l'operazione")?;

    match teams.into_iter().next() {
        Some(team) => Ok(ApiResponse {
            status: Status::Ok,
            body: team,
            headers: HeaderMap::new(),
        }),
        None => Err(ApiResponse {
            status: Status::NotFound,
            body: ApiError { error: "".to_string() },
            headers: HeaderMap::new(),
        }),
    }
}

#[delete("/contests/<id>/teams/<team_id>")]
pub async fn delete_team<'r>(
    id: i32,
    team_id: i32,
    mut db: Connection<DB>
) -> Result<ApiResponse<'r, ()>, ApiResponse<'r, ApiError>> {
    use crate::schema::teams;

    let pos = diesel::delete(teams::dsl::teams)
        .filter(teams::id.eq(team_id))
        .filter(teams::contest_id.eq(id))
        .returning(teams::position)
        .load::<i32>(&mut **db)
        .await
        .attach_info(Status::InternalServerError, "Errore incontrato durante l'eliminazione della squadra")?;

    let Some(&pos) = pos.get(0) else {
        return Ok(ApiResponse {
            status: Status::NotFound,
            body: (),
            headers: HeaderMap::new(),
        });
    };

    let max_pos = teams::dsl::teams
        .select(max(teams::position))
        .load::<Option<i32>>(&mut **db)
        .await
        .attach_info(Status::InternalServerError, "Errore incontrato durante l'eliminazione della squadra")?;

    let resp = ApiResponse {
        status: Status::NoContent,
        body: (),
        headers: HeaderMap::new(),
    };

    let Some(Some(max_pos)) = max_pos.get(0) else {
        return Ok(resp);
    };

    if *max_pos < pos {
        return Ok(resp);
    }

    diesel::update(teams::dsl::teams)
        .filter(teams::position.eq(max_pos))
        .set(teams::position.eq(pos))
        .execute(&mut **db)
        .await
        .attach_info(Status::InternalServerError, "Errore incontrato durante l'eliminazione della squadra")?;

    Ok(resp)
}
