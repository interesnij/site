use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    error::InternalError,
    http::StatusCode,
    Responder,
    dev::ConnectionInfo,
};
use crate::models::User;
use std::borrow::BorrowMut;
use crate::utils::{
    is_signed_in,
    get_request_user_data,
    establish_connection,
};
use crate::models::{
    ServeCategories,
    Serve,
    WebService,
};
use actix_session::Session;
use actix_multipart::Multipart;
use sailfish::TemplateOnce;


pub fn serve_routes(config: &mut web::ServiceConfig) {
    config.route("/serve/{id}/", web::get().to(get_serve_page));
    config.route("/serve_categories/", web::get().to(serve_categories_page));

    config.service(web::resource("/create_web_services/")
        .route(web::get().to(create_web_services_page))
        .route(web::post().to(create_web_services))
    );
    config.route("/load_serve_categories_from_level/{level}/", web::get().to(load_serve_categories_from_level));
    config.route("/load_form_from_level/{level}/", web::get().to(load_form_from_level));
    config.service(web::resource("/create_serve_category/")
        .route(web::get().to(create_serve_category_page))
        .route(web::post().to(create_serve_category))
    );
    config.service(web::resource("/edit_web_service/{id}/")
        .route(web::get().to(edit_web_service_page))
        .route(web::post().to(edit_web_service))
    );
    config.service(web::resource("/edit_serve_category/{id}/")
        .route(web::get().to(edit_serve_category_page))
        .route(web::post().to(edit_serve_category))
    );

    config.service(web::resource("/create_serve/")
        .route(web::get().to(create_serve_page))
        .route(web::post().to(create_serve))
    );
    config.service(web::resource("/edit_serve/{id}/")
        .route(web::get().to(edit_serve_page))
        .route(web::post().to(edit_serve))
    ); 
    config.route("/delete_serve/", web::post().to(delete_serve));
    config.route("/delete_serve_category/", web::post().to(delete_serve_category));
    config.route("/delete_web_service/", web::post().to(delete_web_service));
}

