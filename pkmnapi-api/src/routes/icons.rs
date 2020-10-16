use pkmnapi_sql::*;
use rocket::http::{ContentType, Header};
use rocket::response::Response;
use rocket::State;
use std::io::Cursor;

use crate::guards::*;
use crate::responses::errors::*;
use crate::utils;

#[get("/icons/<icon_id>", format = "image/gif")]
pub fn get_icon<'a>(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    icon_id: u8,
) -> Result<Response<'a>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let icon = db.get_icon(&icon_id)?;
    let gif = icon.to_gif(26)?;

    let response = Response::build()
        .header(ContentType::GIF)
        .header(Header::new(
            "Content-Disposition",
            format!(r#"attachment; filename="icon-{}.gif""#, icon_id),
        ))
        .sized_body(Cursor::new(gif))
        .finalize();

    Ok(response)
}
