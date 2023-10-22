//use actix::Addr;
use actix_web::{
    HttpRequest,
    HttpResponse,
    Responder,
    web,
    web::Json,
};
use crate::models::{
    Categories,
    Item,
    CookieStat,
};
use serde::{Deserialize, Serialize};

use crate::utils::{
    is_signed_in,
    get_request_user_data,
    HistoryData,
}; 
use actix_session::Session;
use actix_multipart::Multipart;
use std::str;
use std::borrow::BorrowMut;
use actix_web::dev::ConnectionInfo;
use crate::errors::Error;
use crate::websocket::{
    //MessageToClient, 
    //Server, 
    ws_index
};


pub fn progs_routes(config: &mut web::ServiceConfig) { 
    config.route("/ws", web::get().to(ws_index));
    config.route("/create_history/", web::post().to(create_history));
    config.route("/object_history/{id}/", web::get().to(object_history));
    config.route("/feedback/", web::post().to(create_feedback));

    config.route("/create_item/", web::post().to(create_item));
    config.route("/edit_item/{id}/", web::post().to(edit_item));
    config.route("/delete_item/", web::post().to(delete_item));
    config.route("/publish_item/", web::post().to(publish_item));
    config.route("/hide_item/", web::post().to(hide_item));
    config.route("/edit_content_item/{id}/", web::post().to(edit_content_item));

    config.route("/create_category/", web::post().to(create_category));
    config.route("/edit_category/{id}/", web::post().to(edit_category));
    config.route("/delete_category/", web::post().to(delete_category));

    config.route("/create_files/{id}/", web::post().to(create_files));
    config.route("/edit_file/{id}/", web::post().to(edit_file));
    config.route("/delete_file/", web::post().to(delete_file));
    config.route("/change_l/", web::post().to(change_l));
    config.route("/change_t/", web::post().to(change_t));
    config.route("/change_c/", web::post().to(change_c));
    config.route("/update_money_rate/", web::get().to(update_money_rate));
}

pub async fn create_history (
    conn: ConnectionInfo,
    data: Json<HistoryData>,
    req: HttpRequest,
) -> Result<Json<CookieStat>, Error> {
    let user = crate::utils::get_or_create_c_user_return_object(conn, &req).await;
    return Ok(Json(CookieStat::create(data, user)?));
} 

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectResponse {
    pub id:         i32,
    pub ip:         String,
    pub device:     i16,
    pub city_ru:    Option<String>,
    pub city_en:    Option<String>,
    pub region_ru:  Option<String>,
    pub region_en:  Option<String>,
    pub country_ru: Option<String>,
    pub country_en: Option<String>,
    pub linguage:   i16,
    pub currency:   String,
    pub template:   i16,
}
pub async fn object_history(conn: ConnectionInfo, req: HttpRequest, id: web::Path<i32>) -> web::Json<ObjectResponse> {
    let _user = crate::utils::get_or_create_c_user_with_id_return_object(*id, conn, &req).await;
    return web::Json( ObjectResponse { 
        id:         _user.id,
        ip:         _user.ip,
        device:     _user.device,
        city_ru:    _user.city_ru,
        city_en:    _user.city_en,
        region_ru:  _user.region_ru,
        region_en:  _user.region_en,
        country_ru: _user.country_ru,
        country_en: _user.country_en,
        linguage:   _user.linguage,
        currency:   _user.currency,
        template:   _user.template,
    })
}

pub async fn create_feedback(mut payload: actix_multipart::Multipart) -> impl Responder {
    let form = crate::utils::feedback_form(payload.borrow_mut()).await;
    crate::models::Feedback::create(form);
    return HttpResponse::Ok();
}


pub async fn create_item(req: HttpRequest, session: Session, mut payload: Multipart) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let form = crate::utils::item_form(payload.borrow_mut(), _request_user.id).await;
            let l = crate::utils::get_c_user_l(&req);
            Item::create(_request_user.id, form, l);
        } 
    };
    HttpResponse::Ok()
}

pub async fn edit_item(req: HttpRequest, session: Session, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let form = crate::utils::item_form(payload.borrow_mut(), _request_user.id).await;
            let l = crate::utils::get_c_user_l(&req);
            Item::update_item_with_id(*_id, form, l); 
        }
    };
    HttpResponse::Ok()
}

pub async fn create_category(req: HttpRequest, session: Session, mut payload: Multipart) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let form = crate::utils::category_form(payload.borrow_mut(), _request_user.id).await;
            let l = crate::utils::get_c_user_l(&req);
            Categories::create(form, l);
        }
    }
    return HttpResponse::Ok();
}

