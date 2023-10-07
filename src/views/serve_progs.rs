use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    error::InternalError,
    http::StatusCode,
    Responder,
};
use crate::models::User;
use std::borrow::BorrowMut;
use crate::utils::{
    is_signed_in,
    get_request_user_data,
    get_first_load_page,
    get_all_storage,
    establish_connection,
    get_linguage_storage,
};
use crate::models::{
    ServeCategories,
    Serve,
    NewServe,
    TechCategories,
};
use actix_session::Session;
use actix_multipart::{Field, Multipart};
use serde::{Deserialize, Serialize};
use std::str;
use sailfish::TemplateOnce;


pub fn serve_routes(config: &mut web::ServiceConfig) {
    config.route("/serve/{id}/", web::get().to(get_serve_page));
    config.route("/serve_categories/", web::get().to(serve_categories_page));

    config.service(web::resource("/create_tech_categories/")
        .route(web::get().to(create_tech_categories_page))
        .route(web::post().to(create_tech_categories))
    );
    config.route("/load_serve_categories_from_level/{level}/", web::get().to(load_serve_categories_from_level));
    config.route("/load_form_from_level/{level}/", web::get().to(load_form_from_level));
    config.service(web::resource("/create_serve_categories/")
        .route(web::get().to(create_serve_categories_page))
        .route(web::post().to(create_serve_categories))
    );
    config.service(web::resource("/edit_tech_category/{id}/")
        .route(web::get().to(edit_tech_category_page))
        .route(web::post().to(edit_tech_category))
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
    config.route("/delete_serve/{id}/", web::get().to(delete_serve));
    config.route("/delete_serve_category/{id}/", web::get().to(delete_serve_category));
    config.route("/delete_tech_category/{id}/", web::get().to(delete_tech_category));
}

pub async fn serve_categories_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let (t, l) = get_all_storage();

    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Категории услуг".to_string(),
            "вебсервисы.рф: Категории услуг".to_string(),
            "/serve_categories/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
            t, 
            l,
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
                    template_types: u8,
                    linguage:       u8,
                }
                let body = Template {
                    request_user:   _request_user,
                    serve_cats:     _serve_cats,
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
                #[template(path = "mobile/serve/categories.stpl")]
                struct Template {
                    serve_cats:     Vec<ServeCategories>,
                    is_ajax:        i32,
                    template_types: u8,
                    linguage:       u8,
                }
                let body = Template {
                    serve_cats:     _serve_cats,
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

pub async fn get_serve_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let (t, l) = get_all_storage();
    let _serve = Serve::get(*_id);

    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Опция ".to_string() + &_serve.name,
            "вебсервисы.рф: Опция ".to_string() + &_serve.name,
            "/serve/".to_string() + &_serve.id.to_string() + &"/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
            t, 
            l,
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
                    template_types: u8,
                    linguage:       u8,
                }
                let body = Template {
                    request_user:   _request_user,
                    category:       _s_category,
                    object:         _serve,
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
                #[template(path = "mobile/serve/serve.stpl")]
                struct Template {
                    category:       ServeCategories,
                    object:         Serve,
                    is_ajax:        i32,
                    template_types: u8,
                    linguage:       u8,
                }
                let body = Template {
                    category:       _s_category,
                    object:         _serve,
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

pub async fn create_tech_categories_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let (t, l) = get_all_storage();
    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Создание веб-сервиса".to_string(),
            "вебсервисы.рф: Создание веб-сервиса".to_string(),
            "/create_tech_categories/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
            t, 
            l,
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
            let _categories = TechCategories::get_all();

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/serve/create_tech_categories.stpl")]
                struct Template {
                    request_user:   User,
                    tech_cats:      Vec<TechCategories>,
                    is_ajax:        i32,
                    template_types: u8,
                    linguage:       u8,
                }
                let body = Template {
                    request_user:   _request_user,
                    tech_cats:      _categories,
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
                #[template(path = "mobile/serve/create_tech_categories.stpl")]
                struct Template {
                    tech_cats:      Vec<TechCategories>,
                    is_ajax:        i32,
                    template_types: u8,
                    linguage:       u8,
                }
                let body = Template {
                    tech_cats:      _categories,
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
pub async fn create_serve_categories_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let (t, l) = get_all_storage();

    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Создание технологии услуг".to_string(),
            "вебсервисы.рф: Создание технологии услуг".to_string(),
            "/create_serve_categories/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
            t, 
            l,
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
            let _tech_categories = TechCategories::get_all();
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/serve/create_serve_categories.stpl")]
                struct Template {
                    request_user:   User,
                    tech_cats:      Vec<TechCategories>,
                    is_ajax:        i32,
                    template_types: u8,
                    linguage:       u8,
                }
                let body = Template {
                    request_user:   _request_user,
                    tech_cats:      _tech_categories,
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
                #[template(path = "mobile/serve/create_serve_categories.stpl")]
                struct Template {
                    tech_cats:      Vec<TechCategories>,
                    is_ajax:        i32,
                    template_types: u8,
                    linguage:       u8,
                }
                let body = Template {
                    tech_cats:      _tech_categories,
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

pub async fn load_serve_categories_from_level(session: Session, level: web::Path<i16>) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let (t, l) = get_all_storage();
        let _request_user = get_request_user_data(&session);
        if _request_user.perm != 60 {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/serve/load_serve_categories.stpl")]
            struct Template {
                serve_cats:     Vec<ServeCategories>,
                template_types: u8,
                linguage:       u8,
            }
            let body = Template {
                serve_cats:     ServeCategories::get_categories_from_level(&*level),
                template_types: t,
                linguage:       l,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}
pub async fn load_form_from_level(session: Session, level: web::Path<i16>) -> actix_web::Result<HttpResponse> {
    if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm != 60 {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
        else {
            let (t, l) = get_all_storage();
            let _tech_categories = TechCategories::get_with_level(*level);

            #[derive(TemplateOnce)]
            #[template(path = "desctop/serve/load_serve_form.stpl")]
            struct Template {
                tech_cats:      Vec<TechCategories>,
                template_types: u8,
                linguage:       u8,
            }
            let body = Template {
                tech_cats:      _tech_categories,
                template_types: t,
                linguage:       l,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn create_serve_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let (t, l) = get_all_storage();
    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Создание опции".to_string(),
            "вебсервисы.рф: Создание опции".to_string(),
            "/create_serve/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
            t, 
            l,
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
                    template_types: u8,
                    linguage:       u8,
                }
                let body = Template {
                    request_user:   _request_user,
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
                #[template(path = "mobile/serve/create_serve.stpl")]
                struct Template {
                    is_ajax:        i32,
                    template_types: u8,
                    linguage:       u8,
                }
                let body = Template {
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

pub async fn edit_tech_category_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;

    let (t, l) = get_all_storage();
    let _category = TechCategories::get(*_id);
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Изменение веб-сервиса ".to_string() + &_category.name,
            "вебсервисы.рф: Изменение веб-сервиса ".to_string() + &_category.name,
            "/edit_tech_category/".to_string() + &_category.id.to_string() + &"/".to_string(),
            "".to_string(),
            t, 
            l,
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
            let _tech_categories = TechCategories::get_all();

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/serve/edit_tech_category.stpl")]
                struct Template {
                    request_user:   User,
                    tech_cats:      Vec<TechCategories>,
                    category:       TechCategories,
                    is_ajax:        i32,
                    template_types: u8,
                    linguage:       u8,
                }
                let body = Template {
                    request_user:   _request_user,
                    tech_cats:      _tech_categories,
                    category:       _category,
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
                #[template(path = "mobile/serve/edit_tech_category.stpl")]
                struct Template {
                    tech_cats:      Vec<TechCategories>,
                    category:       TechCategories,
                    is_ajax:        i32,
                    template_types: u8,
                    linguage:       u8,
                }
                let body = Template {
                    tech_cats:      _tech_categories,
                    category:       _category,
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

pub async fn edit_serve_category_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;

    let (t, l) = get_all_storage();
    let _category = ServeCategories::get(*_id);
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Изменение категории опций ".to_string() + &_category.name,
            "вебсервисы.рф: Изменение категории опций ".to_string() + &_category.name,
            "/edit_serve_category/".to_string() + &_category.id.to_string() + &"/".to_string(),
            "".to_string(),
            t, 
            l,
        ).await
    }
    else if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(&session);
        let _tech_categories = TechCategories::get_all();

        if _category.user_id != _request_user.id {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
        else {
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/serve/edit_serve_category.stpl")]
                struct Template {
                    request_user:   User,
                    tech_cats:      Vec<TechCategories>,
                    category:       ServeCategories,
                    is_ajax:        i32,
                    template_types: u8,
                    linguage:       u8,
                }
                let body = Template {
                    request_user:   _request_user,
                    tech_cats:      _tech_categories,
                    category:       _category,
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
                #[template(path = "mobile/serve/edit_serve_category.stpl")]
                struct Template {
                    tech_cats:      Vec<TechCategories>,
                    category:       ServeCategories,
                    is_ajax:        i32,
                    template_types: u8,
                    linguage:       u8,
                }
                let body = Template {
                    tech_cats:      _tech_categories,
                    category:       _category,
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

pub async fn edit_serve_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let (t, l) = get_all_storage();
    let _serve = Serve::get(*_id);

    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Изменение опции ".to_string() + &_serve.name,
            "вебсервисы.рф: Изменение опции ".to_string() + &_serve.name,
            "/edit_serve/".to_string() + &_serve.id.to_string() + &"/".to_string(),
            "".to_string(),
            t, 
            l,
        ).await
    }
    else if !is_signed_in(&session) {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = get_request_user_data(&session);
        let _serve_cat = ServeCategories::get(_serve.category_id);
        let _level = TechCategories::get(_serve_cat.category_id).level;
        let _serve_cats = ServeCategories::get_categories_from_level(&_level);

        if _serve.user_id != _request_user.id {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
        }
        else {
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/serve/edit_serve.stpl")]
                struct Template {
                    request_user:   User,
                    level:          i16,
                    serve_cats:     Vec<ServeCategories>,
                    object:         Serve,
                    is_ajax:        i32,
                    template_types: u8,
                    linguage:       u8,
                }
                let body = Template {
                    request_user:   _request_user,
                    level:          _level,
                    serve_cats:     _serve_cats,
                    object:         _serve,
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
                #[template(path = "mobile/serve/edit_serve.stpl")]
                struct Template {
                    level:          i16,
                    serve_cats:     Vec<ServeCategories>,
                    object:         Serve,
                    is_ajax:        i32,
                    template_types: u8,
                    linguage:       u8,
                }
                let body = Template {
                    level:          _level,
                    serve_cats:     _serve_cats,
                    object:         _serve,
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

pub async fn create_tech_categories(session: Session, mut payload: Multipart) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let _connection = establish_connection();
            let form = crate::utils::category_form(payload.borrow_mut(), _request_user.id).await;
            TechCategories::create(_request_user.id, form);
        }
    }
    return HttpResponse::Ok();
}

pub async fn create_serve_categories(session: Session, mut payload: Multipart) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let form = crate::utils::serve_category_form(payload.borrow_mut(), _request_user.id).await;
            ServeCategories::create(_request_user.id, form);
        }
    }
    return HttpResponse::Ok();
}

pub async fn edit_tech_category(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session); 
        let form = crate::utils::category_form(payload.borrow_mut(), _request_user.id).await;
        TechCategories::update_category_with_id(_request_user, *_id, form);
    }
    return HttpResponse::Ok();
}

pub async fn edit_serve_category(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let form = crate::utils::serve_category_form(payload.borrow_mut(), _request_user.id).await;
        ServeCategories::update_category_with_id(_request_user, *_id, form);
    }
    return HttpResponse::Ok();
}

pub async fn create_serve(session: Session, mut payload: Multipart) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let form = crate::utils::serve_split_payload(payload.borrow_mut()).await;
        Serve::create(_request_user, form);
    }
    return HttpResponse::Ok();
}

pub async fn edit_serve(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let form = crate::utils::serve_split_payload(payload.borrow_mut()).await;
        Serve::update_serve_with_id(_request_user, *_id, form, get_linguage_storage());
    }
    return HttpResponse::Ok();
}


pub async fn delete_serve(session: Session, _id: web::Path<i32>) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        Serve::delete(_request_user, *_id);
    }
    HttpResponse::Ok()
}

pub async fn delete_tech_category(session: Session, _id: web::Path<i32>) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        TechCategories::delete(_request_user, *_id);
    }
    HttpResponse::Ok()
}
pub async fn delete_serve_category(session: Session, _id: web::Path<i32>) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        ServeCategories::delete(_request_user, *_id);
    }
    HttpResponse::Ok()
}
