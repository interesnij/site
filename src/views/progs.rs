//use actix::Addr;
use actix_web::{
    HttpRequest,
    HttpResponse,
    Responder,
    web,
    web::Json,
};
use crate::schema;
use crate::models::{
    CookieUser,
    Categories,
    Item,
    CookieStat,
};
use crate::diesel::{
    RunQueryDsl,
};
use serde::{Deserialize, Serialize};

use crate::utils::{
    establish_connection,
    is_signed_in,
    get_request_user_data,
    HistoryData,
    get_linguage_storage,
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
    config.route("/delete_item/{id}/", web::post().to(delete_item));
    config.route("/publish_item/{id}/", web::post().to(publish_item));
    config.route("/hide_item/{id}/", web::post().to(hide_item));
    config.route("/edit_content_item/{id}/", web::post().to(edit_content_item));

    config.route("/create_category/", web::post().to(create_category));
    config.route("/edit_category/{id}/", web::post().to(edit_category));
    config.route("/delete_category/{id}/", web::post().to(delete_category));

    config.route("/create_files/{id}/", web::post().to(create_files));
    config.route("/edit_file/{id}/", web::post().to(edit_file));
    config.route("/delete_file/{id}/", web::post().to(delete_file));
    config.route("/change_l/{id}", web::get().to(change_l));
}

pub async fn create_c_user(conn: ConnectionInfo, req: &HttpRequest) -> CookieUser {
    let device: i16;
    if crate::utils::is_desctop(&req) {
        device = 1;
    }
    else {
        device = 2;
    }

    let ipaddr: String;
    let ip = conn.realip_remote_addr();
    if ip.is_some() {
        ipaddr = ip.unwrap().to_string();
    }
    else if let Some(val) = &req.peer_addr() {
        ipaddr = val.ip().to_string();
    }
    else {
        ipaddr = String::new();
    };
    #[derive(Deserialize)] 
        pub struct UserLoc {
            pub city:    CityLoc,
            pub region:  RegionLoc,
            pub country: CountryLoc,
        }
        #[derive(Deserialize)]
        pub struct CityLoc {
            pub name_ru: String,
            pub name_en: String,
        }
        #[derive(Deserialize)]
        pub struct RegionLoc {
            pub name_ru: String,
            pub name_en: String,
        }
        #[derive(Deserialize)]
        pub struct CountryLoc {
            pub name_ru: String,
            pub name_en: String,
        }

        let _connection = establish_connection();
        let _geo_url = "http://api.sypexgeo.net/J5O6d/json/".to_string() + &ipaddr;
        let _geo_request = reqwest::get(_geo_url).await.expect("E.");
        let new_request = _geo_request.text().await.unwrap();
        //println!("request {:?}", new_request);
    
        let location200: UserLoc = serde_json::from_str(&new_request).unwrap();
        let _user = crate::models::NewCookieUser { 
            ip:         ipaddr,
            device:     device,
            city_ru:    Some(location200.city.name_ru),
            city_en:    Some(location200.city.name_en),
            region_ru:  Some(location200.region.name_ru),
            region_en:  Some(location200.region.name_en),
            country_ru: Some(location200.country.name_ru),
            country_en: Some(location200.country.name_en),
            height:     0.0,
            seconds:    0,
            created:    chrono::Local::now().naive_utc() + chrono::Duration::hours(3),
        };
        let _new_user = diesel::insert_into(schema::cookie_users::table)
            .values(&_user)
            .get_result::<crate::models::CookieUser>(&_connection)
            .expect("Error.");
    return _new_user;
}

pub async fn get_c_user(conn: ConnectionInfo, id: i32, req: &HttpRequest) -> CookieUser {
    if id > 0 {
        return crate::models::CookieUser::get(id);
    }
    return create_c_user(conn, &req).await;
}

pub async fn create_history (
    conn: ConnectionInfo,
    data: Json<HistoryData>,
    req: HttpRequest,
) -> Result<Json<CookieStat>, Error> {
    let p_id = data.user_id;
    let user = crate::views::get_c_user(conn, p_id, &req).await;
    return Ok(Json(CookieStat::create(data, user, get_linguage_storage())?));
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
}
pub async fn object_history(conn: ConnectionInfo, req: HttpRequest, id: web::Path<i32>) -> web::Json<ObjectResponse> {
    let _user = get_c_user(conn, *id, &req).await;
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
    })
}

pub async fn create_feedback(mut payload: actix_multipart::Multipart) -> impl Responder {
    let form = crate::utils::feedback_form(payload.borrow_mut()).await;
    crate::models::Feedback::create(form);
    return HttpResponse::Ok();
}


pub async fn create_item(session: Session, mut payload: Multipart) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let form = crate::utils::item_form(payload.borrow_mut(), _request_user.id).await;
            Item::create(_request_user.id, form);
        } 
    };
    HttpResponse::Ok()
}

pub async fn edit_item(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let form = crate::utils::item_form(payload.borrow_mut(), _request_user.id).await;
            Item::update_item_with_id(*_id, form); 
        }
    };
    HttpResponse::Ok()
}

pub async fn create_category(session: Session, mut payload: Multipart) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let form = crate::utils::category_form(payload.borrow_mut(), _request_user.id).await;
            Categories::create(form);
        }
    }
    return HttpResponse::Ok();
}

pub async fn edit_category(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let form = crate::utils::category_form(payload.borrow_mut(), _request_user.id).await;
        Categories::update_category_with_id(_request_user, *_id, form);
    }
    HttpResponse::Ok()
}

pub async fn edit_content_item(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    if is_signed_in(&session) { 
        let _request_user = get_request_user_data(&session);
        let form = crate::utils::content_form(payload.borrow_mut()).await;
        Item::update_content_with_id(_request_user, *_id, form);
    }
    HttpResponse::Ok().body("")
}

pub async fn delete_item(session: Session, _id: web::Path<i32>) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        Item::delete(_request_user, *_id);
    }
    HttpResponse::Ok()
}

pub async fn delete_category(session: Session, _id: web::Path<i32>) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        Categories::delete(_request_user, *_id);
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

pub async fn edit_file(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    if is_signed_in(&session) { 
        let _request_user = get_request_user_data(&session);
        let form = crate::utils::category_form(payload.borrow_mut(), _request_user.id).await;
        crate::models::File::update_file_with_id(_request_user, *_id, form);
    } 
    HttpResponse::Ok()
}

pub async fn delete_file(session: Session, _id: web::Path<i32>) -> impl Responder {
    if is_signed_in(&session) { 
        let _request_user = get_request_user_data(&session);
        crate::models::File::delete(_request_user, *_id);
    }
    HttpResponse::Ok()
}

pub async fn publish_item(session: Session, _id: web::Path<i32>) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        Item::publish(_request_user, *_id);
    }
    HttpResponse::Ok()
}
pub async fn hide_item(session: Session, _id: web::Path<i32>) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        Item::hide(_request_user, *_id);
    }
    HttpResponse::Ok()
}

pub async fn change_l(l: web::Path<u8>) -> impl Responder {
    crate::utils::set_linguage(*l);
    println!("progs set l {:?}", *l);
    HttpResponse::Ok()
} 