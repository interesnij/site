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
    config.route("/test/", web::get().to(test_page));
    config.route("/", web::get().to(index_page));
    config.route("/info/", web::get().to(info_page));
    config.route("/history/", web::get().to(history_page));
    config.route("/feedback_list/", web::get().to(feedback_list_page));
    config.route("/serve_list/", web::get().to(serve_list_page));
    config.route("/cookie_users_list/", web::get().to(cookie_users_list_page));

    config.route("/load_tech_category/{id}/", web::get().to(get_tech_category_page));
    config.route("/load_serve_category/{id}/", web::get().to(get_serve_category_page));
    config.route("/load_serve/{id}/", web::get().to(get_serve_page));
    config.route("/load_feedback/", web::get().to(get_feedback_page));
    config.route("/load_user_history/{id}/", web::get().to(get_user_history_page));
    config.route("/load_tech_objects/{id}/", web::get().to(get_tech_objects_page));
    config.route("/unical_object_form/{id}/", web::get().to(unical_object_form_page));

    config.route("/create_category/", web::get().to(create_category_page));
    config.route("/edit_category/{id}/", web::get().to(edit_category_page));
    config.route("/create_item/", web::get().to(create_item_page));
    config.route("/edit_item/{id}/", web::get().to(edit_item_page));
    config.route("/edit_content_item/{id}/", web::get().to(edit_content_item_page));

    config.route("/edit_file/{id}/", web::get().to(edit_file_page));
    config.route("/image/{id}/", web::get().to(image_page));
}


