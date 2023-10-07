use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    //web::block,
    error::InternalError,
    http::StatusCode,
    Responder,
};
use crate::models::User;
use actix_multipart::Multipart;
use std::borrow::BorrowMut;
use crate::diesel::{
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
};
use actix_session::Session;
use crate::utils::{
    establish_connection,
    is_signed_in,
    get_request_user_data,
    get_first_load_page,
    get_all_storage,
};
use crate::schema;
use crate::models::{
    Tag, StatPage,
    SmallTag,
    TagItems,
};
use sailfish::TemplateOnce;


pub fn tag_routes(config: &mut web::ServiceConfig) {
    config.route("/tags/", web::get().to(tags_page));
    config.route("/tag/{slug}/", web::get().to(tag_page));
    config.route("/tag_blogs/{slug}/", web::get().to(tag_blogs_page));
    config.route("/tag_services/{slug}/", web::get().to(tag_services_page));
    config.route("/tag_stores/{slug}/", web::get().to(tag_stores_page));
    config.route("/tag_wikis/{slug}/", web::get().to(tag_wikis_page));
    config.route("/tag_works/{slug}/", web::get().to(tag_works_page));
    config.route("/tag_helps/{slug}/", web::get().to(tag_helps_page));
    config.service(web::resource("/create_tag/")
        .route(web::get().to(create_tag_page))
        .route(web::post().to(create_tag))
    );
    config.service(web::resource("/edit_tag/{id}/")
        .route(web::get().to(edit_tag_page))
        .route(web::post().to(edit_tag))
    );
    config.route("/delete_tag/{id}/", web::get().to(delete_tag));
}

pub async fn create_tag_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let (t, l) = get_all_storage();
    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Создание тега".to_string(),
            "вебсервисы.рф: Создание тега".to_string(),
            "/create_tag/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
            t, 
            l,
        ).await
    }
    else {
        let all_tags = Tag::get_all();

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/tags/create_tag.stpl")]
                struct Template {
                    request_user:   User,
                    all_tags:       Vec<Tag>,
                    is_ajax:        i32,
                    template_types: u8,
                    linguage:       u8,
                }
                let body = Template {
                    request_user:   _request_user,
                    all_tags:       all_tags,
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
                #[template(path = "mobile/tags/create_tag.stpl")]
                struct Template {
                    all_tags:       Vec<Tag>,
                    is_ajax:        i32,
                    template_types: u8,
                    linguage:       u8,
                }
                let body = Template {
                    all_tags:       all_tags,
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
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Permission Denied."))
        }
    }
}

pub async fn create_tag(session: Session, mut payload: Multipart) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.is_superuser() {
            let form = crate::utils::category_form(payload.borrow_mut(), _request_user.id).await;
            Tag::create(_request_user, form);
        }
    }
    return HttpResponse::Ok();
}