pub async fn serve_categories_page(conn: ConnectionInfo, session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let (l, t, c) = crate::utils::get_or_create_c_user_return_ltc(conn, &req).await;

    let title: String;
    let description: String;
    let link = "/serve_categories/".to_string();
    let image = "/static/images/dark/store.jpg".to_string();
    if l == 2 {
        title = "Categories of options".to_string();
        description = "Web-services - Categories of options".to_string();
    }
    else {
        title = "Категории опций".to_string();
        description = "вебсервисы.рф - Категории опций".to_string();
    }

    if is_ajax == 0 {
        crate::utils::get_first_load_page (
            &session,
            is_desctop,
            &title,
            &description,
            &link,
            &image,
            t, 
        ).await
    }
    else if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm != 60 {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
        else {
            let _serve_cats = ServeCategories::get_all();

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/serve/categories.stpl")]
                struct Template {
                    request_user:   User,
                    serve_cats:     Vec<ServeCategories>,
                    is_ajax:        i32,
                    template_types: i16,
                    linguage:       i16,
                    currency:       String,
                    title:          String,
                    description:    String,
                    link:           String,
                    image:          String,
                }
                let body = Template {
                    request_user:   _request_user,
                    serve_cats:     _serve_cats,
                    is_ajax:        is_ajax,
                    template_types: t,
                    linguage:       l,
                    currency:       c,
                    title:          title,
                    description:    description,
                    link:           link,
                    image:          image,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/serve/categories.stpl")]
                struct Template {
                    serve_cats:     Vec<ServeCategories>,
                    is_ajax:        i32,
                    template_types: i16,
                    linguage:       i16,
                    currency:       String,
                    title:          String,
                    description:    String,
                    link:           String,
                    image:          String,
                }
                let body = Template {
                    serve_cats:     _serve_cats,
                    is_ajax:        is_ajax,
                    template_types: t,
                    linguage:       l,
                    currency:       c,
                    title:          title,
                    description:    description,
                    link:           link,
                    image:          image,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}

pub async fn get_serve_page(conn: ConnectionInfo, session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let (l, t, c) = crate::utils::get_or_create_c_user_return_ltc(conn, &req).await;
    let _serve = Serve::get(*_id);

    let title: String;
    let description: String;
    let link = "/serve/".to_string() + &_serve.id.to_string() + &"/".to_string();
    let image = "/static/images/dark/store.jpg".to_string();
    if l == 2 {
        title = "Serve ".to_string() + &_serve.name_en;
        description = "Web-services: Serve ".to_string() + &_serve.name_en;
    }
    else {
        title = "Опция ".to_string() + &_serve.name;
        description = "вебсервисы.рф - Опция ".to_string() + &_serve.name;
    }

    if is_ajax == 0 {
        crate::utils::get_first_load_page (
            &session,
            is_desctop,
            &title,
            &description,
            &link,
            &image,
            t, 
        ).await
    }
    else if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm != 60 {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
        else {
            let _s_category = ServeCategories::get(_serve.category_id);

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/serve/serve.stpl")]
                struct Template {
                    request_user:   User,
                    category:       ServeCategories,
                    object:         Serve,
                    is_ajax:        i32,
                    template_types: i16,
                    linguage:       i16,
                    currency:       String,
                    title:          String,
                    description:    String,
                    link:           String,
                    image:          String,
                }
                let body = Template {
                    request_user:   _request_user,
                    category:       _s_category,
                    object:         _serve,
                    is_ajax:        is_ajax,
                    template_types: t,
                    linguage:       l,
                    currency:       c,
                    title:          title,
                    description:    description,
                    link:           link,
                    image:          image,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/serve/serve.stpl")]
                struct Template {
                    category:       ServeCategories,
                    object:         Serve,
                    is_ajax:        i32,
                    template_types: i16,
                    linguage:       i16,
                    currency:       String,
                    title:          String,
                    description:    String,
                    link:           String,
                    image:          String,
                }
                let body = Template {
                    category:       _s_category,
                    object:         _serve,
                    is_ajax:        is_ajax,
                    template_types: t,
                    linguage:       l,
                    currency:       c,
                    title:          title,
                    description:    description,
                    link:           link,
                    image:          image,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}

pub async fn create_web_service_page(conn: ConnectionInfo, session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let (l, t, c) = crate::utils::get_or_create_c_user_return_ltc(conn, &req).await;

    let title: String;
    let description: String;
    let link = "/create_web_service/".to_string();
    let image = "/static/images/dark/store.jpg".to_string();
    if l == 2 {
        title = "Creating a web-service".to_string();
        description = "Web-services - Creating a web-service".to_string();
    }
    else {
        title = "Создание веб-сервиса".to_string();
        description = "вебсервисы.рф - Создание веб-сервиса".to_string();
    }

    if is_ajax == 0 {
        crate::utils::get_first_load_page (
            &session,
            is_desctop,
            &title,
            &description,
            &link,
            &image,
            t, 
        ).await
    }
    else if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm != 60 {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
        else {
            let _categories = WebService::get_all();

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/serve/create_web_service.stpl")]
                struct Template {
                    //request_user:   User,
                    _web_services:  Vec<WebService>,
                    is_ajax:        i32,
                    template_types: i16,
                    linguage:       i16,
                    currency:       String,
                    title:          String,
                    description:    String,
                    link:           String,
                    image:          String,
                }
                let body = Template {
                    //request_user:   _request_user,
                    _web_services:  _categories,
                    is_ajax:        is_ajax,
                    template_types: t,
                    linguage:       l,
                    currency:       c,
                    title:          title,
                    description:    description,
                    link:           link,
                    image:          image,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/serve/create_web_service.stpl")]
                struct Template {
                    _web_services:  Vec<WebService>,
                    is_ajax:        i32,
                    template_types: i16,
                    linguage:       i16,
                    currency:       String,
                    title:          String,
                    description:    String,
                    link:           String,
                    image:          String,
                }
                let body = Template {
                    _web_services:  _categories,
                    is_ajax:        is_ajax,
                    template_types: t,
                    linguage:       l,
                    currency:       c,
                    title:          title,
                    description:    description,
                    link:           link,
                    image:          image,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}
pub async fn create_serve_category_page(conn: ConnectionInfo, session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let (l, t, c) = crate::utils::get_or_create_c_user_return_ltc(conn, &req).await;

    let title: String;
    let description: String;
    let link = "/create_serve_category/".to_string();
    let image = "/static/images/dark/store.jpg".to_string();
    if l == 2 {
        title = "Creation of service technology".to_string();
        description = "Web-services - Creation of service technology".to_string();
    }
    else {
        title = "Создание технологии услуг".to_string();
        description = "вебсервисы.рф - Создание технологии услуг".to_string();
    }

    if is_ajax == 0 {
        crate::utils::get_first_load_page (
            &session,
            is_desctop,
            &title,
            &description,
            &link,
            &image,
            t, 
        ).await
    }
    else if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm != 60 {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
        else {
            let _web_services = WebService::get_all();
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/serve/create_serve_category.stpl")]
                struct Template {
                    //request_user:   User,
                    _web_services:  Vec<WebService>,
                    is_ajax:        i32,
                    template_types: i16,
                    linguage:       i16,
                    currency:       String,
                    title:          String,
                    description:    String,
                    link:           String,
                    image:          String,
                }
                let body = Template {
                    //request_user:   _request_user,
                    _web_services:  _web_services,
                    is_ajax:        is_ajax,
                    template_types: t,
                    linguage:       l,
                    currency:       c,
                    title:          title,
                    description:    description,
                    link:           link,
                    image:          image,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/serve/create_serve_category.stpl")]
                struct Template {
                    _web_services:  Vec<WebService>,
                    is_ajax:        i32,
                    template_types: i16,
                    linguage:       i16,
                    currency:       String,
                    title:          String,
                    description:    String,
                    link:           String,
                    image:          String,
                }
                let body = Template {
                    _web_services:  _web_services,
                    is_ajax:        is_ajax,
                    template_types: t,
                    linguage:       l,
                    currency:       c,
                    title:          title,
                    description:    description,
                    link:           link,
                    image:          image,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}

pub async fn load_serve_categories_from_level(req: HttpRequest, conn: ConnectionInfo, session: Session, level: web::Path<i16>) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let (l, t, c) = crate::utils::get_or_create_c_user_return_ltc(conn, &req).await;
        let _request_user = get_request_user_data(&session);
        if _request_user.perm != 60 {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/serve/load_serve_categories.stpl")]
            struct Template {
                serve_cats:     Vec<ServeCategories>,
                template_types: i16,
                linguage:       i16,
                currency:       String,
            }
            let body = Template {
                serve_cats:     ServeCategories::get_categories_from_level(&*level),
                template_types: t,
                linguage:       l,
                currency:       c,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}
pub async fn load_form_from_level(req: HttpRequest, conn: ConnectionInfo, session: Session, level: web::Path<i16>) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm != 60 {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
        else {
            let (l, t, c) = crate::utils::get_or_create_c_user_return_ltc(conn, &req).await;
            let _web_services = WebService::get_with_level(*level);

            #[derive(TemplateOnce)]
            #[template(path = "desctop/serve/load_serve_form.stpl")]
            struct Template {
                _web_services:  Vec<WebService>,
                template_types: i16,
                linguage:       i16,
                currency:       String,
            }
            let body = Template {
                _web_services:  _web_services,
                template_types: t,
                linguage:       l,
                currency:       c,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn create_serve_page(conn: ConnectionInfo, session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let (l, t, c) = crate::utils::get_or_create_c_user_return_ltc(conn, &req).await;

    let title: String;
    let description: String;
    let link = "/create_serve/".to_string();
    let image = "/static/images/dark/store.jpg".to_string();
    if l == 2 {
        title = "Creation a option".to_string();
        description = "Web-services - Creation a option".to_string();
    }
    else {
        title = "Создание опции".to_string();
        description = "вебсервисы.рф - Создание опции".to_string();
    }

    if is_ajax == 0 {
        crate::utils::get_first_load_page (
            &session,
            is_desctop,
            &title,
            &description,
            &link,
            &image,
            t, 
        ).await
    }
    else if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm != 60 {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
        else {
            let _connection = establish_connection();

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/serve/create_serve.stpl")]
                struct Template {
                    request_user:   User,
                    is_ajax:        i32,
                    template_types: i16,
                    linguage:       i16,
                    currency:       String,
                    title:          String,
                    description:    String,
                    link:           String,
                    image:          String,
                }
                let body = Template {
                    request_user:   _request_user,
                    is_ajax:        is_ajax,
                    template_types: t,
                    linguage:       l,
                    currency:       c,
                    title:          title,
                    description:    description,
                    link:           link,
                    image:          image,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/serve/create_serve.stpl")]
                struct Template {
                    is_ajax:        i32,
                    template_types: i16,
                    linguage:       i16,
                    currency:       String,
                    title:          String,
                    description:    String,
                    link:           String,
                    image:          String,
                }
                let body = Template {
                    is_ajax:        is_ajax,
                    template_types: t,
                    linguage:       l,
                    currency:       c,
                    title:          title,
                    description:    description,
                    link:           link,
                    image:          image,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}

pub async fn edit_web_service_page(conn: ConnectionInfo, session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;

    let (l, t, c) = crate::utils::get_or_create_c_user_return_ltc(conn, &req).await;
    let _category = WebService::get(*_id);
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    let title: String;
    let description: String;
    let link = "/edit_web_service/".to_string() + &_category.id.to_string() + &"/".to_string();
    let image = "/static/images/dark/store.jpg".to_string();
    if l == 2 {
        title = "Update web-service ".to_string() + &_category.name_en;
        description = "Web-services: Update web-service ".to_string() + &_category.name_en;
    }
    else {
        title = "Изменение веб-сервиса ".to_string() + &_category.name;
        description = "вебсервисы.рф - Изменение веб-сервиса ".to_string() + &_category.name;
    }

    if is_ajax == 0 {
        crate::utils::get_first_load_page (
            &session,
            is_desctop,
            &title,
            &description,
            &link,
            &image,
            t, 
        ).await
    }
    else if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(&session);
        if _category.user_id != _request_user.id {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
        else {
            let _web_services = WebService::get_all();

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/serve/edit_web_service.stpl")]
                struct Template {
                    //request_user:   User,
                    _web_services:  Vec<WebService>,
                    category:       WebService,
                    is_ajax:        i32,
                    template_types: i16,
                    linguage:       i16,
                    currency:       String,
                    title:          String,
                    description:    String,
                    link:           String,
                    image:          String,
                }
                let body = Template {
                    //request_user:   _request_user,
                    _web_services:  _web_services,
                    category:       _category,
                    is_ajax:        is_ajax,
                    template_types: t,
                    linguage:       l,
                    currency:       c,
                    title:          title,
                    description:    description,
                    link:           link,
                    image:          image,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/serve/edit_web_service.stpl")]
                struct Template {
                    _web_services:  Vec<WebService>,
                    category:       WebService,
                    is_ajax:        i32,
                    template_types: i16,
                    linguage:       i16,
                    currency:       String,
                    title:          String,
                    description:    String,
                    link:           String,
                    image:          String,
                }
                let body = Template {
                    _web_services:  _web_services,
                    category:       _category,
                    is_ajax:        is_ajax,
                    template_types: t,
                    linguage:       l,
                    currency:       c,
                    title:          title,
                    description:    description,
                    link:           link,
                    image:          image,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}

pub async fn edit_serve_category_page(conn: ConnectionInfo, session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;

    let (l, t, c) = crate::utils::get_or_create_c_user_return_ltc(conn, &req).await;
    let _category = ServeCategories::get(*_id);
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    let title: String;
    let description: String;
    let link = "/edit_serve_category/".to_string() + &_category.id.to_string() + &"/".to_string();
    let image = "/static/images/dark/store.jpg".to_string();
    if l == 2 {
        title = "Update category of serve ".to_string() + &_category.name_en;
        description = "Web-services: Update category of serve ".to_string() + &_category.name_en;
    }
    else {
        title = "Изменение категории опций ".to_string() + &_category.name;
        description = "вебсервисы.рф - Изменение категории опций ".to_string() + &_category.name;
    }

    if is_ajax == 0 {
        crate::utils::get_first_load_page (
            &session,
            is_desctop,
            &title,
            &description,
            &link,
            &image,
            t, 
        ).await
    }
    else if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(&session);
        let _web_services = WebService::get_all();

        if _category.user_id != _request_user.id {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
        else {
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/serve/edit_serve_category.stpl")]
                struct Template {
                    //request_user:   User,
                    _web_services:  Vec<WebService>,
                    category:       ServeCategories,
                    is_ajax:        i32,
                    template_types: i16,
                    linguage:       i16,
                    currency:       String,
                    title:          String,
                    description:    String,
                    link:           String,
                    image:          String,
                }
                let body = Template {
                    //request_user:   _request_user,
                    _web_services:  _web_services,
                    category:       _category,
                    is_ajax:        is_ajax,
                    template_types: t,
                    linguage:       l,
                    currency:       c,
                    title:          title,
                    description:    description,
                    link:           link,
                    image:          image,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/serve/edit_serve_category.stpl")]
                struct Template {
                    _web_services:  Vec<WebService>,
                    category:       ServeCategories,
                    is_ajax:        i32,
                    template_types: i16,
                    linguage:       i16,
                    currency:       String,
                    title:          String,
                    description:    String,
                    link:           String,
                    image:          String,
                }
                let body = Template {
                    _web_services:  _web_services,
                    category:       _category,
                    is_ajax:        is_ajax,
                    template_types: t,
                    linguage:       l,
                    currency:       c,
                    title:          title,
                    description:    description,
                    link:           link,
                    image:          image,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}

pub async fn edit_serve_page(conn: ConnectionInfo, session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let (l, t, c) = crate::utils::get_or_create_c_user_return_ltc(conn, &req).await;
    let _serve = Serve::get(*_id);

    let title: String;
    let description: String;
    let link = "/edit_serve/".to_string() + &_serve.id.to_string() + &"/".to_string();
    let image = "/static/images/dark/store.jpg".to_string();
    if l == 2 {
        title = "Update serve ".to_string() + &_serve.name_en;
        description = "Web-services: Update serve ".to_string() + &_serve.name_en;
    }
    else {
        title = "Изменение опции ".to_string() + &_serve.name;
        description = "вебсервисы.рф - Изменение опции ".to_string() + &_serve.name;
    }

    if is_ajax == 0 {
        crate::utils::get_first_load_page (
            &session,
            is_desctop,
            &title,
            &description,
            &link,
            &image,
            t, 
        ).await
    }
    else if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(&session);
        let _serve_cat = ServeCategories::get(_serve.category_id);
        let _level = WebService::get(_serve_cat.category_id).level;
        let _serve_cats = ServeCategories::get_categories_from_level(&_level);

        if _serve.user_id != _request_user.id {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
        else {
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/serve/edit_serve.stpl")]
                struct Template {
                    //request_user:   User,
                    level:          i16,
                    serve_cats:     Vec<ServeCategories>,
                    object:         Serve,
                    is_ajax:        i32,
                    template_types: i16,
                    linguage:       i16,
                    currency:       String,
                    title:          String,
                    description:    String,
                    link:           String,
                    image:          String,
                }
                let body = Template {
                    //request_user:   _request_user,
                    level:          _level,
                    serve_cats:     _serve_cats,
                    object:         _serve,
                    is_ajax:        is_ajax,
                    template_types: t,
                    linguage:       l,
                    currency:       c,
                    title:          title,
                    description:    description,
                    link:           link,
                    image:          image,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/serve/edit_serve.stpl")]
                struct Template {
                    level:          i16,
                    serve_cats:     Vec<ServeCategories>,
                    object:         Serve,
                    is_ajax:        i32,
                    template_types: i16,
                    linguage:       i16,
                    currency:       String,
                    title:          String,
                    description:    String,
                    link:           String,
                    image:          String,
                }
                let body = Template {
                    level:          _level,
                    serve_cats:     _serve_cats,
                    object:         _serve,
                    is_ajax:        is_ajax,
                    template_types: t,
                    linguage:       l,
                    currency:       c,
                    title:          title,
                    description:    description,
                    link:           link,
                    image:          image,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}

pub async fn create_web_service(req: HttpRequest, session: Session, mut payload: Multipart) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let _connection = establish_connection();
            let form = crate::utils::category_form(payload.borrow_mut(), _request_user.id).await;
            let l = crate::utils::get_c_user_l(&req);
            WebService::create(_request_user.id, form, l);
        }
    }
    return HttpResponse::Ok();
}

pub async fn create_serve_category(req: HttpRequest, session: Session, mut payload: Multipart) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let form = crate::utils::serve_category_form(payload.borrow_mut(), _request_user.id).await;
            let l = crate::utils::get_c_user_l(&req);
            ServeCategories::create(_request_user.id, form, l);
        }
    }
    return HttpResponse::Ok();
}

pub async fn edit_web_service(req: HttpRequest, session: Session, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session); 
        let form = crate::utils::category_form(payload.borrow_mut(), _request_user.id).await;
        let l = crate::utils::get_c_user_l(&req);
        WebService::update_category_with_id(_request_user, *_id, form, l);
    }
    return HttpResponse::Ok();
}

pub async fn edit_serve_category(req: HttpRequest, session: Session, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let form = crate::utils::serve_category_form(payload.borrow_mut(), _request_user.id).await;
        let l = crate::utils::get_c_user_l(&req);
        ServeCategories::update_category_with_id(_request_user, *_id, form, l);
    }
    return HttpResponse::Ok();
}

pub async fn create_serve(req: HttpRequest, session: Session, mut payload: Multipart) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let form = crate::utils::serve_split_payload(payload.borrow_mut()).await;
        let l = crate::utils::get_c_user_l(&req);
        Serve::create(_request_user, form, l);
    }
    return HttpResponse::Ok();
}

pub async fn edit_serve(req: HttpRequest, session: Session, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let form = crate::utils::serve_split_payload(payload.borrow_mut()).await;
        let l = crate::utils::get_c_user_l(&req);
        Serve::update_serve_with_id(_request_user, *_id, form, l);
    }
    return HttpResponse::Ok();
}

pub async fn delete_serve(session: Session, mut payload: Multipart) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let form = crate::utils::id_form(payload.borrow_mut()).await;
        Serve::delete(_request_user, form.id);
    }
    HttpResponse::Ok()
}

pub async fn delete_web_service(session: Session, mut payload: Multipart) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let form = crate::utils::id_form(payload.borrow_mut()).await;
        WebService::delete(_request_user, form.id);
    }
    HttpResponse::Ok()
}
pub async fn delete_serve_category(session: Session, mut payload: Multipart) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let form = crate::utils::id_form(payload.borrow_mut()).await;
        ServeCategories::delete(_request_user, form.id);
    }
    HttpResponse::Ok()
}