pub async fn not_found(conn: ConnectionInfo, req: HttpRequest, session: Session) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    let (l, t) = crate::utils::get_or_create_c_user_return_lt(conn, &req).await;

    let title: String;
    let description: String;
    let link = "/not_found/".to_string();
    let image = "/static/images/dark/store.jpg".to_string();
    if l == 2 {
        title = "Page not found".to_string();
        description = "Web-services: Page not found".to_string();
    }
    else {
        title = "Страница не найдена".to_string();
        description = "вебсервисы.рф: Страница не найдена".to_string();
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
    else {
        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/pages/404.stpl")]
                struct Template {
                    request_user:   User,
                    is_ajax:        i32,
                    template_types: i16,
                    linguage:       i16,
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
                #[template(path = "mobile/pages/404.stpl")]
                struct Template {
                    is_ajax:        i32,
                    template_types: i16,
                    linguage:       i16,
                    title:          String,
                    description:    String,
                    link:           String,
                    image:          String,
                }
                let body = Template {
                    is_ajax:        is_ajax,
                    template_types: t,
                    linguage:       l,
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
        else {
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/pages/anon_404.stpl")]
                struct Template {
                    is_ajax:        i32,
                    template_types: i16,
                    linguage:       i16,
                    title:          String,
                    description:    String,
                    link:           String,
                    image:          String,
                }
                let body = Template {
                    is_ajax:        is_ajax,
                    template_types: t,
                    linguage:       l,
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
                #[template(path = "mobile/pages/anon_404.stpl")]
                struct Template {
                    is_ajax:        i32,
                    template_types: i16,
                    linguage:       i16,
                    title:          String,
                    description:    String,
                    link:           String,
                    image:          String,
                }
                let body = Template {
                    is_ajax:        is_ajax,
                    template_types: t,
                    linguage:       l,
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

pub async fn test_page(state: web::Data<AppState>) -> Result<web::Json<IndexResponse>> {
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);
    let ms = state.messages.lock().unwrap();

    Ok(web::Json(IndexResponse {
        server_id: state.server_id,
        request_count,
        messages: ms.clone(),
    }))
}

pub async fn index_page (
    req: HttpRequest,
    session: Session,
    conn: ConnectionInfo, 
) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let (l, t) = crate::utils::get_or_create_c_user_return_lt(conn, &req).await;

    let title: String;
    let description: String;
    let link = "/".to_string();
    let image = "/static/images/dark/store.jpg".to_string();
    if l == 2 {
        title = "Home".to_string();
        description = "Web-services - Comprehensive, expert creation and development of highly loaded web resources".to_string();
    }
    else {
        title = "Главная страница".to_string();
        description = "вебсервисы.рф - Комплексное, экспертное создание и развитие высоконагруженных веб-ресурсов".to_string();
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
    else {
        use crate::models::{Blog, Service, Store, Wiki, Work};

        let _stat = crate::models::StatPage::get_or_create(1);
        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            let is_admin = _request_user.is_superuser();
            //User::create_superuser(_request_user.id);
            let (_last_works, work_count) = Item::get_works(3, 0, is_admin, l); 
            let (_last_services, service_count) = Item::get_services(3, 0, is_admin, l);
            let (_last_wikis, wiki_count) = Item::get_wikis(3, 0, is_admin, l);
            let (_last_blogs, blog_count) = Item::get_blogs(3, 0, is_admin, l);
            let (_last_stores, store_count) = Item::get_stores(3, 0, is_admin, l);

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/main/mainpage.stpl")]
                struct Template {
                    //request_user:   User,
                    last_works:     Vec<Work>,
                    last_services:  Vec<Service>,
                    last_wikis:     Vec<Wiki>,
                    last_blogs:     Vec<Blog>,
                    last_stores:    Vec<Store>,
                    works_count:    usize,
                    services_count: usize,
                    wikis_count:    usize,
                    blogs_count:    usize,
                    stores_count:   usize,
                    is_ajax:        i32,
                    stat:           StatPage,
                    template_types: i16,
                    linguage:       i16,
                    title:          String,
                    description:    String,
                    link:           String,
                    image:          String,
                }
                let body = Template {
                    //request_user:   _request_user,
                    last_works:     _last_works,
                    last_services:  _last_services,
                    last_wikis:     _last_wikis,
                    last_blogs:     _last_blogs,
                    last_stores:    _last_stores,
                    works_count:    work_count,
                    services_count: service_count,
                    wikis_count:    wiki_count,
                    blogs_count:    blog_count,
                    stores_count:   store_count,
                    is_ajax:        is_ajax,
                    stat:           _stat,
                    template_types: t,
                    linguage:       l,
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
                #[template(path = "mobile/main/mainpage.stpl")]
                struct Template {
                    //request_user:   User,
                    last_works:     Vec<Work>,
                    last_services:  Vec<Service>,
                    last_wikis:     Vec<Wiki>,
                    last_blogs:     Vec<Blog>,
                    last_stores:    Vec<Store>,
                    works_count:    usize,
                    services_count: usize,
                    wikis_count:    usize,
                    blogs_count:    usize,
                    stores_count:   usize,
                    is_ajax:        i32,
                    stat:           StatPage,
                    template_types: i16,
                    linguage:       i16,
                    title:          String,
                    description:    String,
                    link:           String,
                    image:          String,
                }
                let body = Template {
                    //request_user:   _request_user,
                    last_works:     _last_works,
                    last_services:  _last_services,
                    last_wikis:     _last_wikis,
                    last_blogs:     _last_blogs,
                    last_stores:    _last_stores,
                    works_count:    work_count,
                    services_count: service_count,
                    wikis_count:    wiki_count,
                    blogs_count:    blog_count,
                    stores_count:   store_count,
                    is_ajax:        is_ajax,
                    stat:           _stat,
                    template_types: t,
                    linguage:       l,
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
        else {
            let (_last_works, work_count) = Item::get_works(3, 0, false, l); 
            let (_last_services, service_count) = Item::get_services(3, 0, false, l);
            let (_last_wikis, wiki_count) = Item::get_wikis(3, 0, false, l);
            let (_last_blogs, blog_count) = Item::get_blogs(3, 0, false, l);
            let (_last_stores, store_count) = Item::get_stores(3, 0, false, l);

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/main/anon_mainpage.stpl")]
                struct Template {
                    last_works:     Vec<Work>,
                    last_services:  Vec<Service>,
                    last_wikis:     Vec<Wiki>,
                    last_blogs:     Vec<Blog>,
                    last_stores:    Vec<Store>,
                    works_count:    usize,
                    services_count: usize,
                    wikis_count:    usize,
                    blogs_count:    usize,
                    stores_count:   usize,
                    is_ajax:        i32,
                    stat:           StatPage,
                    //template_types: i16,
                    linguage:       i16,
                    title:          String,
                    description:    String,
                    link:           String,
                    image:          String,
                }
                let body = Template {
                    last_works:     _last_works,
                    last_services:  _last_services,
                    last_wikis:     _last_wikis,
                    last_blogs:     _last_blogs,
                    last_stores:    _last_stores,
                    works_count:    work_count,
                    services_count: service_count,
                    wikis_count:    wiki_count,
                    blogs_count:    blog_count,
                    stores_count:   store_count,
                    is_ajax:        is_ajax,
                    stat:           _stat,
                    //template_types: t,
                    linguage:       l,
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
                #[template(path = "mobile/main/anon_mainpage.stpl")]
                struct Template {
                    last_works:     Vec<Work>,
                    last_services:  Vec<Service>,
                    last_wikis:     Vec<Wiki>,
                    last_blogs:     Vec<Blog>,
                    last_stores:    Vec<Store>,
                    works_count:    usize,
                    services_count: usize,
                    wikis_count:    usize,
                    blogs_count:    usize,
                    stores_count:   usize,
                    is_ajax:        i32,
                    stat:           StatPage,
                    //template_types: i16,
                    linguage:       i16,
                    title:          String,
                    description:    String,
                    link:           String,
                    image:          String,
                }
                let body = Template {
                    last_works:     _last_works,
                    last_services:  _last_services,
                    last_wikis:     _last_wikis,
                    last_blogs:     _last_blogs,
                    last_stores:    _last_stores,
                    works_count:    work_count,
                    services_count: service_count,
                    wikis_count:    wiki_count,
                    blogs_count:    blog_count,
                    stores_count:   store_count,
                    is_ajax:        is_ajax,
                    stat:           _stat,
                    //template_types: t,
                    linguage:       l,
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

pub async fn info_page(conn: ConnectionInfo, req: HttpRequest, session: Session) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let (l, t) = crate::utils::get_or_create_c_user_return_lt(conn, &req).await;

    let title: String;
    let description: String;
    let link = "/info/".to_string();
    let image = "/static/images/dark/store.jpg".to_string();
    if l == 2 {
        title = "Info".to_string();
        description = "Web-services - Info".to_string();
    }
    else {
        title = "Информация".to_string();
        description = "вебсервисы.рф - Информация о нас, о сайте, контакты, вкладка помощи".to_string();
    }

    if is_ajax == 0 {
        return crate::utils::get_first_load_page (
            &session,
            is_desctop,
            &title,
            &description,
            &link,
            &image,
            t, 
        ).await;
    }

    let _stat = crate::models::StatPage::get_or_create(10);
    let _help_cats = block(move || Categories::get_categories_for_types(6, l)).await?;
    if is_signed_in(&session) { 
        let _request_user = get_request_user_data(&session);
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/pages/info.stpl")]
            struct Template {
                request_user:   User,
                is_ajax:        i32,
                help_cats:      Vec<Cat>,
                stat:           StatPage,
                template_types: i16,
                linguage:       i16,
                title:          String,
                description:    String,
                link:           String,
                image:          String,
            }
            let body = Template {
                request_user:   _request_user,
                is_ajax:        is_ajax,
                help_cats:      _help_cats,
                stat:           _stat,
                template_types: t,
                linguage:       l,
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
            #[template(path = "mobile/pages/info.stpl")]
            struct Template {
                is_ajax:        i32,
                help_cats:      Vec<Cat>,
                stat:           StatPage,
                template_types: i16,
                linguage:       i16,
                title:          String,
                description:    String,
                link:           String,
                image:          String,
            }
            let body = Template {
                is_ajax:        is_ajax,
                help_cats:      _help_cats,
                stat:           _stat,
                template_types: t,
                linguage:       l,
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
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/pages/anon_info.stpl")]
            struct Template {
                is_ajax:        i32,
                help_cats:      Vec<Cat>,
                stat:           StatPage,
                template_types: i16,
                linguage:       i16,
                title:          String,
                description:    String,
                link:           String,
                image:          String,
            }
            let body = Template {
                is_ajax:        is_ajax,
                help_cats:      _help_cats,
                stat:           _stat,
                template_types: t,
                linguage:       l,
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
            #[template(path = "mobile/pages/anon_info.stpl")]
            struct Template {
                help_cats:      Vec<Cat>,
                is_ajax:        i32,
                stat:           StatPage,
                template_types: i16,
                linguage:       i16,
                title:          String,
                description:    String,
                link:           String,
                image:          String,
            }
            let body = Template {
                is_ajax:        is_ajax,
                help_cats:      _help_cats,
                stat:           _stat,
                template_types: t,
                linguage:       l,
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

pub async fn history_page(conn: ConnectionInfo, req: HttpRequest, session: Session) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    
    let (l, t, user_id) = crate::utils::get_or_create_c_user_return_lti(conn, &req).await;

    let title: String;
    let description: String;
    let link = "/history/".to_string();
    let image = "/static/images/dark/store.jpg".to_string();
    if l == 2 {
        title = "Your browsing history".to_string();
        description = "Web-services - Your browsing history".to_string();
    }
    else {
        title = "История просмотров".to_string();
        description = "вебсервисы.рф - История просмотров".to_string();
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
    else {
        use crate::models::{CookieUser, CookieStat};

        let _cookie_user = CookieUser::get(user_id);
        let object_list: Vec<CookieStat>;
        let next_page_number: i32; 
        let page = crate::utils::get_page(&req);
        let _res = block(move || CookieStat::get_stat_list(user_id, page, 20)).await?;
        let _dict = match _res {
            Ok(_ok) => {object_list = _ok.0; next_page_number = _ok.1},
            Err(_error) => {object_list = Vec::new(); next_page_number = 0},
        };

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/pages/history.stpl")]
                struct Template {
                    request_user:     User,
                    user:             CookieUser,
                    object_list:      Vec<CookieStat>,
                    is_ajax:          i32,
                    next_page_number: i32,
                    template_types:   i16,
                    linguage:         i16,
                    title:            String,
                    description:      String,
                    link:             String,
                    image:            String,

                }
                let body = Template {
                    request_user:     _request_user,
                    user:             _cookie_user,
                    object_list:      object_list,
                    is_ajax:          is_ajax,
                    next_page_number: next_page_number,
                    template_types:   t,
                    linguage:         l,
                    title:            title,
                    description:      description,
                    link:             link,
                    image:            image,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/pages/history.stpl")]
                struct Template {
                    user:             CookieUser,
                    object_list:      Vec<CookieStat>,
                    is_ajax:          i32,
                    next_page_number: i32,
                    template_types:   i16,
                    linguage:         i16,
                    title:            String,
                    description:      String,
                    link:             String,
                    image:            String,
                }
                let body = Template {
                    user:             _cookie_user,
                    object_list:      object_list,
                    is_ajax:          is_ajax,
                    next_page_number: next_page_number,
                    template_types:   t,
                    linguage:         l,
                    title:            title,
                    description:      description,
                    link:             link,
                    image:            image,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
        else {
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/pages/anon_history.stpl")]
                struct Template {
                    user:             CookieUser,
                    object_list:      Vec<CookieStat>,
                    is_ajax:          i32,
                    next_page_number: i32,
                    template_types:   i16,
                    linguage:         i16,
                    title:            String,
                    description:      String,
                    link:             String,
                    image:            String,
                }
                let body = Template {
                    user:             _cookie_user,
                    object_list:      object_list,
                    is_ajax:          is_ajax,
                    next_page_number: next_page_number,
                    template_types:   t,
                    linguage:         l,
                    title:            title,
                    description:      description,
                    link:             link,
                    image:            image,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/pages/anon_history.stpl")]
                struct Template {
                    user:             CookieUser,
                    object_list:      Vec<CookieStat>,
                    is_ajax:          i32,
                    next_page_number: i32,
                    template_types:   i16,
                    linguage:         i16,
                    title:            String,
                    description:      String,
                    link:             String,
                    image:            String,
                }
                let body = Template {
                    user:             _cookie_user,
                    object_list:      object_list,
                    is_ajax:          is_ajax,
                    next_page_number: next_page_number,
                    template_types:   t,
                    linguage:         l,
                    title:            title,
                    description:      description,
                    link:             link,
                    image:            image,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}

pub async fn feedback_list_page(conn: ConnectionInfo, req: HttpRequest, session: Session) -> actix_web::Result<HttpResponse> {
        if !is_signed_in(&session) {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Permission Denied"))
        }
        else {
            use crate::models::Feedback;

            let (l, t) = crate::utils::get_or_create_c_user_return_lt(conn, &req).await;

            let title: String;
            let description: String;
            let link = "/feedback_list/".to_string();
            let image = "/static/images/dark/store.jpg".to_string();
            if l == 2 {
                title = "Feedback list".to_string();
                description = "Web-services: Feedback list".to_string();
            }
            else { 
                title = "Письма пользователей".to_string();
                description = "вебсервисы.рф: Письма пользователей".to_string();
            }

            let _feedbacks = Feedback::get_all();

            let _request_user = get_request_user_data(&session);
            let (is_desctop, is_ajax) = get_device_and_ajax(&req);
            if _request_user.perm < 60 {
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Permission Denied"))
            }
            else if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/main/feedback_list.stpl")]
                struct Template { 
                    request_user:   User,
                    is_ajax:        i32,
                    feedback_list:  Vec<Feedback>,
                    template_types: i16,
                    linguage:       i16,
                    title:          String,
                    description:    String,
                    link:           String,
                    image:          String,
                }
                let body = Template {
                    request_user:   _request_user,
                    is_ajax:        is_ajax,
                    feedback_list:  _feedbacks,
                    template_types: t,
                    linguage:       l,
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
                #[template(path = "mobile/main/feedback_list.stpl")]
                struct Template {
                    is_ajax:        i32,
                    feedback_list:  Vec<Feedback>,
                    template_types: i16,
                    linguage:       i16,
                    title:          String,
                    description:    String,
                    link:           String,
                    image:          String,
                }
                let body = Template {
                    is_ajax:        is_ajax,
                    feedback_list:  _feedbacks,
                    template_types: t,
                    linguage:       l,
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

pub async fn serve_list_page(conn: ConnectionInfo, req: HttpRequest, session: Session) -> actix_web::Result<HttpResponse> {
    use crate::models::TechCategories;
    let (l, t) = crate::utils::get_or_create_c_user_return_lt(conn, &req).await;
    let all_tech_categories = TechCategories::get_all();

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    let title: String;
    let description: String;
    let link = "/serve_list/".to_string();
    let image = "/static/images/dark/store.jpg".to_string();
    if l == 2 {
        title = "List of options and services".to_string();
        description = "Web-services - List of options and services".to_string();
    }
    else {
        title = "Список опций и услуг".to_string();
        description = "вебсервисы.рф - Список опций и услуг".to_string();
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
    else if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/main/serve_list.stpl")]
            struct Template {
                request_user:   User,
                is_ajax:        i32,
                tech_cats:      Vec<TechCategories>,
                template_types: i16,
                linguage:       i16,
                title:          String,
                description:    String,
                link:           String,
                image:          String,
            }
            let body = Template {
                request_user:   _request_user,
                is_ajax:        is_ajax,
                tech_cats:      all_tech_categories,
                template_types: t,
                linguage:       l,
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
            #[template(path = "mobile/main/serve_list.stpl")]
            struct Template {
                request_user:   User,
                is_ajax:        i32,
                tech_cats:      Vec<TechCategories>,
                template_types: i16,
                linguage:       i16,
                title:          String,
                description:    String,
                link:           String,
                image:          String,
            }
            let body = Template {
                request_user:   _request_user,
                is_ajax:        is_ajax,
                tech_cats:      all_tech_categories,
                template_types: t,
                linguage:       l,
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
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/main/anon_serve_list.stpl")]
            struct Template {
                is_ajax:        i32,
                tech_cats:      Vec<TechCategories>,
                template_types: i16,
                linguage:       i16,
                title:          String,
                description:    String,
                link:           String,
                image:          String,
            }
            let body = Template {
                is_ajax:        is_ajax,
                tech_cats:      all_tech_categories,
                template_types: t,
                linguage:       l,
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
            #[template(path = "mobile/main/anon_serve_list.stpl")]
            struct Template {
                is_ajax:        i32,
                tech_cats:      Vec<TechCategories>,
                template_types: i16,
                linguage:       i16,
                title:          String,
                description:    String,
                link:           String,
                image:          String,
            }
            let body = Template {
                is_ajax:        is_ajax,
                tech_cats:      all_tech_categories,
                template_types: t,
                linguage:       l,
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

pub async fn get_tech_category_page(req: HttpRequest, conn: ConnectionInfo, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::models::TechCategories;

    let (l, t) = crate::utils::get_or_create_c_user_return_lt(conn, &req).await;
    let tech_category = TechCategories::get(*_id);

    let title: String;
    let description: String;
    let link = "/load_tech_category/".to_string() + &tech_category.name_en + &"/".to_string();
    let image = "/static/images/dark/store.jpg".to_string();
    if l == 2 {
        title = "Web service ".to_string() + &tech_category.name_en;
        description = "Web service ".to_string() + &tech_category.name_en + &" : Web-services".to_string();
    }
    else {
        title = "Веб-сервис ".to_string() + &tech_category.name_en;
        description = "Веб-сервис ".to_string() + &tech_category.name_en + &" : вебсервисы.рф".to_string();
    }

    #[derive(TemplateOnce)]
    #[template(path = "desctop/load/tech_category.stpl")]
    struct Template {
        object:         TechCategories,
        template_types: i16,
        linguage:       i16,
        title:          String,
        description:    String,
        link:           String,
        image:          String,
    }
    let body = Template {
        object:         tech_category,
        template_types: t,
        linguage:       l,
        title:          title,
        description:    description,
        link:           link,
        image:          image,
    }
    .render_once()
    .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
}

pub async fn get_serve_category_page(req: HttpRequest, conn: ConnectionInfo, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::models::ServeCategories;

    let (l, t) = crate::utils::get_or_create_c_user_return_lt(conn, &req).await;
    let serve_category = ServeCategories::get(*_id);

    let title: String;
    let description: String;
    let link = "/load_serve_category/".to_string() + &serve_category.name_en + &"/".to_string();
    let image = "/static/images/dark/store.jpg".to_string();
    if l == 2 {
        title = "Category of serve ".to_string() + &serve_category.name_en;
        description = "Category of serve ".to_string() + &serve_category.name_en + &" : Web-services".to_string();
    }
    else {
        title = "Опция ".to_string() + &serve_category.name_en;
        description = "Опция ".to_string() + &serve_category.name_en + &" : вебсервисы.рф".to_string();
    }

    #[derive(TemplateOnce)]
    #[template(path = "desctop/load/serve_category.stpl")]
    struct Template {
        object:         ServeCategories,
        template_types: i16,
        linguage:       i16,
        title:          String,
        description:    String,
        link:           String,
        image:          String,
    }
    let body = Template {
        object:         serve_category,
        template_types: t,
        linguage:       l,
        title:          title,
        description:    description,
        link:           link,
        image:          image,
    }
    .render_once()
    .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
}

pub async fn get_serve_page(req: HttpRequest, conn: ConnectionInfo, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::models::Serve;

    let (l, t) = crate::utils::get_or_create_c_user_return_lt(conn, &req).await;
    let _serve = Serve::get(*_id);

    let title: String;
    let description: String;
    let link = "/load_serve/".to_string() + &_serve.name_en + &"/".to_string();
    let image = "/static/images/dark/store.jpg".to_string();
    if l == 2 {
        title = "Serve ".to_string() + &_serve.name_en;
        description = "Serve ".to_string() + &_serve.name_en + &" : Web-services".to_string();
    }
    else {
        title = "Опция ".to_string() + &_serve.name_en;
        description = "Опция ".to_string() + &_serve.name_en + &" : вебсервисы.рф".to_string();
    }

    #[derive(TemplateOnce)]
    #[template(path = "desctop/load/serve.stpl")]
    struct Template {
        object:         Serve,
        template_types: i16,
        linguage:       i16,
        title:          String,
        description:    String,
        link:           String,
        image:          String,
    }
    let body = Template {
        object:         _serve,
        template_types: t,
        linguage:       l,
        title:          title,
        description:    description,
        link:           link,
        image:          image,
    }
    .render_once()
    .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
}

pub async fn get_feedback_page(req: HttpRequest, conn: ConnectionInfo) -> actix_web::Result<HttpResponse> {
    let (l, t) = crate::utils::get_or_create_c_user_return_lt(conn, &req).await;
    let title: String;
    let description: String;
    let link = "/load_feedback/".to_string();
    let image = "/static/images/dark/store.jpg".to_string();
    if l == 2 {
        title = "Load feedback list".to_string();
        description = "Web-services - Load feedback list".to_string();
    }
    else {
        title = "Письма пользователей".to_string();
        description = "вебсервисы.рф - Письма пользователей".to_string();
    }

    #[derive(TemplateOnce)]
    #[template(path = "desctop/load/feedback.stpl")]
    struct Template {
        template_types: i16,
        linguage:       i16,
        title:          String,
        description:    String,
        link:           String,
        image:          String,
    }
    let body = Template {
        template_types: t,
        linguage:       l,
        title:          title,
        description:    description,
        link:           link,
        image:          image,
    }
    .render_once()
    .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
}

pub async fn cookie_users_list_page(conn: ConnectionInfo, session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_page;
    use crate::models::CookieUser;

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let (l, t) = crate::utils::get_or_create_c_user_return_lt(conn, &req).await;

    let title: String;
    let description: String;
    let link = "/cookie_users_list/".to_string();
    let image = "/static/images/dark/store.jpg".to_string();
    if l == 2 {
        title = "General site statistics".to_string();
        description = "Web-services - General site statistics".to_string();
    }
    else {
        title = "Общая статистика сайта".to_string();
        description = "вебсервисы.рф - Общая статистика сайта".to_string();
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
    else {
        let (object_list, next_page_number) = CookieUser::get_users_list(get_page(&req), 20);

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/pages/stat.stpl")]
                struct Template {
                    request_user:     User,
                    object_list:      Vec<CookieUser>,
                    next_page_number: i32,
                    is_ajax:          i32,
                    template_types:   i16,
                    linguage:         i16,
                    title:            String,
                    description:      String,
                    link:             String,
                    image:            String,
                }
                let body = Template {
                    request_user:     _request_user,
                    object_list:      object_list,
                    next_page_number: next_page_number,
                    is_ajax:          is_ajax,
                    template_types:   t,
                    linguage:         l,
                    title:            title,
                    description:      description,
                    link:             link,
                    image:            image,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/pages/stat.stpl")]
                struct Template {
                    request_user:     User,
                    object_list:      Vec<CookieUser>,
                    next_page_number: i32,
                    is_ajax:          i32,
                    template_types:   i16,
                    linguage:         i16,
                    title:            String,
                    description:      String,
                    link:             String,
                    image:            String,
                }
                let body = Template {
                    request_user:     _request_user,
                    object_list:      object_list,
                    next_page_number: next_page_number,
                    is_ajax:          is_ajax,
                    template_types:   t,
                    linguage:         l,
                    title:            title,
                    description:      description,
                    link:             link,
                    image:            image,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
        else {
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/pages/anon_stat.stpl")]
                struct Template {
                    object_list:      Vec<CookieUser>,
                    next_page_number: i32,
                    is_ajax:          i32,
                    template_types:   i16,
                    linguage:         i16,
                    title:            String,
                    description:      String,
                    link:             String,
                    image:            String,
                }
                let body = Template {
                    object_list:      object_list,
                    next_page_number: next_page_number,
                    is_ajax:          is_ajax,
                    template_types:   t,
                    linguage:         l,
                    title:            title,
                    description:      description,
                    link:             link,
                    image:            image,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/pages/anon_stat.stpl")]
                struct Template {
                    object_list:      Vec<CookieUser>,
                    next_page_number: i32,
                    is_ajax:          i32,
                    template_types:   i16,
                    linguage:         i16,
                    title:            String,
                    description:      String,
                    link:             String,
                    image:            String,
                }
                let body = Template {
                    object_list:      object_list,
                    next_page_number: next_page_number,
                    is_ajax:          is_ajax,
                    template_types:   t,
                    linguage:         l,
                    title:            title,
                    description:      description,
                    link:             link,
                    image:            image,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}

pub async fn get_user_history_page(conn: ConnectionInfo, session: Session, req: HttpRequest, user_id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        let (l, t) = crate::utils::get_or_create_c_user_return_lt(conn, &req).await;
        if _request_user.is_superuser() {

            use crate::utils::get_page;
            use crate::models::CookieStat;

            let user_id_str = _request_user.id.to_string();
            let title: String;
            let description: String;
            let link = "/load_user_history/".to_string() + &user_id_str + &"/".to_string();
            let image = "/static/images/dark/store.jpg".to_string();
            if l == 2 {
                title = "User history ".to_string() + &user_id_str;
                description = "User history ".to_string() + &user_id_str + &" : Web-services".to_string();
            }
            else {
                title = "История просмотров пользователя ".to_string() + &user_id_str;
                description = "Веб-сервис ".to_string() + &user_id_str + &" : вебсервисы.рф".to_string();
            }


            let object_list: Vec<CookieStat>;
            let next_page_number: i32;
            let page = get_page(&req);
            let _res = block(move || CookieStat::get_stat_list(*user_id, page, 20)).await?;
            let _dict = match _res {
                Ok(_ok) => {object_list = _ok.0; next_page_number = _ok.1},
                Err(_error) => {object_list = Vec::new(); next_page_number = 0},
            };

            #[derive(TemplateOnce)]
            #[template(path = "desctop/load/user_stat.stpl")]
            struct Template {
                object_list:      Vec<CookieStat>,
                next_page_number: i32,
                template_types:   i16,
                linguage:         i16,
                title:            String,
                description:      String,
                link:             String,
                image:            String,
            }
            let body = Template {
                object_list:      object_list,
                next_page_number: next_page_number,
                template_types:   t,
                linguage:         l,
                title:            title,
                description:      description,
                link:             link,
                image:            image,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Permission Denied"))
        }
    }
    else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Permission Denied"))
    }
}

pub async fn get_tech_objects_page(req: HttpRequest, conn: ConnectionInfo, session: Session, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::models::TechCategories;

    let _cat = TechCategories::get(*_id);
    let (l, t) = crate::utils::get_or_create_c_user_return_lt(conn, &req).await;

    let title: String;
    let description: String;
    let link = "/load_tech_objects/".to_string() + &_cat.name_en + &"/".to_string();
    let image = "/static/images/dark/store.jpg".to_string();
    if l == 2 {
        title = "Web-service ".to_string() + &_cat.name_en.to_string();
        description = "Web-service ".to_string() + &_cat.name_en + &" : Web-services".to_string();
    }
    else {
        title = "Веб-сервис ".to_string() + &_cat.name;
        description = "Веб-сервис ".to_string() + &_cat.name + &" : вебсервисы.рф".to_string();
    }


    let mut is_admin = false;
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.is_superuser() {
            is_admin = true;
        }
    }

    #[derive(TemplateOnce)]
    #[template(path = "desctop/load/tech_category_objects.stpl")]
    struct Template {
        object:         TechCategories,
        is_admin:       bool,
        template_types: i16,
        linguage:       i16,
        title:          String,
        description:    String,
        link:           String,
        image:          String,
    }
    let body = Template {
        object:         _cat,
        is_admin:       is_admin,
        template_types: t,
        linguage:       l,
        title:          title,
        description:    description,
        link:           link,
        image:          image,
    }
    .render_once()
    .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
}

pub async fn unical_object_form_page(req: HttpRequest, conn: ConnectionInfo, session: Session, _id: web::Path<i16>) -> actix_web::Result<HttpResponse> {
    let (l, t) = crate::utils::get_or_create_c_user_return_lt(conn, &req).await;
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if !_request_user.is_superuser() {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Permission Denied"))
        }
        else {
            let types = *_id;
            let mut biznes_mode = false;
            if vec![2,3,5].iter().any(|i| i==&types) {
                biznes_mode = true;
            }
            let _cats = block(move || Categories::get_categories_for_types(types, l)).await?;

            #[derive(TemplateOnce)]
            #[template(path = "desctop/load/unical_object_form.stpl")]
            struct Template {
                cats:           Vec<Cat>,
                biznes_mode:    bool,
                template_types: i16,
                linguage:       i16,
            }
            let body = Template {
                cats:           _cats,
                biznes_mode:    biznes_mode,
                template_types: t,
                linguage:       l,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Permission Denied"))
    }
}

pub async fn create_category_page(conn: ConnectionInfo, session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let (l, t) = crate::utils::get_or_create_c_user_return_lt(conn, &req).await;

    let title: String;
    let description: String;
    let link = "/create_category/".to_string();
    let image = "/static/images/dark/store.jpg".to_string();
    if l == 2 {
        title = "Creating a category".to_string();
        description = "Web-services - Creating a category".to_string();
    }
    else {
        title = "Создание категории".to_string();
        description = "вебсервисы.рф - Создание категории".to_string();
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
    else if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 { 
            let _cats = Categories::get_all();

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/pages/create_category.stpl")]
                struct Template {
                    request_user:   User,
                    cats:           Vec<Categories>,
                    is_ajax:        i32,
                    template_types: i16,
                    linguage:       i16,
                    title:          String,
                    description:    String,
                    link:           String,
                    image:          String,
                }
                let body = Template {
                    request_user:   _request_user,
                    cats:           _cats,
                    is_ajax:        is_ajax,
                    template_types: t,
                    linguage:       l,
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
                #[template(path = "mobile/pages/create_category.stpl")]
                struct Template {
                    cats:           Vec<Categories>,
                    is_ajax:        i32,
                    template_types: i16,
                    linguage:       i16,
                    title:          String,
                    description:    String,
                    link:           String,
                    image:          String,
                }
                let body = Template {
                    cats:           _cats,
                    is_ajax:        is_ajax,
                    template_types: t,
                    linguage:       l,
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
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Permission Denied."))
        }
    }
    else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Permission Denied."))
    }
}

pub async fn edit_category_page(conn: ConnectionInfo, session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let (l, t) = crate::utils::get_or_create_c_user_return_lt(conn, &req).await;
    let _cat = Categories::get_with_id(*_id);

    let title: String;
    let description: String;
    let link = "/edit_category/".to_string() + &_cat.id.to_string() + &"/".to_string();
    let image = _cat.get_image();
    if l == 2 {
        title = "Creating a category ".to_string() + &_cat.name_en;
        description = "Web-services - Creating a category ".to_string() + &_cat.name_en;
    }
    else {
        title = "Изменение категории ".to_string() + &_cat.name;
        description = "вебсервисы.рф: Изменение категории ".to_string() + &_cat.name;
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
    else if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            let _cats = Categories::get_all();

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/pages/edit_category.stpl")]
                struct Template {
                    request_user:   User,
                    cat:            Categories,
                    cats:           Vec<Categories>,
                    is_ajax:        i32,
                    template_types: i16,
                    linguage:       i16,
                    title:          String,
                    description:    String,
                    link:           String,
                    image:          String,
                }
                let body = Template {
                    request_user:   _request_user,
                    cat:            _cat,
                    cats:           _cats,
                    is_ajax:        is_ajax,
                    template_types: t,
                    linguage:       l,
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
                #[template(path = "mobile/pages/edit_category.stpl")]
                struct Template {
                    cat:            Categories,
                    cats:           Vec<Categories>,
                    is_ajax:        i32,
                    template_types: i16,
                    linguage:       i16,
                    title:          String,
                    description:    String,
                    link:           String,
                    image:          String,
                }
                let body = Template {
                    cat:            _cat,
                    cats:           _cats,
                    is_ajax:        is_ajax,
                    template_types: t,
                    linguage:       l,
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
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Permission Denied."))
        }
    }
    else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Permission Denied."))
    }
}

pub async fn create_item_page(conn: ConnectionInfo, session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let (l, t) = crate::utils::get_or_create_c_user_return_lt(conn, &req).await;

    let title: String;
    let description: String;
    let link = "/create_item/".to_string();
    let image = "/static/images/dark/store.jpg".to_string();
    if l == 2 {
        title = "Creating a object".to_string();
        description = "Web-services - Creating a object".to_string();
    }
    else {
        title = "Создание объекта".to_string();
        description = "вебсервисы.рф - Создание объекта".to_string();
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
    else if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 {
            use crate::models::TechCategories;

            let all_tags = Tag::get_all();
            let _tech_categories = TechCategories::get_all();

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/pages/create_item.stpl")]
                struct Template {
                    request_user:   User,
                    all_tags:       Vec<Tag>,
                    is_ajax:        i32,
                    template_types: i16,
                    linguage:       i16,
                    title:          String,
                    description:    String,
                    link:           String,
                    image:          String,
                }
                let body = Template {
                    request_user:   _request_user,
                    all_tags:       all_tags,
                    is_ajax:        is_ajax,
                    template_types: t,
                    linguage:       l,
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
                #[template(path = "mobile/pages/create_item.stpl")]
                struct Template {
                    all_tags:       Vec<Tag>,
                    is_ajax:        i32,
                    template_types: i16,
                    linguage:       i16,
                    title:          String,
                    description:    String,
                    link:           String,
                    image:          String,
                }
                let body = Template {
                    all_tags:       all_tags,
                    is_ajax:        is_ajax,
                    template_types: t,
                    linguage:       l,
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
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Permission Denied."))
        }
    }
    else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Permission Denied."))
    }
}
pub async fn edit_item_page(conn: ConnectionInfo, session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let (l, t) = crate::utils::get_or_create_c_user_return_lt(conn, &req).await;
    let _item = Item::get_with_id(*_id); 

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    let title: String;
    let description: String;
    let link = "/edit_item/".to_string() + &_item.id.to_string() + &"/".to_string();
    let image = "/static/images/dark/store.jpg".to_string();
    if l == 2 {
        title = "Update a object ".to_string() + &_item.title_en;
        description = "Web-services - Update a object ".to_string() + &_item.title_en;
    }
    else {
        title = "Изменение объекта ".to_string() + &_item.title;
        description = "вебсервисы.рф - Изменение объекта ".to_string() + &_item.title;
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
    else if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 || _item.user_id == _request_user.id {
            use crate::models::TechCategories;

            let item_cats = _item.get_categories_obj();
            let item_tags = _item.get_tags_obj();
            let _all_tags = Tag::get_all();

            let _cats = Categories::get_with_types(_item.types);

            let mut level: i16 = 0;
            let mut _tech_categories: Vec<TechCategories> = Vec::new();
            let _serve = _item.get_serves();
            if _serve.len() > 0 {
                let tech_id = _serve[0].tech_cat_id;
                let _tech_categories = TechCategories::get(tech_id);
                level = _tech_categories.level;
                let _connection = establish_connection();
                let _tech_categories = schema::tech_categories::table
                    .filter(schema::tech_categories::level.eq(level))
                    .load::<TechCategories>(&_connection)
                    .expect("E");
            }

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/pages/edit_item.stpl")]
                struct Template {
                    request_user:   User,
                    object:         Item,
                    cats:           Vec<Categories>,
                    is_ajax:        i32,
                    all_tags:       Vec<Tag>,
                    item_tags:      Vec<Tag>,
                    item_cats:      Vec<Categories>,
                    tech_cats:      Vec<TechCategories>,
                    level:          i16,
                    template_types: i16,
                    linguage:       i16,
                    title:          String,
                    description:    String,
                    link:           String,
                    image:          String,
                }
                let body = Template {
                    request_user:   _request_user,
                    object:         _item,
                    cats:           _cats,
                    is_ajax:        is_ajax,
                    all_tags:       _all_tags,
                    item_tags:      item_tags,
                    item_cats:      item_cats,
                    tech_cats:      _tech_categories,
                    level:          level,
                    template_types: t,
                    linguage:       l,
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
                #[template(path = "mobile/pages/edit_item.stpl")]
                struct Template {
                    object:         Item,
                    cats:           Vec<Categories>,
                    is_ajax:        i32,
                    all_tags:       Vec<Tag>,
                    item_tags:      Vec<Tag>,
                    item_cats:      Vec<Categories>,
                    tech_cats:      Vec<TechCategories>,
                    level:          i16,
                    template_types: i16,
                    linguage:       i16,
                    title:          String,
                    description:    String,
                    link:           String,
                    image:          String,
                }
                let body = Template {
                    object:         _item,
                    cats:           _cats,
                    is_ajax:        is_ajax,
                    all_tags:       _all_tags,
                    item_tags:      item_tags,
                    item_cats:      item_cats,
                    tech_cats:      _tech_categories,
                    level:          level,
                    template_types: t,
                    linguage:       l,
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
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Permission Denied."))
        }
    }
    else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Permission Denied."))
    }
}

pub async fn edit_content_item_page(conn: ConnectionInfo, session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let (l, t) = crate::utils::get_or_create_c_user_return_lt(conn, &req).await;
    let _item = Item::get_with_id(*_id);

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    let title: String;
    let description: String;
    let link = "/edit_content_item/".to_string() + &_item.id.to_string() + &"/".to_string();
    let image = "/static/images/dark/store.jpg".to_string();
    if l == 2 {
        title = "Update a object content ".to_string() + &_item.title_en;
        description = "Web-services - Update a object content ".to_string() + &_item.title_en;
    }
    else {
        title = "Изменение текста объекта ".to_string() + &_item.title;
        description = "вебсервисы.рф - Изменение текста объекта ".to_string() + &_item.title;
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
    else if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 || _request_user.id == _item.user_id {
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/pages/edit_content_item.stpl")]
                struct Template {
                    request_user:   User,
                    item:           Item,
                    is_ajax:        i32,
                    template_types: i16,
                    linguage:       i16,
                    title:          String,
                    description:    String,
                    link:           String,
                    image:          String,
                }
                let body = Template {
                    request_user:   _request_user,
                    item:           _item,
                    is_ajax:        is_ajax,
                    template_types: t,
                    linguage:       l,
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
                #[template(path = "mobile/pages/edit_content_item.stpl")]
                struct Template {
                    item:           Item,
                    is_ajax:        i32,
                    template_types: i16,
                    linguage:       i16,
                    title:          String,
                    description:    String,
                    link:           String,
                    image:          String,
                }
                let body = Template {
                    item:           _item,
                    is_ajax:        is_ajax,
                    template_types: t,
                    linguage:       l,
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
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Permission Denied."))
        }
    }
    else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Permission Denied."))
    }
}

pub async fn edit_file_page(conn: ConnectionInfo, session: Session, req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::models::File;

    let (l, t) = crate::utils::get_or_create_c_user_return_lt(conn, &req).await;
    let _file = File::get(*_id);
    let id_str = _file.id.to_string();

    let title: String;
    let description: String;
    let link = "/edit_file/".to_string() + &id_str + &"/".to_string();
    let image = "/static/images/dark/store.jpg".to_string();
    if l == 2 {
        title = "Update file #".to_string() + &id_str;
        description = "Web-services: Update file #".to_string() + &id_str;
    }
    else {
        title = "Изменение файла ".to_string() + &id_str;
        description = "вебсервисы.рф: Изменение файла ".to_string() + &id_str;
    }

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    
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
    else if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if _request_user.perm == 60 || _request_user.id == _file.user_id {
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/pages/edit_file.stpl")]
                struct Template {
                    request_user:   User,
                    file:           File,
                    is_ajax:        i32,
                    template_types: i16,
                    linguage:       i16,
                    title:          String,
                    description:    String,
                    link:           String,
                    image:          String,
                }
                let body = Template {
                    request_user:   _request_user,
                    file:           _file,
                    is_ajax:        is_ajax,
                    template_types: t,
                    linguage:       l,
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
                #[template(path = "mobile/pages/edit_file.stpl")]
                struct Template {
                    file:           File,
                    is_ajax:        i32,
                    template_types: i16,
                    linguage:       i16,
                    title:          String,
                    description:    String,
                    link:           String,
                    image:          String,
                }
                let body = Template {
                    file:           _file,
                    is_ajax:        is_ajax,
                    template_types: t,
                    linguage:       l,
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
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Permission Denied."))
        }
    }
    else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Permission Denied."))
    }
}

pub async fn image_page(req: HttpRequest, conn: ConnectionInfo, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    use crate::models::File;

    let _connection = establish_connection();
    let (l, t) = crate::utils::get_or_create_c_user_return_lt(conn, &req).await;
    let _file = File::get(*_id);
    let _item = Item::get_with_id(_file.item_id);
    let _images = _item.get_images_ids();
    let _images_len = _images.len();
    let mut prev: Option<File> = None;
    let mut next: Option<File> = None;

    let title: String;
    let description: String;
    let link = "/image/".to_string() + &_file.id.to_string() + &"/".to_string();
    let image = "/static/images/dark/store.jpg".to_string();
    if l == 2 {
        title = "Image #".to_string() + &_file.id.to_string();
        description = "Изображение #".to_string() + &_file.id.to_string() + &" : Web-services".to_string();
    }
    else {
        title = "Изображение №".to_string() + &_file.id.to_string();
        description = "Изображение №".to_string() + &_file.id.to_string() + &" : вебсервисы.рф".to_string();
    }

    for (i, obj) in _images.iter().enumerate().rev() {
        if obj == &*_id { 
            if (i + 1) != _images_len {
                let _next = Some(&_images[i + 1]);
                next = Some(schema::files::table
                    .filter(schema::files::id.eq(_next.unwrap()))
                    .filter(schema::files::types.eq(_item.types))
                    .first::<File>(&_connection)
                    .expect("E"));
            };
            if i != 0 {
                let _prev = Some(&_images[i - 1]);
                prev = Some(schema::files::table
                    .filter(schema::files::id.eq(_prev.unwrap()))
                    .filter(schema::files::types.eq(_item.types))
                    .first::<File>(&_connection)
                    .expect("E"));
            };
            break;
        }
    };

    #[derive(TemplateOnce)]
    #[template(path = "desctop/load/image.stpl")]
    struct Template {
        object:         File,
        item:           Item,
        prev:           Option<File>,
        next:           Option<File>,
        template_types: i16,
        linguage:       i16,
        title:          String,
        description:    String,
        link:           String,
        image:          String,
    } 
    let body = Template {
        object:         _file,
        item:           _item,
        prev:           prev,
        next:           next,
        template_types: t,
        linguage:       l,
        title:          title,
        description:    description,
        link:           link,
        image:          image,
    }
    .render_once()
    .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
}
