use actix_web::http::header;
use actix_web::{web, error, Error, HttpRequest, HttpResponse};
use tera::{Context};
use crate::TERA;

use crate::prelude::*;

pub async fn create_post<T: ActixAdminAppDataTrait, E: ActixAdminViewModelTrait>(
    _req: HttpRequest,
    data: web::Data<T>,
    text: String,
) -> Result<HttpResponse, Error> {
    let db = &data.get_db();
    let entity_name = E::get_entity_name();
    let entity_names = &data.get_actix_admin().entity_names;
    let actix_admin = data.get_actix_admin();
    let view_model = actix_admin.view_models.get(&entity_name).unwrap();
    let mut model = ActixAdminModel::from(text);
    model = E::create_entity(db, model).await;

    if model.has_errors() {
        let mut ctx = Context::new();
        ctx.insert("entity_names", &entity_names);
        ctx.insert("view_model", &view_model);
        ctx.insert("select_lists", &E::get_select_lists(db).await);
        ctx.insert("list_link", &E::get_list_link(&entity_name));
        ctx.insert("model", &model);

        let body = TERA
            .render("create_or_edit.html", &ctx)
            .map_err(|err| error::ErrorInternalServerError(err))?;
        Ok(HttpResponse::Ok().content_type("text/html").body(body))
    }
    else {
        Ok(HttpResponse::Found()
        .append_header((
            header::LOCATION,
            format!("/admin/{}/list", view_model.entity_name),
        ))
        .finish())
    }
}