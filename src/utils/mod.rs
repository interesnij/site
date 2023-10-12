mod forms;
mod auth;
mod stat;

pub use self::{
    forms::*,
    auth::*,
    stat::*,
};
use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    error::InternalError,
    http::StatusCode,
    dev::ConnectionInfo,
};
use crate::schema;
use serde::{Deserialize, Serialize};
use crate::models::{
    Categories,
    User,
    Cat,
    CookieUser,
};
use crate::diesel::{
    Connection,
    PgConnection,
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
};
use actix_session::Session;
use crate::errors::AuthError;
use sailfish::TemplateOnce;
use std::cell::Cell;
use std::sync::{Arc, Mutex};


pub struct AppState {
    pub server_id: usize,
    pub request_count: Cell<usize>,
    pub messages: Arc<Mutex<Vec<String>>>,
}
#[derive(Serialize)]
pub struct IndexResponse {
    pub server_id: usize,
    pub request_count: usize,
    pub messages: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct NewUserForm {
    pub username: String,
    pub email:    String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct HistoryData {
    pub user_id:   i32, 
    pub object_id: i32,
    pub page_id:   i16,
    pub link:      String,
    pub title:     String,
    pub height:    f64,
    pub seconds:   i32,
    pub template:  String,
}

pub fn get_price_acc_values(price: &i32) -> Option<i32> {
    if price > &3_000_000 {
        let acc = (price * 10) / 100; // 10% скидка
        return Some(acc);
    }
    else if price > &2_000_000 && price < &3_000_000 {
        let acc = (price * 7) / 100; // 10% скидка
        return Some(acc);
    }
    else if price > &1_000_000 && price < &2_000_000 {
        let acc = (price * 5) / 100; // 5% скидка
        return Some(acc);
    }
    else {
        return None;
    }
}

pub fn establish_connection() -> PgConnection {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn get_content_type<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    return req.headers().get("user-agent")?.to_str().ok();
}
pub fn is_desctop(req: &HttpRequest) -> bool {
    return !get_content_type(req).unwrap().contains("Mobile");
} 

    pub fn get_device_and_ajax(req: &HttpRequest) -> (bool, i32) {
        #[derive(Debug, Deserialize)]
        struct Params {
            pub ajax: Option<i32>,
        }
        let params_some = web::Query::<Params>::from_query(&req.query_string());
        let mut is_ajax = 0;
        let _type = true;

        if params_some.is_ok() {
            let params = params_some.unwrap();
            if params.ajax.is_some() {
                is_ajax = params.ajax.unwrap();
            }
            else {
                is_ajax = 0;
            }
        }

        (is_desctop(req), is_ajax)
    }

    pub fn get_categories_2(l: i16) -> (
        Vec<Cat>,
        Vec<Cat>,
        Vec<Cat>,
        Vec<Cat>,
        Vec<Cat>,
        Vec<Cat>, 
    ) { 
        let _service_cats = Categories::get_categories_for_types(2, l);
        let _store_cats = Categories::get_categories_for_types(3, l);
        let _blog_cats = Categories::get_categories_for_types(1, l);
        let _wiki_cats = Categories::get_categories_for_types(4, l);
        let _work_cats = Categories::get_categories_for_types(5, l);
        let _help_cats = Categories::get_categories_for_types(6, l);

        return (
            _service_cats,
            _store_cats,
            _blog_cats,
            _wiki_cats,
            _work_cats,
            _help_cats
        );
    }
//}

pub fn get_page(req: &HttpRequest) -> i32 {
    #[derive(Debug, Deserialize)]
    struct Params {
        pub page: Option<i32>,
    }
    let params_some = web::Query::<Params>::from_query(&req.query_string());
    let page: i32;
    if params_some.is_ok() {
        let params = params_some.unwrap();
        if params.page.is_some() {
            page = params.page.unwrap();
        }
        else {
            page = 1;
        }
    }
    else {
        page = 1;
    }
    page
}


pub fn get_request_user_data(session: &Session) -> User {
    use crate::models::SessionUser;
    use crate::schema::users::dsl::users;

    let _connection = establish_connection();
    let mut user_id = 0;
    if let Some(user_str) = session.get::<String>("user")
        .map_err(|_| AuthError::AuthenticationError(String::from("Не удалось извлечь пользователя из сеанса")))
        .unwrap() {
            let user: SessionUser = serde_json::from_str(&user_str).expect("E.");
            user_id = user.id;
        }
    if user_id != 0 {
        users
            .filter(schema::users::id.eq(user_id))
            .first::<User>(&_connection)
            .expect("E")
    } else {
        users
            .filter(schema::users::id.eq(1))
            .first::<User>(&_connection)
            .expect("E")
    }
}

pub async fn get_first_load_page (
    session:     &Session,
    is_desctop:  bool,
    title:       &String,
    description: &String,
    uri:         &String,
    image:       &String,
    t:           i16,
) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if is_desctop {
            #[derive(TemplateOnce)] 
            #[template(path = "desctop/generic/first_load.stpl")]
            struct Template<'a> {
                request_user:   User,
                title:          &'a String,
                description:    &'a String,
                image:          &'a String,
                uri:            &'a String,
                template_types: i16,
            }
            let body = Template {
                request_user:   _request_user,
                title:          title,
                description:    description,
                image:          image,
                uri:            uri,
                template_types: t,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/generic/first_load.stpl")]
            struct Template<'a> {
                request_user:   User,
                title:          &'a String,
                description:    &'a String,
                image:          &'a String,
                uri:            &'a String,
                template_types: i16,
            }
            let body = Template {
                request_user:   _request_user,
                title:          title,
                description:    description,
                image:          image,
                uri:            uri,
                template_types: t,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/generic/anon_first_load.stpl")]
            struct Template<'a> {
                title:          &'a String,
                description:    &'a String,
                image:          &'a String,
                uri:            &'a String,
                template_types: i16,
            }
            let body = Template {
                title:          title,
                description:    description,
                image:          image,
                uri:            uri,
                template_types: t,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/generic/anon_first_load.stpl")]
            struct Template<'a> {
                title:          &'a String,
                description:    &'a String,
                image:          &'a String,
                uri:            &'a String,
                template_types: i16,
            }
            let body = Template {
                title:          title,
                description:    description,
                image:          image,
                uri:            uri,
                template_types: t,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn get_private_page (
    is_ajax:     i32,
    user:        User,
    is_desctop:  bool,
    title:       &String,
    description: &String,
    link:        &String,
    image:       &String,
    t:           i16,
    l:           i16,
) -> actix_web::Result<HttpResponse> {
    if is_desctop {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/generic/private_object.stpl")]
        struct Template<'a> {
            is_ajax:        i32,
            request_user:   User,
            title:          &'a String,
            description:    &'a String,
            image:          &'a String,
            link:           &'a String,
            template_types: i16,
            linguage:       i16,
        }
        let body = Template {
            is_ajax:        is_ajax,
            request_user:   user,
            title:          title,
            description:    description,
            image:          image,
            link:           link,
            template_types: t,
            linguage:       l,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/generic/private_object.stpl")]
        struct Template<'a> {
            is_ajax:        i32,
            title:          &'a String,
            description:    &'a String,
            image:          &'a String,
            link:           &'a String,
            template_types: i16,
            linguage:       i16,
        }
        let body = Template {
            is_ajax:        is_ajax,
            title:          title,
            description:    description,
            image:          image,
            link:           link,
            template_types: t,
            linguage:       l,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub async fn get_anon_private_page (
    is_ajax:     i32,
    is_desctop:  bool,
    title:       &String,
    description: &String,
    link:        &String,
    image:       &String,
    t:           i16,
    l:           i16,
) -> actix_web::Result<HttpResponse> {
    if is_desctop {
        #[derive(TemplateOnce)]
        #[template(path = "desctop/generic/anon_private_object.stpl")]
        struct Template<'a> {
            is_ajax:        i32,
            title:          &'a String,
            description:    &'a String,
            image:          &'a String,
            link:           &'a String,
            template_types: i16,
            linguage:       i16,
        }
        let body = Template {
            is_ajax:        is_ajax,
            title:          title,
            description:    description,
            image:          image,
            link:           link,
            template_types: t,
            linguage:       l,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        #[derive(TemplateOnce)]
        #[template(path = "mobile/generic/anon_private_object.stpl")]
        struct Template<'a> {
            is_ajax:        i32,
            title:          &'a String,
            description:    &'a String,
            image:          &'a String,
            link:           &'a String,
            template_types: i16,
            linguage:       i16,
        }
        let body = Template {
            is_ajax:        is_ajax,
            title:          title,
            description:    description,
            image:          image,
            link:           link,
            template_types: t,
            linguage:       l,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub fn get_count_for_ru(count: i16, word1: String, word2: String, word3: String) -> String {
    let a = count % 10;
    let b = count % 100;
    let count_str: String = count.to_string().parse().unwrap();
    if a == 1 && b != 11 {
        return count_str + &word1;
    }
    else if a >= 2 && a <= 4 && (b < 10 || b >= 20) {
        return count_str + &word2;
    }
    else {
        return count_str + &word3;
    }
}

pub async fn get_or_create_cookie_user_id(conn: &ConnectionInfo, req: &HttpRequest) -> i32 {
    let user_id = get_cookie_user_id(&req);
    if user_id != 0 {
        let user = get_or_create_c_user_return_object(conn.clone(), &req).await;
        return user.id;
    }
    else {
        let user = create_c_user_return_object(conn.clone(), &req).await;
        return user.id;
    }
}

async fn create_c_user_return_object(conn: ConnectionInfo, req: &HttpRequest) -> CookieUser {
    let device: i16;
    if is_desctop(&req) {
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
            pub iso:     String,
        }

        let _connection = establish_connection();
        let _geo_url = "http://api.sypexgeo.net/J5O6d/json/".to_string() + &ipaddr;
        let _geo_request = reqwest::get(_geo_url).await.expect("E.");
        let new_request = _geo_request.text().await.unwrap();
        //println!("request {:?}", new_request);
    
        let location200: UserLoc = serde_json::from_str(&new_request).unwrap();
        let linguage: i16;
        if location200.country.iso == "Ru".to_string() {
            linguage = 1;
        }
        else {
            linguage = 2;
        }
        let _user = crate::models::NewCookieUser { 
            ip:         ipaddr,
            device:     device,
            linguage:   linguage,
            template:   1,
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
            .get_result::<CookieUser>(&_connection)
            .expect("Error.");
    return _new_user;
}

async fn create_c_user_return_lt(conn: ConnectionInfo, req: &HttpRequest) -> (i16, i16) {
    let device: i16;
    if is_desctop(&req) {
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
            pub iso:     String,
        }

        let _connection = establish_connection();
        let _geo_url = "http://api.sypexgeo.net/J5O6d/json/".to_string() + &ipaddr;
        let _geo_request = reqwest::get(_geo_url).await.expect("E.");
        let new_request = _geo_request.text().await.unwrap();
        //println!("request {:?}", new_request);
    
        let location200: UserLoc = serde_json::from_str(&new_request).unwrap();
        let linguage: i16;
        if location200.country.iso == "Ru".to_string() {
            linguage = 1;
        }
        else {
            linguage = 2;
        }
        let _user = crate::models::NewCookieUser { 
            ip:         ipaddr,
            device:     device,
            linguage:   linguage,
            template:   1,
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
            .execute(&_connection)
            .expect("Error.");
    return (linguage, 1);
}

async fn create_c_user_return_lti(conn: ConnectionInfo, req: &HttpRequest) -> (i16, i16, i32) {
    let device: i16;
    if is_desctop(&req) {
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
            pub iso:     String,
        }

        let _connection = establish_connection();
        let _geo_url = "http://api.sypexgeo.net/J5O6d/json/".to_string() + &ipaddr;
        let _geo_request = reqwest::get(_geo_url).await.expect("E.");
        let new_request = _geo_request.text().await.unwrap();
        //println!("request {:?}", new_request);
    
        let location200: UserLoc = serde_json::from_str(&new_request).unwrap();
        let linguage: i16;
        if location200.country.iso == "Ru".to_string() {
            linguage = 1;
        }
        else {
            linguage = 2;
        }
        let _user = crate::models::NewCookieUser { 
            ip:         ipaddr,
            device:     device,
            linguage:   linguage,
            template:   1,
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
            .get_result::<CookieUser>(&_connection)
            .expect("Error.");
    return (linguage, 1, _new_user.id);
}

pub async fn get_or_create_c_user_with_id_return_object(id: i32, conn: ConnectionInfo, req: &HttpRequest) -> CookieUser {
    if id > 0 { 
        let res = CookieUser::get_res(id);
        if res.is_ok() {
            return res.expect("E.");
        }
        else {
            return create_c_user_return_object(conn, req).await;
        }
    } 
    return create_c_user_return_object(conn, &req).await;
}
pub async fn get_or_create_c_user_return_object(conn: ConnectionInfo, req: &HttpRequest) -> CookieUser {
    let res = CookieUser::get_res(get_cookie_user_id(req));
    if res.is_ok() {
        return res.expect("E.");
    }
    else {
        return create_c_user_return_object(conn, &req).await;
    }
}
pub async fn get_or_create_c_user_return_lt(conn: ConnectionInfo, req: &HttpRequest) -> (i16, i16) {
    let res = CookieUser::get_res_lt(get_cookie_user_id(req));
    if res.is_ok() {
        return res.expect("E.");
    }
    else {
        return create_c_user_return_lt(conn, req).await;
    }
}
pub async fn get_or_create_c_user_return_lti(conn: ConnectionInfo, req: &HttpRequest) -> (i16, i16, i32) {
    let res = CookieUser::get_res_lti(get_cookie_user_id(req));
    if res.is_ok() {
        return res.expect("E.");
    }
    else {
        return create_c_user_return_lti(conn, req).await;
    }
}

pub fn get_cookie_user_id(req: &HttpRequest) -> i32 {
    let _cookie = req.headers().get("cookie").expect("E.").to_str().ok();
    let mut user_id = 0;
    if _cookie.is_some() {
        for c in _cookie.unwrap().split("; ").collect::<Vec<&str>>().iter() {
            let split_c: Vec<&str> = c.split("=").collect();
            if split_c[0] == "user" {
                user_id = split_c[1].parse().unwrap();
            }
        }
    }
    user_id
}  
 
pub fn get_c_user_lt(req: &HttpRequest) -> (i16, i16) {
    return CookieUser::get_res_lt(get_cookie_user_id(req)).expect("E.");
} 
pub fn get_c_user_l(req: &HttpRequest) -> i16 {
    return CookieUser::get_res_l(get_cookie_user_id(req)).expect("E.");
} 