pub async fn tag_page(req: HttpRequest, session: Session, _id: web::Path<String>) -> actix_web::Result<HttpResponse> {
    let (t, l) = get_all_storage();
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let slug = _id.to_string();
    let _tag = Tag::get_tag_with_slug(&slug);

    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            _tag.name.clone() + &" | Тег".to_string(),
            _tag.name.clone() + &" | вебсервисы.рф:Тег".to_string(),
            "/tag/".to_string() + &slug + &"/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
            t, 
            l,
        ).await
    }
    else {
        use crate::models::{Item, Blog, Service, Store, Wiki, Work, Help};

        let (blog_stack, service_stack, store_stack, wiki_stack, work_stack, help_stack) = Tag::get_objects_ids(_tag.id);
        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            let is_admin = _request_user.is_superuser(); 
            let (blogs, blogs_count) = Item::get_blogs_for_ids(3, 0, blog_stack, is_admin, l);
            let (_services, services_count) = Item::get_services_for_ids(3, 0, service_stack, is_admin, l);
            let (_stores, stores_count) = Item::get_stores_for_ids(3, 0, store_stack, is_admin, l);
            let (_wikis, wikis_count) = Item::get_wikis_for_ids(3, 0, wiki_stack, is_admin, l);
            let (_works, works_count) = Item::get_works_for_ids(3, 0, work_stack, is_admin, l);
            let (_helps, helps_count) = Item::get_helps_for_ids(3, 0, help_stack, is_admin, l);

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/tags/tag.stpl")]
                struct Template {
                    tag:            Tag,
                    request_user:   User,
                    works_list:     Vec<Work>,
                    services_list:  Vec<Service>,
                    wikis_list:     Vec<Wiki>,
                    blogs_list:     Vec<Blog>,
                    stores_list:    Vec<Store>,
                    helps_list:     Vec<Help>,

                    works_count:    usize,
                    services_count: usize,
                    wikis_count:    usize,
                    blogs_count:    usize,
                    stores_count:   usize,
                    helps_count:    usize,
                    is_ajax:        i32,
                    template_types: u8,
                    linguage:       u8,
                }
                let body = Template {
                    tag:            _tag,
                    request_user:   _request_user,
                    works_list:     _works,
                    services_list:  _services,
                    wikis_list:     _wikis,
                    blogs_list:     _blogs,
                    stores_list:    _stores,
                    helps_list:     _helps,

                    works_count:    works_count,
                    services_count: services_count,
                    wikis_count:    wikis_count,
                    blogs_count:    blogs_count,
                    stores_count:   stores_count,
                    helps_count:    helps_count,
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
                #[template(path = "mobile/tags/tag.stpl")]
                struct Template {
                    tag:            Tag,
                    works_list:     Vec<Work>,
                    services_list:  Vec<Service>,
                    wikis_list:     Vec<Wiki>,
                    blogs_list:     Vec<Blog>,
                    stores_list:    Vec<Store>,
                    helps_list:     Vec<Help>,

                    works_count:    usize,
                    services_count: usize,
                    wikis_count:    usize,
                    blogs_count:    usize,
                    stores_count:   usize,
                    helps_count:    usize,
                    is_ajax:        i32,
                    template_types: u8,
                    linguage:       u8,
                }
                let body = Template {
                    tag:            _tag,
                    works_list:     _works,
                    services_list:  _services,
                    wikis_list:     _wikis,
                    blogs_list:     _blogs,
                    stores_list:    _stores,
                    helps_list:     _helps,

                    works_count:    works_count,
                    services_count: services_count,
                    wikis_count:    wikis_count,
                    blogs_count:    blogs_count,
                    stores_count:   stores_count,
                    helps_count:    helps_count,
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
            let (_blogs, blogs_count) = Item::get_blogs_for_ids(3, 0, blog_stack, false, l);
            let (_services, services_count) = Item::get_services_for_ids(3, 0, service_stack, false, l);
            let (_stores, stores_count) = Item::get_stores_for_ids(3, 0, store_stack, false, l);
            let (_wikis, wikis_count) = Item::get_wikis_for_ids(3, 0, wiki_stack, false, l);
            let (_works, works_count) = Item::get_works_for_ids(3, 0, work_stack, false, l);
            let (_helps, helps_count) = Item::get_helps_for_ids(3, 0, help_stack, false, l);

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/tags/anon_tag.stpl")]
                struct Template {
                    tag:            Tag,
                    works_list:     Vec<Work>,
                    services_list:  Vec<Service>,
                    wikis_list:     Vec<Wiki>,
                    blogs_list:     Vec<Blog>,
                    stores_list:    Vec<Store>,
                    helps_list:     Vec<Help>,

                    works_count:    usize,
                    services_count: usize,
                    wikis_count:    usize,
                    blogs_count:    usize,
                    stores_count:   usize,
                    helps_count:    usize,
                    is_ajax:        i32,
                    template_types: u8,
                    linguage:       u8,
                }
                let body = Template {
                    tag:            _tag,
                    works_list:     _works,
                    services_list:  _services,
                    wikis_list:     _wikis,
                    blogs_list:     _blogs,
                    stores_list:    _stores,
                    helps_list:     _helps,

                    works_count:    works_count,
                    services_count: services_count,
                    wikis_count:    wikis_count,
                    blogs_count:    blogs_count,
                    stores_count:   stores_count,
                    helps_count:    helps_count,
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
                #[template(path = "mobile/tags/anon_tag.stpl")]
                struct Template {
                    tag:            Tag,
                    works_list:     Vec<Work>,
                    services_list:  Vec<Service>,
                    wikis_list:     Vec<Wiki>,
                    blogs_list:     Vec<Blog>,
                    stores_list:    Vec<Store>,
                    helps_list:     Vec<Help>,

                    works_count:    usize,
                    services_count: usize,
                    wikis_count:    usize,
                    blogs_count:    usize,
                    stores_count:   usize,
                    helps_count:    usize,
                    is_ajax:        i32,
                    template_types: u8,
                    linguage:       u8,
                }
                let body = Template {
                    tag:            _tag,
                    works_list:     _works,
                    services_list:  _services,
                    wikis_list:     _wikis,
                    blogs_list:     _blogs,
                    stores_list:    _stores,
                    helps_list:     _helps,

                    works_count:    works_count,
                    services_count: services_count,
                    wikis_count:    wikis_count,
                    blogs_count:    blogs_count,
                    stores_count:   stores_count,
                    helps_count:    helps_count,
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

pub async fn tag_blogs_page(session: Session, req: HttpRequest, _id: web::Path<String>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let _connection = establish_connection();
    let (t, l) = get_all_storage();
    let slug = _id.to_string();
    let _tag = Tag::get_tag_with_slug(&slug); 

    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            _tag.name.clone() + &" | Статьи тега".to_string(),
            _tag.name.clone() + &" | вебсервисы.рф: Статьи тега".to_string(),
            "/tag_blogs/".to_string() + &slug + &"/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
            t, 
            l,
        ).await
    }
    else {
        use crate::schema::tags_items::dsl::tags_items;
        use crate::models::{Item, Blog};
        use crate::utils::get_page;

        let page = get_page(&req);

        let _tag_items = tags_items
            .filter(schema::tags_items::tag_id.eq(&_tag.id))
            .filter(schema::tags_items::types.eq(1))
            .select(schema::tags_items::item_id)
            .load::<i32>(&_connection)
            .expect("E");

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);

            let data = Item::get_blogs_list_for_ids(page, 20, _tag_items, _request_user.is_superuser(), l);
            let (_blogs, blog_count) = data.0;
            let next_page_number = data.1;

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/tags/tag_blogs.stpl")]
                struct Template {
                    request_user:     User,
                    tag:              Tag,
                    blogs_list:       Vec<Blog>,
                    blogs_count:      usize,
                    next_page_number: i32,
                    is_ajax:          i32,
                    template_types:   u8,
                    linguage:         u8,
                }
                let body = Template {
                    request_user:     _request_user,
                    tag:              _tag,
                    blogs_list:       _blogs,
                    blogs_count:      blog_count,
                    next_page_number: next_page_number,
                    is_ajax:          is_ajax,
                    template_types:   t,
                    linguage:         l,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/tags/tag_blogs.stpl")]
                struct Template {
                    tag:              Tag,
                    blogs_list:       Vec<Blog>,
                    blogs_count:      usize,
                    next_page_number: i32,
                    is_ajax:          i32,
                    template_types:   u8,
                    linguage:         u8,
                }
                let body = Template {
                    tag:              _tag,
                    blogs_list:       _blogs,
                    blogs_count:      blog_count,
                    next_page_number: next_page_number,
                    is_ajax:          is_ajax,
                    template_types:   t,
                    linguage:         l,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
        else {
            let data = Item::get_blogs_list_for_ids(page, 20, _tag_items, false, l);
            let (_blogs, blog_count) = data.0;
            let next_page_number = data.1;

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/tags/anon_tag_blogs.stpl")]
                struct Template {
                    tag:              Tag,
                    blogs_list:       Vec<Blog>,
                    blogs_count:      usize,
                    next_page_number: i32,
                    is_ajax:          i32,
                    template_types:   u8,
                    linguage:         u8,
                }
                let body = Template {
                    tag:              _tag,
                    blogs_list:       _blogs,
                    blogs_count:      blog_count,
                    next_page_number: next_page_number,
                    is_ajax:          is_ajax,
                    template_types:   t,
                    linguage:         l,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/tags/anon_tag_blogs.stpl")]
                struct Template {
                    tag:              Tag,
                    blogs_list:       Vec<Blog>,
                    blogs_count:      usize,
                    next_page_number: i32,
                    is_ajax:          i32,
                    template_types:   u8,
                    linguage:         u8,
                }
                let body = Template {
                    tag:              _tag,
                    blogs_list:       _blogs,
                    blogs_count:      blog_count,
                    next_page_number: next_page_number,
                    is_ajax:          is_ajax,
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

pub async fn tag_services_page(session: Session, req: HttpRequest, _id: web::Path<String>) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;

    let _connection = establish_connection();
    let (t, l) = get_all_storage();
    let slug = _id.to_string();
    let _tag = Tag::get_tag_with_slug(&slug); 
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            _tag.name.clone() + &" | Услуги тега".to_string(),
            _tag.name.clone() + &" | вебсервисы.рф: Услуги тега".to_string(),
            "/tag_services/".to_string() + &slug + &"/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
            t, 
            l,
        ).await
    }
    else {
        use crate::schema::tags_items::dsl::tags_items;
        use crate::models::{Item, Service};
        use crate::utils::get_page;

        let page = get_page(&req);
        let _tag_items = tags_items
            .filter(schema::tags_items::tag_id.eq(&_tag.id))
            .filter(schema::tags_items::types.eq(2))
            .select(schema::tags_items::item_id)
            .load::<i32>(&_connection)
            .expect("E");

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            let data = Item::get_services_list_for_ids(page, 20, _tag_items, _request_user.is_superuser(), l);
            let (_services, service_count) = data.0;
            let next_page_number = data.1;

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/tags/tag_services.stpl")]
                struct Template {
                    request_user:     User,
                    tag:              Tag,
                    services_list:    Vec<Service>,
                    services_count:   usize,
                    next_page_number: i32,
                    is_ajax:          i32,
                    template_types:   u8,
                    linguage:         u8,
                }
                let body = Template {
                    request_user:     _request_user,
                    tag:              _tag,
                    services_list:    _services,
                    services_count:   service_count,
                    next_page_number: next_page_number,
                    is_ajax:          is_ajax,
                    template_types:   t,
                    linguage:         l,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/tags/tag_services.stpl")]
                struct Template {
                    tag:              Tag,
                    services_list:    Vec<Service>,
                    services_count:   usize,
                    next_page_number: i32,
                    is_ajax:          i32,
                    template_types:   u8,
                    linguage:         u8,
                }
                let body = Template {
                    tag:              _tag,
                    services_list:    _services,
                    services_count:   service_count,
                    next_page_number: next_page_number,
                    is_ajax:          is_ajax,
                    template_types:   t,
                    linguage:         l,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
        else {
            let data = Item::get_services_list_for_ids(page, 20, _tag_items, false, l);
            let (_services, service_count) = data.0;
            let next_page_number = data.1;

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/tags/anon_tag_services.stpl")]
                struct Template {
                    tag:              Tag,
                    services_list:    Vec<Service>,
                    services_count:   usize,
                    next_page_number: i32,
                    is_ajax:          i32,
                    template_types:   u8,
                    linguage:         u8,
                }
                let body = Template {
                    tag:              _tag,
                    services_list:    _services,
                    services_count:   service_count,
                    next_page_number: next_page_number,
                    is_ajax:          is_ajax,
                    template_types:   t,
                    linguage:         l,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/tags/anon_tag_services.stpl")]
                struct Template {
                    tag:              Tag,
                    services_list:    Vec<Service>,
                    services_count:   usize,
                    next_page_number: i32,
                    is_ajax:          i32,
                    template_types:   u8,
                    linguage:         u8,
                }
                let body = Template {
                    tag:              _tag,
                    services_list:    _services,
                    services_count:   service_count,
                    next_page_number: next_page_number,
                    is_ajax:          is_ajax,
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

pub async fn tag_stores_page(session: Session, req: HttpRequest, _id: web::Path<String>) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;

    let _connection = establish_connection();
    let (t, l) = get_all_storage();
    let slug = _id.to_string();
    let _tag = Tag::get_tag_with_slug(&slug); 
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            _tag.name.clone() + &" | Товары тега ".to_string(),
            _tag.name.clone() + &" | вебсервисы.рф: Товары тега ".to_string(),
            "/tag_stores/".to_string() + &slug + &"/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
            t, 
            l,
        ).await
    }
    else {
        use crate::schema::tags_items::dsl::tags_items;
        use crate::models::{Item, Store};
        use crate::utils::get_page;

        let page = get_page(&req);

        let _tag_items = tags_items
            .filter(schema::tags_items::tag_id.eq(&_tag.id))
            .filter(schema::tags_items::types.eq(3))
            .select(schema::tags_items::item_id)
            .load::<i32>(&_connection)
            .expect("E");

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            let data = Item::get_stores_list_for_ids(page, 20, _tag_items, _request_user.is_superuser(), l);
            let (_stores, stores_count) = data.0;
            let next_page_number = data.1;

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/tags/tag_stores.stpl")]
                struct Template {
                    request_user:     User,
                    tag:              Tag,
                    stores_list:      Vec<Store>,
                    stores_count:     usize,
                    next_page_number: i32,
                    is_ajax:          i32,
                    template_types:   u8,
                    linguage:         u8,
                }
                let body = Template {
                    request_user:     _request_user,
                    tag:              _tag,
                    stores_list:      _stores,
                    stores_count:     stores_count,
                    next_page_number: next_page_number,
                    is_ajax:          is_ajax,
                    template_types:   t,
                    linguage:         l,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/tags/tag_stores.stpl")]
                struct Template {
                    tag:              Tag,
                    stores_list:      Vec<Store>,
                    stores_count:     usize,
                    next_page_number: i32,
                    is_ajax:          i32,
                    template_types:   u8,
                    linguage:         u8,
                }
                let body = Template {
                    tag:              _tag,
                    stores_list:      _stores,
                    stores_count:     stores_count,
                    next_page_number: next_page_number,
                    is_ajax:          is_ajax,
                    template_types:   t,
                    linguage:         l,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
        else {
            let data = Item::get_stores_list_for_ids(page, 20, _tag_items, false, l);
            let (_stores, stores_count) = data.0;
            let next_page_number = data.1;

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/tags/anon_tag_stores.stpl")]
                struct Template {
                    tag:              Tag,
                    stores_list:      Vec<Store>,
                    stores_count:     usize,
                    next_page_number: i32,
                    is_ajax:          i32,
                    template_types:   u8,
                    linguage:         u8,
                }
                let body = Template {
                    tag:              _tag,
                    stores_list:      _stores,
                    stores_count:     stores_count,
                    next_page_number: next_page_number,
                    is_ajax:          is_ajax,
                    template_types:   t,
                    linguage:         l,                
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/tags/anon_tag_stores.stpl")]
                struct Template {
                    tag:              Tag,
                    stores_list:      Vec<Store>,
                    stores_count:     usize,
                    next_page_number: i32,
                    is_ajax:          i32,
                    template_types:   u8,
                    linguage:         u8,
                }
                let body = Template {
                    tag:              _tag,
                    stores_list:      _stores,
                    stores_count:     stores_count,
                    next_page_number: next_page_number,
                    is_ajax:          is_ajax,
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

pub async fn tag_wikis_page(session: Session, req: HttpRequest, _id: web::Path<String>) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;

    let _connection = establish_connection();
    let (t, l) = get_all_storage();
    let slug = _id.to_string();
    let _tag = Tag::get_tag_with_slug(&slug); 
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            _tag.name.clone() + &" | Статьи тега".to_string(),
            _tag.name.clone() + &" | вебсервисы.рф: Статьи тега".to_string(),
            "/tag_wikis/".to_string() + &slug + &"/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
            t, 
            l,
        ).await
    }
    else {
        use crate::schema::tags_items::dsl::tags_items;
        use crate::models::{Item, Wiki};
        use crate::utils::get_page;

        let page = get_page(&req);

        let _tag_items = tags_items
            .filter(schema::tags_items::tag_id.eq(&_tag.id))
            .filter(schema::tags_items::types.eq(4))
            .select(schema::tags_items::item_id)
            .load::<i32>(&_connection)
            .expect("E");

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            let data = Item::get_wikis_list_for_ids(page, 20, _tag_items, _request_user.is_superuser(), l);
            let (_wikis, wikis_count) = data.0;
            let next_page_number = data.1;

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/tags/tag_wikis.stpl")]
                struct Template {
                    request_user:     User,
                    tag:              Tag,
                    wikis_list:       Vec<Wiki>,
                    wikis_count:      usize,
                    next_page_number: i32,
                    is_ajax:          i32,
                    template_types:   u8,
                    linguage:         u8,
                }
                let body = Template {
                    request_user:     _request_user,
                    tag:              _tag,
                    wikis_list:       _wikis,
                    wikis_count:      wikis_count,
                    next_page_number: next_page_number,
                    is_ajax:          is_ajax,
                    template_types:   t,
                    linguage:         l,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/tags/tag_wikis.stpl")]
                struct Template {
                    tag:              Tag,
                    wikis_list:       Vec<Wiki>,
                    wikis_count:      usize,
                    next_page_number: i32,
                    is_ajax:          i32,
                    template_types:   u8,
                    linguage:         u8,
                }
                let body = Template {
                    tag:              _tag,
                    wikis_list:       _wikis,
                    wikis_count:      wikis_count,
                    next_page_number: next_page_number,
                    is_ajax:          is_ajax,
                    template_types:   t,
                    linguage:         l,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
        else {
            let data = Item::get_wikis_list_for_ids(page, 20, _tag_items, false, l);
            let (_wikis, wikis_count) = data.0;
            let next_page_number = data.1;

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/tags/anon_tag_wikis.stpl")]
                struct Template {
                    tag:              Tag,
                    wikis_list:       Vec<Wiki>,
                    wikis_count:      usize,
                    next_page_number: i32,
                    is_ajax:          i32,
                    template_types:   u8,
                    linguage:         u8,
                }
                let body = Template {
                    tag:              _tag,
                    wikis_list:       _wikis,
                    wikis_count:      wikis_count,
                    next_page_number: next_page_number,
                    is_ajax:          is_ajax,
                    template_types:   t,
                    linguage:         l,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/tags/anon_tag_wikis.stpl")]
                struct Template {
                    tag:              Tag,
                    wikis_list:       Vec<Wiki>,
                    wikis_count:      usize,
                    next_page_number: i32,
                    is_ajax:          i32,
                    template_types:   u8,
                    linguage:         u8,
                }
                let body = Template {
                    tag:              _tag,
                    wikis_list:       _wikis,
                    wikis_count:      wikis_count,
                    next_page_number: next_page_number,
                    is_ajax:          is_ajax,
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

pub async fn tag_works_page(session: Session, req: HttpRequest, _id: web::Path<String>) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;

    let _connection = establish_connection();
    let (t, l) = get_all_storage();
    let slug = _id.to_string();
    let _tag = Tag::get_tag_with_slug(&slug); 
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            _tag.name.clone() + &" | Работы тега".to_string(),
            _tag.name.clone() + &" | вебсервисы.рф: Работы тега".to_string(),
            "/tag_works/".to_string() + &slug + &"/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
            t, 
            l,
        ).await
    }
    else {
        use crate::schema::tags_items::dsl::tags_items;
        use crate::models::{Item, Work};
        use crate::utils::get_page;

        let page = get_page(&req);

        let _tag_items = tags_items
            .filter(schema::tags_items::tag_id.eq(&_tag.id))
            .filter(schema::tags_items::types.eq(5))
            .select(schema::tags_items::item_id)
            .load::<i32>(&_connection)
            .expect("E");

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            let data = Item::get_works_list_for_ids(page, 20, _tag_items, _request_user.is_superuser(), l);
            let (_works, works_count) = data.0;
            let next_page_number = data.1;

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/tags/tag_works.stpl")]
                struct Template {
                    request_user:     User,
                    tag:              Tag,
                    works_list:       Vec<Work>,
                    works_count:      usize,
                    next_page_number: i32,
                    is_ajax:          i32,
                    template_types:   u8,
                    linguage:         u8,
                }
                let body = Template {
                    request_user:     _request_user,
                    tag:              _tag,
                    works_list:       _works,
                    works_count:      works_count,
                    next_page_number: next_page_number,
                    is_ajax:          is_ajax,
                    template_types:   t,
                    linguage:         l,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/tags/tag_works.stpl")]
                struct Template {
                    tag:              Tag,
                    works_list:       Vec<Work>,
                    works_count:      usize,
                    next_page_number: i32,
                    is_ajax:          i32,
                    template_types:   u8,
                    linguage:         u8,
                }
                let body = Template {
                    tag:              _tag,
                    works_list:       _works,
                    works_count:      works_count,
                    next_page_number: next_page_number,
                    is_ajax:          is_ajax,
                    template_types:   t,
                    linguage:         l,
                }

                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
        else {
            let data = Item::get_works_list_for_ids(page, 20, _tag_items, false, l);
            let (_works, works_count) = data.0;
            let next_page_number = data.1;

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/tags/anon_tag_works.stpl")]
                struct Template {
                    tag:              Tag,
                    works_list:       Vec<Work>,
                    works_count:      usize,
                    next_page_number: i32,
                    is_ajax:          i32,
                    template_types:   u8,
                    linguage:         u8,
                }
                let body = Template {
                    tag:              _tag,
                    works_list:       _works,
                    works_count:      works_count,
                    next_page_number: next_page_number,
                    is_ajax:          is_ajax,
                    template_types:   t,
                    linguage:         l,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/tags/anon_tag_works.stpl")]
                struct Template {
                    tag:              Tag,
                    works_list:       Vec<Work>,
                    works_count:      usize,
                    next_page_number: i32,
                    is_ajax:          i32,
                    template_types:   u8,
                    linguage:         u8,
                }
                let body = Template {
                    tag:              _tag,
                    works_list:       _works,
                    works_count:      works_count,
                    next_page_number: next_page_number,
                    is_ajax:          is_ajax,
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

pub async fn tag_helps_page(session: Session, req: HttpRequest, _id: web::Path<String>) -> actix_web::Result<HttpResponse> {
    let _connection = establish_connection();
    let (t, l) = get_all_storage();
    let slug = _id.to_string();
    let _tag = Tag::get_tag_with_slug(&slug); 
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);

    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            _tag.name.clone() + &" | Справки тега".to_string(),
            _tag.name.clone() + &" | вебсервисы.рф: Справки тега".to_string(),
            "/tag_helps/".to_string() + &slug + &"/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
            t, 
            l,
        ).await
    }
    else {
        use crate::schema::tags_items::dsl::tags_items;
        use crate::models::{Item, Help};
        use crate::utils::get_page;

        let page = get_page(&req);

        let _tag_items = tags_items
            .filter(schema::tags_items::tag_id.eq(&_tag.id))
            .filter(schema::tags_items::types.eq(6))
            .select(schema::tags_items::item_id)
            .load::<i32>(&_connection)
            .expect("E");

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            let data = Item::get_helps_list_for_ids(page, 20, _tag_items, _request_user.is_superuser(), l);
            let (_helps, helps_count) = data.0;
            let next_page_number = data.1;

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/tags/tag_helps.stpl")]
                struct Template {
                    request_user:     User,
                    tag:              Tag,
                    helps_list:       Vec<Help>,
                    helps_count:      usize,
                    next_page_number: i32,
                    is_ajax:          i32,
                    template_types:   u8,
                    linguage:         u8,
                }
                let body = Template {
                    request_user:     _request_user,
                    tag:              _tag,
                    helps_list:       _helps,
                    helps_count:      helps_count,
                    next_page_number: next_page_number,
                    is_ajax:          is_ajax,
                    template_types:   t,
                    linguage:         l,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/tags/tag_helps.stpl")]
                struct Template {
                    tag:              Tag,
                    helps_list:       Vec<Help>,
                    helps_count:      usize,
                    next_page_number: i32,
                    is_ajax:          i32,
                    template_types:   u8,
                    linguage:         u8,
                }
                let body = Template {
                    tag:              _tag,
                    helps_list:       _helps,
                    helps_count:      helps_count,
                    next_page_number: next_page_number,
                    is_ajax:          is_ajax,
                    template_types:   t,
                    linguage:         l,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
        else {
            let data = Item::get_helps_list_for_ids(page, 20, _tag_items, false, l);
            let (_helps, helps_count) = data.0;
            let next_page_number = data.1;

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/tags/anon_tag_helps.stpl")]
                struct Template {
                    tag:              Tag,
                    helps_list:       Vec<Help>,
                    helps_count:      usize,
                    next_page_number: i32,
                    is_ajax:          i32,
                    template_types:   u8,
                    linguage:         u8,
                }
                let body = Template {
                    tag:              _tag,
                    helps_list:       _helps,
                    helps_count:      helps_count,
                    next_page_number: next_page_number,
                    is_ajax:          is_ajax,
                    template_types:   t,
                    linguage:         l,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/tags/anon_tag_helps.stpl")]
                struct Template {
                    tag:              Tag,
                    helps_list:       Vec<Help>,
                    helps_count:      usize,
                    next_page_number: i32,
                    is_ajax:          i32,
                    template_types:   u8,
                    linguage:         u8,
                }
                let body = Template {
                    tag:              _tag,
                    helps_list:       _helps,
                    helps_count:      helps_count,
                    next_page_number: next_page_number,
                    is_ajax:          is_ajax,
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

pub async fn tags_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let (t, l) = get_all_storage();
    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Ключевые слова".to_string(),
            "вебсервисы.рф: Ключевые слова".to_string(),
            "/tags/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
            t, 
            l,
        ).await
    }
    else {
        let page = crate::utils::get_page(&req);
        let (all_tags, next_page_number) = Tag::get_tags_list(page, 20);
        let tags_count = all_tags.len();

        let _stat = crate::models::StatPage::get_or_create(31);

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/tags/tags.stpl")]
                struct Template {
                    request_user:     User,
                    all_tags:         Vec<SmallTag>,
                    tags_count:       usize,
                    next_page_number: i32,
                    is_ajax:          i32,
                    stat:             StatPage,
                    template_types:   u8,
                    linguage:         u8,
                }
                let body = Template {
                    request_user:     _request_user,
                    all_tags:         all_tags,
                    tags_count:       tags_count,
                    next_page_number: next_page_number,
                    is_ajax:          is_ajax,
                    stat:             _stat,
                    template_types:   t,
                    linguage:         l,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/tags/tags.stpl")]
                struct Template {
                    all_tags:         Vec<SmallTag>,
                    tags_count:       usize,
                    next_page_number: i32,
                    is_ajax:          i32,
                    stat:             StatPage,
                    template_types:   u8,
                    linguage:         u8,
                }
                let body = Template {
                    all_tags:         all_tags,
                    tags_count:       tags_count,
                    next_page_number: next_page_number,
                    is_ajax:          is_ajax,
                    stat:             _stat,
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
                #[template(path = "desctop/tags/anon_tags.stpl")]
                struct Template {
                    all_tags:         Vec<SmallTag>,
                    tags_count:       usize,
                    next_page_number: i32,
                    is_ajax:          i32,
                    stat:             StatPage,
                    template_types:   u8,
                    linguage:         u8,
                }
                let body = Template {
                    all_tags:         all_tags,
                    tags_count:       tags_count,
                    next_page_number: next_page_number,
                    is_ajax:          is_ajax,
                    stat:             _stat,
                    template_types:   t,
                    linguage:         l,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/tags/anon_tags.stpl")]
                struct Template {
                    all_tags:         Vec<SmallTag>,
                    tags_count:       usize,
                    next_page_number: i32,
                    is_ajax:          i32,
                    stat:             StatPage,
                    template_types:   u8,
                    linguage:         u8,
                }
                let body = Template {
                    all_tags:         all_tags,
                    tags_count:       tags_count,
                    next_page_number: next_page_number,
                    is_ajax:          is_ajax,
                    stat:             _stat,
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

pub async fn edit_tag_page(session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;
    use schema::tags::dsl::tags;

    let _tag_id: i32 = *_id;
    let (t, l) = get_all_storage();
    let _connection = establish_connection();
    let _tag = tags
        .filter(schema::tags::id.eq(&_tag_id))
        .first::<Tag>(&_connection)
        .expect("E");

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    if is_ajax == 0 {
        get_first_load_page (
            &session,
            is_desctop,
            "Изменение тега ".to_string() + &_tag.name,
            "вебсервисы.рф: Изменение тега ".to_string() + &_tag.name,
            "/edit_tag/".to_string() + &_tag.id.to_string() + &"/".to_string(),
            "/static/images/dark/store.jpg".to_string(),
            t, 
            l,
        ).await
    }
    else if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/tags/edit_tag.stpl")]
                struct Template {
                    request_user:   User,
                    tag:            Tag,
                    is_ajax:        i32,
                    template_types: u8,
                    linguage:       u8,
                }
                let body = Template {
                    request_user:   _request_user,
                    tag:            _tag,
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
                #[template(path = "mobile/tags/edit_tag.stpl")]
                struct Template {
                    tag:            Tag,
                    is_ajax:        i32,
                    template_types: u8,
                    linguage:       u8,
                }
                let body = Template {
                    tag:            _tag,
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
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Permission Denied."))
        }
    }
    else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Permission Denied."))
    }
}

pub async fn edit_tag(session: Session, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let form = crate::utils::category_form(payload.borrow_mut(), _request_user.id).await;
            Tag::update_tag_with_id(*_id, form);
        }
    }

    HttpResponse::Ok()
}

pub async fn delete_tag(session: Session, _id: web::Path<i32>) -> impl Responder {

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let _connection = establish_connection();
            diesel::delete(schema::tags_items::table.filter(schema::tags_items::tag_id.eq(*_id))).execute(&_connection).expect("E");
            diesel::delete(schema::tags::table.filter(schema::tags::id.eq(*_id))).execute(&_connection).expect("E");
        }
    }
    HttpResponse::Ok()
}
