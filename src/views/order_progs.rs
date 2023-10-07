use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    error::InternalError,
    http::StatusCode,
    Responder,
};
use std::borrow::BorrowMut;
use crate::utils::{
    is_signed_in,
    get_request_user_data,
    get_first_load_page,
    get_or_create_cookie_user_id,
    get_cookie_user_id,
    get_all_storage,
    get_linguage_storage,
};
use crate::models::{
    Order,
    NewOrder,
    OrderFile,
    NewOrderFile,
};
use actix_session::Session;
use actix_multipart::Multipart;
use sailfish::TemplateOnce;
use crate::models::User;
use actix_web::dev::ConnectionInfo;


pub fn order_routes(config: &mut web::ServiceConfig) {
    config.route("/orders/", web::get().to(get_orders_page));
    config.route("/user_orders/", web::get().to(get_user_orders_page));
    config.route("/order/{id}/", web::get().to(get_order_page));
    config.service(web::resource("/create_order/")
        .route(web::get().to(create_order_page))
        .route(web::post().to(create_order))
    );
    //config.service(web::resource("/edit_order/{id}/")
    //    .route(web::get().to(edit_order_page))
    //    .route(web::post().to(edit_order))
    //);
    config.route("/delete_order/{id}/", web::get().to(delete_order));
}

