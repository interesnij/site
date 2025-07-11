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
    pub linguage: i16,
    pub template: i16,
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
    c:           String,
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
            currency:       String,
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
            currency:       c,
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
            currency:       String,
        }
        let body = Template {
            is_ajax:        is_ajax,
            title:          title,
            description:    description,
            image:          image,
            link:           link,
            template_types: t,
            linguage:       l,
            currency:       c,
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
    c:           String,
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
            currency:       String,
        }
        let body = Template {
            is_ajax:        is_ajax,
            title:          title,
            description:    description,
            image:          image,
            link:           link,
            template_types: t,
            linguage:       l,
            currency:       c,
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
            currency:       String,
        }
        let body = Template {
            is_ajax:        is_ajax,
            title:          title,
            description:    description,
            image:          image,
            link:           link,
            template_types: t,
            linguage:       l,
            currency:       c,
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
            pub name_ru:   String,
            pub name_en:   String,
            pub iso:       String,
            pub continent: String,
        }

        let _connection = establish_connection();
        let _geo_url = "http://api.sypexgeo.net/J5O6d/json/".to_string() + &ipaddr;
        let _geo_request = reqwest::get(_geo_url).await.expect("E.");
        let new_request = _geo_request.text().await.unwrap();
        //println!("request {:?}", new_request);
    
        let location200: UserLoc = serde_json::from_str(&new_request).unwrap();
        let linguage: i16;
        let mut currency: String;

        if location200.country.continent == "EU".to_string() {
            currency = "EUR".to_string();
        }
        else {
            currency = "USD".to_string();
        }

        if location200.country.iso == "RU".to_string() {
            linguage = 1;
            currency = "RUB".to_string();
        }
        else {
            linguage = 2;
        }
        let _user = crate::models::NewCookieUser { 
            ip:         ipaddr,
            device:     device,
            linguage:   linguage,
            template:   1,
            currency:   currency,
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

async fn create_c_user_return_ltc(conn: ConnectionInfo, req: &HttpRequest) -> (i16, i16, String) {
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
            pub name_ru:   String,
            pub name_en:   String,
            pub iso:       String,
            pub continent: String,
        }

        let _connection = establish_connection();
        let _geo_url = "http://api.sypexgeo.net/J5O6d/json/".to_string() + &ipaddr;
        let _geo_request = reqwest::get(_geo_url).await.expect("E.");
        let new_request = _geo_request.text().await.unwrap();
        //println!("request {:?}", new_request);
    
        let location200: UserLoc = serde_json::from_str(&new_request).unwrap();
        let linguage: i16;
        let mut currency: String;

        if location200.country.continent == "EU".to_string() {
            currency = "EUR".to_string();
        }
        else {
            currency = "USD".to_string();
        }

        if location200.country.iso == "RU".to_string() {
            linguage = 1;
            currency = "RUB".to_string();
        }
        else {
            linguage = 2;
        }
        let _user = crate::models::NewCookieUser { 
            ip:         ipaddr,
            device:     device,
            linguage:   linguage,
            template:   1,
            currency:   currency.clone(),
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
    return (linguage, 1, currency);
}

async fn create_c_user_return_ltic(conn: ConnectionInfo, req: &HttpRequest) -> (i16, i16, i32, String) {
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
            pub name_ru:   String,
            pub name_en:   String,
            pub iso:       String,
            pub continent: String,
        }

        let _connection = establish_connection();
        let _geo_url = "http://api.sypexgeo.net/J5O6d/json/".to_string() + &ipaddr;
        let _geo_request = reqwest::get(_geo_url).await.expect("E.");
        let new_request = _geo_request.text().await.unwrap();
        //println!("request {:?}", new_request);
    
        let location200: UserLoc = serde_json::from_str(&new_request).unwrap();
        let linguage: i16;
        let mut currency: String;

        if location200.country.continent == "EU".to_string() {
            currency = "EUR".to_string();
        }
        else {
            currency = "USD".to_string();
        }

        if location200.country.iso == "RU".to_string() {
            linguage = 1;
            currency = "RUB".to_string();
        }
        else {
            linguage = 2;
        }

        let _user = crate::models::NewCookieUser { 
            ip:         ipaddr,
            device:     device,
            linguage:   linguage,
            template:   1,
            currency:   currency.clone(),
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
    return (linguage, 1, _new_user.id, currency);
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
pub async fn get_or_create_c_user_return_ltc(conn: ConnectionInfo, req: &HttpRequest) -> (i16, i16, String) {
    let res = CookieUser::get_res_ltc(get_cookie_user_id(req));
    if res.is_ok() {
        return res.expect("E.");
    } 
    else {
        return create_c_user_return_ltc(conn, req).await;
    }
}
pub async fn get_or_create_c_user_return_ltic(conn: ConnectionInfo, req: &HttpRequest) -> (i16, i16, i32, String) {
    let res = CookieUser::get_res_ltic(get_cookie_user_id(req));
    if res.is_ok() {
        return res.expect("E.");
    }
    else {
        return create_c_user_return_ltic(conn, req).await;
    }
}

pub fn get_cookie_user_id(req: &HttpRequest) -> i32 {
    let mut user_id = 0;
    let _cookie_res = req.headers().get("cookie");

    if _cookie_res.is_none() {
        return user_id;
    }
    let _cookie = _cookie_res.expect("E.").to_str().ok();
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
 
pub fn get_c_user_ltc(req: &HttpRequest) -> (i16, i16, String) {
    return CookieUser::get_res_ltc(get_cookie_user_id(req)).expect("E.");
}
pub fn get_c_user_l(req: &HttpRequest) -> i16 {
    return CookieUser::get_res_l(get_cookie_user_id(req)).expect("E.");
}


pub fn check_last_currencies() -> () {
    use crate::models::PriceCorrect;

    let usd_ratio = web_local_storage_api::get_item("usd_ratio").is_ok();
    let usd_adder = web_local_storage_api::get_item("usd_adder").is_ok();
    if !usd_ratio || !usd_adder {
        let (ratio, adder) = PriceCorrect::get_info_with_currency("USD".to_string());
        let _a = web_local_storage_api::set_item("usd_ratio", &ratio.to_string());
        let _a = web_local_storage_api::set_item("usd_adder", &adder.to_string());
    }

    let eur_ratio = web_local_storage_api::get_item("eur_ratio").is_ok();
    let eur_adder = web_local_storage_api::get_item("eur_adder").is_ok();
    if !eur_ratio || !eur_adder {
        let (ratio, adder) = PriceCorrect::get_info_with_currency("EUR".to_string());
        let _a = web_local_storage_api::set_item("eur_ratio", &ratio.to_string());
        let _a = web_local_storage_api::set_item("eur_adder", &adder.to_string());
    }

    let gbr_ratio = web_local_storage_api::get_item("gbr_ratio").is_ok();
    let gbr_adder = web_local_storage_api::get_item("gbr_adder").is_ok();
    if !gbr_ratio || !gbr_adder {
        let (ratio, adder) = PriceCorrect::get_info_with_currency("GBR".to_string());
        let _a = web_local_storage_api::set_item("gbr_ratio", &ratio.to_string());
        let _a = web_local_storage_api::set_item("gbr_adder", &adder.to_string());
    }

    let byn_ratio = web_local_storage_api::get_item("byn_ratio").is_ok();
    let byn_adder = web_local_storage_api::get_item("byn_adder").is_ok();
    if !byn_ratio || !byn_adder {
        let (ratio, adder) = PriceCorrect::get_info_with_currency("BYN".to_string());
        let _a = web_local_storage_api::set_item("byn_ratio", &ratio.to_string());
        let _a = web_local_storage_api::set_item("byn_adder", &adder.to_string());
    }

    let gel_ratio = web_local_storage_api::get_item("gel_ratio").is_ok();
    let gel_adder = web_local_storage_api::get_item("gel_adder").is_ok();
    if !gel_ratio || !gel_adder {
        let (ratio, adder) = PriceCorrect::get_info_with_currency("GEL".to_string());
        let _a = web_local_storage_api::set_item("gel_ratio", &ratio.to_string());
        let _a = web_local_storage_api::set_item("gel_adder", &adder.to_string());
    }

    let jpy_ratio = web_local_storage_api::get_item("jpy_ratio").is_ok();
    let jpy_adder = web_local_storage_api::get_item("jpy_adder").is_ok();
    if !jpy_ratio || !jpy_adder {
        let (ratio, adder) = PriceCorrect::get_info_with_currency("JPY".to_string());
        let _a = web_local_storage_api::set_item("jpy_ratio", &ratio.to_string());
        let _a = web_local_storage_api::set_item("jpy_adder", &adder.to_string());
    }

    let chf_ratio = web_local_storage_api::get_item("chf_ratio").is_ok();
    let chf_adder = web_local_storage_api::get_item("chf_adder").is_ok();
    if !chf_ratio || !chf_adder {
        let (ratio, adder) = PriceCorrect::get_info_with_currency("CHF".to_string());
        let _a = web_local_storage_api::set_item("chf_ratio", &ratio.to_string());
        let _a = web_local_storage_api::set_item("chf_adder", &adder.to_string());
    }

    let try_ratio = web_local_storage_api::get_item("try_ratio").is_ok();
    let try_adder = web_local_storage_api::get_item("try_adder").is_ok();
    if !try_ratio || !try_adder {
        let (ratio, adder) = PriceCorrect::get_info_with_currency("TRY".to_string());
        let _a = web_local_storage_api::set_item("try_ratio", &ratio.to_string());
        let _a = web_local_storage_api::set_item("try_adder", &adder.to_string());
    }

    let pln_ratio = web_local_storage_api::get_item("pln_ratio").is_ok();
    let pln_adder = web_local_storage_api::get_item("pln_adder").is_ok();
    if !pln_ratio || !pln_adder {
        let (ratio, adder) = PriceCorrect::get_info_with_currency("PLN".to_string());
        let _a = web_local_storage_api::set_item("pln_ratio", &ratio.to_string());
        let _a = web_local_storage_api::set_item("pln_adder", &adder.to_string());
    }

    let cny_ratio = web_local_storage_api::get_item("cny_ratio").is_ok();
    let cny_adder = web_local_storage_api::get_item("cny_adder").is_ok();
    if !cny_ratio || !cny_adder {
        let (ratio, adder) = PriceCorrect::get_info_with_currency("CNY".to_string());
        let _a = web_local_storage_api::set_item("cny_ratio", &ratio.to_string());
        let _a = web_local_storage_api::set_item("cny_adder", &adder.to_string());
    }

    let cad_ratio = web_local_storage_api::get_item("cad_ratio").is_ok();
    let cad_adder = web_local_storage_api::get_item("cad_adder").is_ok();
    if !cad_ratio || !cad_adder {
        let (ratio, adder) = PriceCorrect::get_info_with_currency("CAD".to_string());
        let _a = web_local_storage_api::set_item("cad_ratio", &ratio.to_string());
        let _a = web_local_storage_api::set_item("cad_adder", &adder.to_string());
    }

    let kzt_ratio = web_local_storage_api::get_item("kzt_ratio").is_ok();
    let kzt_adder = web_local_storage_api::get_item("kzt_adder").is_ok();
    if !kzt_ratio || !kzt_adder {
        let (ratio, adder) = PriceCorrect::get_info_with_currency("KZT".to_string());
        let _a = web_local_storage_api::set_item("kzt_ratio", &ratio.to_string());
        let _a = web_local_storage_api::set_item("kzt_adder", &adder.to_string());
    }

    let inr_ratio = web_local_storage_api::get_item("inr_ratio").is_ok();
    let inr_adder = web_local_storage_api::get_item("inr_adder").is_ok();
    if !inr_ratio || !inr_adder {
        let (ratio, adder) = PriceCorrect::get_info_with_currency("INR".to_string());
        let _a = web_local_storage_api::set_item("inr_ratio", &ratio.to_string());
        let _a = web_local_storage_api::set_item("inr_adder", &adder.to_string());
    }
}

pub fn get_price_rate_ratio_adder(currency: &String) -> (f64, f64, i32) {
    return match currency.as_str() {
        "USD" => {(
            web_local_storage_api::get_item("USD").expect("E.").unwrap().parse().unwrap(),
            web_local_storage_api::get_item("usd_ratio").expect("E.").unwrap().parse().unwrap(),
            web_local_storage_api::get_item("usd_adder").expect("E.").unwrap().parse().unwrap()
        )},
        "EUR" => {(
            web_local_storage_api::get_item("EUR").expect("E.").unwrap().parse().unwrap(),
            web_local_storage_api::get_item("eur_ratio").expect("E.").unwrap().parse().unwrap(),
            web_local_storage_api::get_item("eur_adder").expect("E.").unwrap().parse().unwrap()
        )},
        "GBP" => {(
            web_local_storage_api::get_item("GBP").expect("E.").unwrap().parse().unwrap(),
            web_local_storage_api::get_item("gbr_ratio").expect("E.").unwrap().parse().unwrap(),
            web_local_storage_api::get_item("gbr_adder").expect("E.").unwrap().parse().unwrap()
        )},
        "BYN" => {(
            web_local_storage_api::get_item("BYN").expect("E.").unwrap().parse().unwrap(),
            web_local_storage_api::get_item("byn_ratio").expect("E.").unwrap().parse().unwrap(),
            web_local_storage_api::get_item("byn_adder").expect("E.").unwrap().parse().unwrap()
        )},
        "GEL" => {(
            web_local_storage_api::get_item("GEL").expect("E.").unwrap().parse().unwrap(),
            web_local_storage_api::get_item("gel_ratio").expect("E.").unwrap().parse().unwrap(),
            web_local_storage_api::get_item("gel_adder").expect("E.").unwrap().parse().unwrap()
        )},
        "JPY" => {(
            web_local_storage_api::get_item("JPY").expect("E.").unwrap().parse().unwrap(),
            web_local_storage_api::get_item("jpy_ratio").expect("E.").unwrap().parse().unwrap(),
            web_local_storage_api::get_item("jpy_adder").expect("E.").unwrap().parse().unwrap()
        )},
        "CHF" => {(
            web_local_storage_api::get_item("CHF").expect("E.").unwrap().parse().unwrap(),
            web_local_storage_api::get_item("chf_ratio").expect("E.").unwrap().parse().unwrap(),
            web_local_storage_api::get_item("chf_adder").expect("E.").unwrap().parse().unwrap()
        )},
        "TRY" => {(
            web_local_storage_api::get_item("TRY").expect("E.").unwrap().parse().unwrap(),
            web_local_storage_api::get_item("try_ratio").expect("E.").unwrap().parse().unwrap(),
            web_local_storage_api::get_item("try_adder").expect("E.").unwrap().parse().unwrap()
        )},
        "PLN" => {(
            web_local_storage_api::get_item("PLN").expect("E.").unwrap().parse().unwrap(),
            web_local_storage_api::get_item("pln_ratio").expect("E.").unwrap().parse().unwrap(),
            web_local_storage_api::get_item("pln_adder").expect("E.").unwrap().parse().unwrap()
        )},
        "CNY" => {(
            web_local_storage_api::get_item("CNY").expect("E.").unwrap().parse().unwrap(),
            web_local_storage_api::get_item("cny_ratio").expect("E.").unwrap().parse().unwrap(),
            web_local_storage_api::get_item("cny_adder").expect("E.").unwrap().parse().unwrap()
        )},
        "CAD" => {(
            web_local_storage_api::get_item("CAD").expect("E.").unwrap().parse().unwrap(),
            web_local_storage_api::get_item("cad_ratio").expect("E.").unwrap().parse().unwrap(),
            web_local_storage_api::get_item("cad_adder").expect("E.").unwrap().parse().unwrap()
        )},
        "KZT" => {(
            web_local_storage_api::get_item("KZT").expect("E.").unwrap().parse().unwrap(),
            web_local_storage_api::get_item("kzt_ratio").expect("E.").unwrap().parse().unwrap(),
            web_local_storage_api::get_item("kzt_adder").expect("E.").unwrap().parse().unwrap()
        )},
        "INR" => {(
            web_local_storage_api::get_item("INR").expect("E.").unwrap().parse().unwrap(),
            web_local_storage_api::get_item("inr_ratio").expect("E.").unwrap().parse().unwrap(),
            web_local_storage_api::get_item("inr_adder").expect("E.").unwrap().parse().unwrap()
        )},
        _ => (0.0,0.0,0)
    };
}