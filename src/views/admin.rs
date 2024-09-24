//use actix::Addr;
use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    error::InternalError,
    http::StatusCode,
    web::block,
    Result,
};
use crate::schema;
use crate::models::{
    User,
    Item,
    Categories,
    Tag,
    StatPage,
    Cat,
};
use crate::utils::{
    establish_connection,
    get_device_and_ajax,
    get_request_user_data,
    is_signed_in,
    IndexResponse, AppState,
};
use crate::diesel::{
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
};
use actix_session::Session;
use sailfish::TemplateOnce;
use actix_web::dev::ConnectionInfo;
//use serde_json::to_value;
//use crate::websocket::Server;


pub fn pages_routes(config: &mut web::ServiceConfig) {
    config.route("/admin", web::get().to(admin_page));
}