pub async fn get_orders_page(req: HttpRequest, session: Session) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let (t, l) = get_all_storage();
    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Заказы".to_string(),
            "вебсервисы.рф: Заказы".to_string(),
            "/orders/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
            t, 
            l,
        ).await
    }
    else if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Permission Denied"))
    }
    else {
        let (_orders, next_page_number) = Order::get_orders_list(crate::utils::get_page(&req), 20);

        let _request_user = get_request_user_data(&session);
        if _request_user.perm < 60 {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Permission Denied"))
        }
        else if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/pages/orders_list.stpl")]
            struct Template {
                request_user:     User,
                is_ajax:          i32,
                object_list:      Vec<Order>,
                next_page_number: i32,
                template_types:   u8,
                linguage:         u8,
            }
            let body = Template {
                request_user:     _request_user,
                is_ajax:          is_ajax,
                object_list:      _orders,
                next_page_number: next_page_number,
                template_types:   t,
                linguage:         l,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/pages/orders_list.stpl")]
            struct Template {
                is_ajax:          i32,
                object_list:      Vec<Order>,
                next_page_number: i32,
                template_types:   u8,
                linguage:         u8,
            }
            let body = Template {
                is_ajax:          is_ajax,
                object_list:      _orders,
                next_page_number: next_page_number,
                template_types:   t,
                linguage:         l,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn get_user_orders_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let (t, l) = get_all_storage();
    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Ваши заказы".to_string(),
            "вебсервисы.рф: Ваши заказы".to_string(),
            "/user_orders/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
            t, 
            l,
        ).await
    }
    else {
        let user_id = get_cookie_user_id(&req);
        let (_orders, next_page_number) = Order::get_user_orders_list(user_id, crate::utils::get_page(&req), 20);
        if user_id == 0 {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Информация о заказчике не найдена"))
        }
        else if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/pages/user_orders.stpl")]
                struct Template {
                    request_user:     User,
                    object_list:      Vec<Order>,
                    is_ajax:          i32,
                    next_page_number: i32,
                    template_types:   u8,
                    linguage:         u8,
                }
                let body = Template {
                    request_user:     _request_user,
                    object_list:      _orders,
                    is_ajax:          is_ajax,
                    next_page_number: next_page_number,
                    template_types:   t,
                    linguage:         l,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/pages/user_orders.stpl")]
                struct Template {
                    object_list:      Vec<Order>,
                    is_ajax:          i32,
                    next_page_number: i32,
                    template_types:   u8,
                    linguage:         u8,
                }
                let body = Template {
                    object_list:      _orders,
                    is_ajax:          is_ajax,
                    next_page_number: next_page_number,
                    template_types:   t,
                    linguage:         l,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
        else {
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/pages/anon_user_orders.stpl")]
                struct Template {
                    object_list:      Vec<Order>,
                    is_ajax:          i32,
                    next_page_number: i32,
                    template_types:   u8,
                    linguage:         u8,
                }
                let body = Template {
                    object_list:      _orders,
                    is_ajax:          is_ajax,
                    next_page_number: next_page_number,
                    template_types:   t,
                    linguage:         l,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/pages/anon_user_orders.stpl")]
                struct Template {
                    object_list:      Vec<Order>,
                    is_ajax:          i32,
                    next_page_number: i32,
                    template_types:   u8,
                    linguage:         u8,
                }
                let body = Template {
                    object_list:      _orders,
                    is_ajax:          is_ajax,
                    next_page_number: next_page_number,
                    template_types:   t,
                    linguage:         l,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}


pub async fn get_order_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let (t, l) = get_all_storage();
    let user_id = get_cookie_user_id(&req);

    let _order = Order::get(*_id); 
    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Заказ ".to_string() + &_order.title,
            "вебсервисы.рф: Заказ ".to_string() + &_order.title,
            "/order/".to_string() + &_order.id.to_string() + &"/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
            t, 
            l,
        ).await
    }
    else if user_id != _order.user_id {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Информация о заказчике не найдена"))
    }
    else {
        let _files = OrderFile::get_object_files(*_id);

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/pages/order.stpl")]
                struct Template {
                    request_user:   User,
                    object:         Order,
                    files:          Vec<OrderFile>,
                    is_ajax:        i32,
                    template_types: u8,
                    linguage:       u8,
                }
                let body = Template {
                    request_user:   _request_user,
                    object:         _order,
                    files:          _files,
                    is_ajax:        is_ajax,
                    template_types: t,
                    linguage:       l,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/pages/order.stpl")]
                struct Template {
                    object:         Order,
                    files:          Vec<OrderFile>,
                    is_ajax:        i32,
                    template_types: u8,
                    linguage:       u8,
                }
                let body = Template {
                    object:         _order,
                    files:          _files,
                    is_ajax:        is_ajax,
                    template_types: t,
                    linguage:       l,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
        else {
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/pages/anon_order.stpl")]
                struct Template {
                    object:         Order,
                    files:          Vec<OrderFile>,
                    is_ajax:        i32,
                    template_types: u8,
                    linguage:       u8,
                }
                let body = Template {
                    object:         _order,
                    files:          _files,
                    is_ajax:        is_ajax,
                    template_types: t,
                    linguage:       l,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/pages/anon_order.stpl")]
                struct Template {
                    object:         Order,
                    files:          Vec<OrderFile>,
                    is_ajax:        i32,
                    template_types: u8,
                    linguage:       u8,
                }
                let body = Template {
                    object:         _order,
                    files:          _files,
                    is_ajax:        is_ajax,
                    template_types: t,
                    linguage:       l,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}

pub async fn create_order_page() -> actix_web::Result<HttpResponse> {
    let (t, l) = get_all_storage();
    #[derive(TemplateOnce)]
    #[template(path = "desctop/pages/create_order.stpl")]
    struct Template {
        template_types: u8,
        linguage:       u8,
    }
    let body = Template {
        template_types: t,
        linguage:       l,
    }
    .render_once()
    .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
}

pub async fn create_order(conn: ConnectionInfo, req: HttpRequest, mut payload: Multipart) -> impl Responder { 
    let user_id = get_or_create_cookie_user_id(conn, &req).await;
    if user_id != 0 {
        let form = crate::utils::order_form(payload.borrow_mut(), user_id).await;
        Order::create(user_id, form, get_linguage_storage())
    }
    HttpResponse::Ok() 
}

pub async fn delete_order(req: HttpRequest, _id: web::Path<i32>) -> impl Responder {
    let user_id = get_cookie_user_id(&req);
    Order::delete(user_id, *_id);
    HttpResponse::Ok()
}