pub async fn edit_category(req: HttpRequest, session: Session, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let form = crate::utils::category_form(payload.borrow_mut(), _request_user.id).await;
        let l = crate::utils::get_c_user_l(&req);
        Categories::update_category_with_id(_request_user, *_id, form, l);
    }
    HttpResponse::Ok()
}

pub async fn edit_content_item(req: HttpRequest, session: Session, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    if is_signed_in(&session) { 
        let _request_user = get_request_user_data(&session);
        let form = crate::utils::content_form(payload.borrow_mut()).await;
        let l = crate::utils::get_c_user_l(&req);
        Item::update_content_with_id(_request_user, *_id, form, l);
    }
    HttpResponse::Ok().body("")
}

pub async fn delete_item(session: Session, mut payload: Multipart) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let form = crate::utils::id_form(payload.borrow_mut()).await;
        Item::delete(_request_user, form.id);
    }
    HttpResponse::Ok()
}

pub async fn delete_category(session: Session, mut payload: Multipart) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let form = crate::utils::id_form(payload.borrow_mut()).await;
        Categories::delete(_request_user, form.id);
    }
    HttpResponse::Ok()
} 

pub async fn create_files(session: Session, mut payload: Multipart, id: web::Path<i32>) -> impl Responder {
    if is_signed_in(&session) { 
        let _request_user = get_request_user_data(&session);
        let form = crate::utils::files_form(payload.borrow_mut(), _request_user.id).await;
        crate::models::File::create(_request_user, *id, form);
    }
    HttpResponse::Ok()
}

pub async fn edit_file(req: HttpRequest, session: Session, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    if is_signed_in(&session) { 
        let _request_user = get_request_user_data(&session);
        let l = crate::utils::get_c_user_l(&req);
        let form = crate::utils::category_form(payload.borrow_mut(), _request_user.id).await;
        crate::models::File::update_file_with_id(_request_user, *_id, form, l);
    } 
    HttpResponse::Ok()
}

pub async fn delete_file(session: Session, mut payload: Multipart) -> impl Responder {
    if is_signed_in(&session) {  
        let _request_user = get_request_user_data(&session);
        let form = crate::utils::id_form(payload.borrow_mut()).await;
        crate::models::File::delete(_request_user, form.id);
    }
    HttpResponse::Ok()
}

pub async fn publish_item(session: Session, mut payload: Multipart) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let form = crate::utils::id_form(payload.borrow_mut()).await;
        Item::publish(_request_user, form.id);
    }
    HttpResponse::Ok() 
}
pub async fn hide_item(session: Session, mut payload: Multipart) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let form = crate::utils::id_form(payload.borrow_mut()).await;
        Item::hide(_request_user, form.id);
    }
    HttpResponse::Ok()
}

pub async fn change_l(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let form = crate::utils::types_form(payload.borrow_mut()).await;
    let user_id = crate::utils::get_cookie_user_id(&req);
    crate::models::CookieUser::update_l(user_id, form.types);
    HttpResponse::Ok()
}
pub async fn change_t(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let form = crate::utils::types_form(payload.borrow_mut()).await;
    let user_id = crate::utils::get_cookie_user_id(&req);
    crate::models::CookieUser::update_t(user_id, form.types);
    HttpResponse::Ok()
}
pub async fn change_c(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let form = crate::utils::string_form(payload.borrow_mut()).await;
    let user_id = crate::utils::get_cookie_user_id(&req);
    crate::models::CookieUser::update_c(user_id, form.string);
    HttpResponse::Ok()
}

#[derive(Debug, Deserialize)] 
pub struct RateData {
    pub rates: Rates,
}  

#[derive(Debug, Deserialize)] 
//#[serde(rename_all = "camelCase")]
pub struct Rates {
    pub USD: f64,
    pub EUR: f64,
    pub GBP: f64,
    pub BYN: f64,
    pub GEL: f64,
    pub JPY: f64,
    pub CHF: f64,
    pub TRY: f64,
    pub PLN: f64,
    pub CNY: f64,
    pub CAD: f64,
    pub KZT: f64,
    pub INR: f64,
}

pub async fn update_money_rate() -> impl Responder {
    let _request = reqwest::get("https://www.cbr-xml-daily.ru/latest.js").await.expect("E.");
    let new_request = _request.text().await.unwrap();
    //println!("request {:?}", new_request);
    let request200: RateData = serde_json::from_str(&new_request).unwrap();
    let rates = request200.rates;
    println!("USD {:?}", round(rates.USD, 2));
    println!("EUR {:?}", round(rates.EUR, 2));
    web_local_storage_api::set_item("USD", rates.USD as &str);
    web_local_storage_api::set_item("EUR", rates.EUR as &str);

    HttpResponse::Ok()
}