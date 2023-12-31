use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    error::InternalError,
    http::StatusCode,
    dev::ConnectionInfo,
};

use actix_session::Session;
use crate::utils::{
    is_signed_in,
    get_request_user_data,
};
use sailfish::TemplateOnce;
use crate::models::User;


pub fn search_routes(config: &mut web::ServiceConfig) {
    config.route("/search/", web::get().to(empty_search_page));
    config.route("/search/{q}/", web::get().to(search_page));
    config.route("/search_blogs/{q}/", web::get().to(search_blogs_page));
    config.route("/search_services/{q}/", web::get().to(search_services_page));
    config.route("/search_stores/{q}/", web::get().to(search_stores_page));
    config.route("/search_wikis/{q}/", web::get().to(search_wikis_page));
    config.route("/search_works/{q}/", web::get().to(search_works_page));
    config.route("/search_help/{q}/", web::get().to(search_help_page));
}


pub async fn empty_search_page(conn: ConnectionInfo, req: HttpRequest, session: Session) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let (l, t, c) = crate::utils::get_or_create_c_user_return_ltc(conn, &req).await;

    let title: String;
    let description: String;
    let link = "/search/".to_string();
    let image = "/static/images/dark/store.jpg".to_string();
    if l == 2 {
        title = "General search".to_string();
        description = "Web-services - General search".to_string();
    }
    else {
        title = "Общий поиск".to_string();
        description = "вебсервисы.рф - Общий поиск".to_string();
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
            #[template(path = "desctop/search/empty_search.stpl")]
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
            #[template(path = "mobile/search/empty_search.stpl")]
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
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/search/anon_empty_search.stpl")]
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
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/search/anon_empty_search.stpl")]
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

pub async fn search_page(conn: ConnectionInfo, session: Session, req: HttpRequest, q: web::Path<String>) -> actix_web::Result<HttpResponse> {
    use crate::utils::get_device_and_ajax;

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let _q = q.clone();
    let _q_standalone = "%".to_owned() + &_q + "%";
    let (l, t, c) = crate::utils::get_or_create_c_user_return_ltc(conn, &req).await;

    let title: String;
    let description: String;
    let link = "/search/".to_string() + &q + &"/".to_string();
    let image = "/static/images/dark/store.jpg".to_string();
    if l == 2 {
        title = "Search for all by fragment ".to_string() + &q;
        description = "Web-services: Search for all by fragment ".to_string() + &q;
    }
    else {
        title = "Общий поиск по фрагменту ".to_string() + &q;
        description = "вебсервисы.рф: Общий поиск по фрагменту ".to_string() + &q;
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
        use crate::models::{Item, Blog, Service, Store, Wiki, Work};
        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            let is_admin = _request_user.is_superuser();

            let (work_list, work_count) = Item::search_works(&_q_standalone, 3, 0, is_admin, l);
            let (service_list, service_count) = Item::search_services(&_q_standalone, 3, 0, is_admin, l);
            let (wiki_list, wiki_count) = Item::search_wikis(&_q_standalone, 3, 0, is_admin, l);
            let (blog_list, blog_count) = Item::search_blogs(&_q_standalone, 3, 0, is_admin, l);
            let (store_list, store_count) = Item::search_stores(&_q_standalone, 3, 0, is_admin, l);

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/search/all.stpl")]
                struct Template {
                    request_user:   User,
                    works_list:     Vec<Work>,
                    services_list:  Vec<Service>,
                    wikis_list:     Vec<Wiki>,
                    blogs_list:     Vec<Blog>,
                    stores_list:    Vec<Store>,

                    works_count:    usize,
                    services_count: usize,
                    wikis_count:    usize,
                    blogs_count:    usize,
                    stores_count:   usize,
                    is_ajax:        i32,
                    q:              String,
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
                    works_list:     work_list,
                    services_list:  service_list,
                    wikis_list:     wiki_list,
                    blogs_list:     blog_list,
                    stores_list:    store_list,

                    works_count:    work_count,
                    services_count: service_count,
                    wikis_count:    wiki_count,
                    blogs_count:    blog_count,
                    stores_count:   store_count,
                    is_ajax:        is_ajax,
                    q:              _q,
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
                #[template(path = "mobile/search/all.stpl")]
                struct Template {
                    works_list:     Vec<Work>,
                    services_list:  Vec<Service>,
                    wikis_list:     Vec<Wiki>,
                    blogs_list:     Vec<Blog>,
                    stores_list:    Vec<Store>,

                    works_count:    usize,
                    services_count: usize,
                    wikis_count:    usize,
                    blogs_count:    usize,
                    stores_count:   usize,
                    is_ajax:        i32,
                    q:              String,
                    template_types: i16,
                    linguage:       i16,
                    currency:       String,
                    title:          String,
                    description:    String,
                    link:           String,
                    image:          String,
                }
                let body = Template {
                    works_list:     work_list,
                    services_list:  service_list,
                    wikis_list:     wiki_list,
                    blogs_list:     blog_list,
                    stores_list:    store_list,

                    works_count:    work_count,
                    services_count: service_count,
                    wikis_count:    wiki_count,
                    blogs_count:    blog_count,
                    stores_count:   store_count,
                    is_ajax:        is_ajax,
                    q:              _q,
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
        else {
            let (work_list, work_count) = Item::search_works(&_q_standalone, 3, 0, false, l);
            let (service_list, service_count) = Item::search_services(&_q_standalone, 3, 0, false, l);
            let (wiki_list, wiki_count) = Item::search_wikis(&_q_standalone, 3, 0, false, l);
            let (blog_list, blog_count) = Item::search_blogs(&_q_standalone, 3, 0, false, l);
            let (store_list, store_count) = Item::search_stores(&_q_standalone, 3, 0, false, l);

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/search/anon_all.stpl")]
                struct Template {
                    works_list:     Vec<Work>,
                    services_list:  Vec<Service>,
                    wikis_list:     Vec<Wiki>,
                    blogs_list:     Vec<Blog>,
                    stores_list:    Vec<Store>,

                    works_count:    usize,
                    services_count: usize,
                    wikis_count:    usize,
                    blogs_count:    usize,
                    stores_count:   usize,
                    is_ajax:        i32,
                    q:              String,
                    template_types: i16,
                    linguage:       i16,
                    currency:       String,
                    title:          String,
                    description:    String,
                    link:           String,
                    image:          String,
                }
                let body = Template {
                    works_list:     work_list,
                    services_list:  service_list,
                    wikis_list:     wiki_list,
                    blogs_list:     blog_list,
                    stores_list:    store_list,

                    works_count:    work_count,
                    services_count: service_count,
                    wikis_count:    wiki_count,
                    blogs_count:    blog_count,
                    stores_count:   store_count,
                    is_ajax:        is_ajax,
                    q:              _q,
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
                #[template(path = "mobile/search/anon_all.stpl")]
                struct Template {
                    works_list:     Vec<Work>,
                    services_list:  Vec<Service>,
                    wikis_list:     Vec<Wiki>,
                    blogs_list:     Vec<Blog>,
                    stores_list:    Vec<Store>,

                    works_count:    usize,
                    services_count: usize,
                    wikis_count:    usize,
                    blogs_count:    usize,
                    stores_count:   usize,
                    is_ajax:        i32,
                    q:              String,
                    template_types: i16,
                    linguage:       i16,
                    currency:       String,
                    title:          String,
                    description:    String,
                    link:           String,
                    image:          String,
                }
                let body = Template {
                    works_list:     work_list,
                    services_list:  service_list,
                    wikis_list:     wiki_list,
                    blogs_list:     blog_list,
                    stores_list:    store_list,

                    works_count:    work_count,
                    services_count: service_count,
                    wikis_count:    wiki_count,
                    blogs_count:    blog_count,
                    stores_count:   store_count,
                    is_ajax:        is_ajax,
                    q:              _q,
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

pub async fn search_blogs_page(conn: ConnectionInfo, session: Session, req: HttpRequest, q: web::Path<String>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let _q = q.clone();
    let (l, t, c) = crate::utils::get_or_create_c_user_return_ltc(conn, &req).await;

    let title: String;
    let description: String;
    let link = "/search_blogs/".to_string() + &q + &"/".to_string();
    let image = "/static/images/dark/store.jpg".to_string();
    if l == 2 {
        title = "Search for articles by fragment ".to_string() + &q;
        description = "Web-services: Search for articles by fragment ".to_string() + &q;
    }
    else {
        title = "Поиск статей по фрагменту ".to_string() + &q;
        description = "вебсервисы.рф: Поиск статей по фрагменту ".to_string() + &q;
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
        use crate::models::{Item, Blog};

        let page = crate::utils::get_page(&req);

        let _q_standalone = "%".to_owned() + &_q + "%";

        let mut next_page_number = 0;
        let offset: i32;
        let next_item: i32;
        if page > 1 {
            offset = (page - 1) * 20;
            next_item = page * 20 + 1;
        }
        else {
            offset = 0;
            next_item = 21;
        }

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            let is_admin = _request_user.is_superuser();
            let (blog_list, blogs_count) = Item::search_blogs(&_q_standalone, 20, offset.into(), is_admin, l);

            if Item::search_blogs(&_q_standalone, 1, next_item.into(), is_admin, l).0.len() > 0 {
                next_page_number = page + 1;
            }
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/search/blogs.stpl")]
                struct Template {
                    request_user:     User,
                    blogs_list:       Vec<Blog>,
                    blogs_count:      usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                    template_types:   i16,
                    linguage:         i16,
                    currency:         String,
                    title:            String,
                    description:      String,
                    link:             String,
                    image:            String,
                }
                let body = Template {
                    request_user:     _request_user,
                    blogs_list:       blog_list,
                    blogs_count:      blogs_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                    template_types:   t,
                    linguage:         l,
                    currency:         c,
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
                #[template(path = "mobile/search/blogs.stpl")]
                struct Template {
                    blogs_list:       Vec<Blog>,
                    blogs_count:      usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                    template_types:   i16,
                    linguage:         i16,
                    currency:         String,
                    title:            String,
                    description:      String,
                    link:             String,
                    image:            String,
                }
                let body = Template {
                    blogs_list:       blog_list,
                    blogs_count:      blogs_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                    template_types:   t,
                    linguage:         l,
                    currency:         c,
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
            let (blog_list, blogs_count) = Item::search_blogs(&_q_standalone, 20, offset.into(), false, l);

            if Item::search_blogs(&_q_standalone, 1, next_item.into(), false, l).0.len() > 0 {
                next_page_number = page + 1;
            }

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/search/anon_blogs.stpl")]
                struct Template {
                    blogs_list:       Vec<Blog>,
                    blogs_count:      usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                    template_types:   i16,
                    linguage:         i16,
                    currency:         String,
                    title:            String,
                    description:      String,
                    link:             String,
                    image:            String,
                }
                let body = Template {
                    blogs_list:       blog_list,
                    blogs_count:      blogs_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                    template_types:   t,
                    linguage:         l,
                    currency:         c,
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
                #[template(path = "mobile/search/anon_blogs.stpl")]
                struct Template {
                    blogs_list:       Vec<Blog>,
                    blogs_count:      usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                    template_types:   i16,
                    linguage:         i16,
                    currency:         String,
                    title:            String,
                    description:      String,
                    link:             String,
                    image:            String,
                }
                let body = Template {
                    blogs_list:       blog_list,
                    blogs_count:      blogs_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                    template_types:   t,
                    linguage:         l,
                    currency:         c,
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

pub async fn search_services_page(conn: ConnectionInfo, session: Session, req: HttpRequest, q: web::Path<String>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let _q = q.clone();
    let (l, t, c) = crate::utils::get_or_create_c_user_return_ltc(conn, &req).await;

    let title: String;
    let description: String;
    let link = "/search_services/".to_string() + &q + &"/".to_string();
    let image = "/static/images/dark/store.jpg".to_string();
    if l == 2 {
        title = "Search for services by fragment ".to_string() + &q;
        description = "Web-services: Search for services by fragment ".to_string() + &q;
    }
    else {
        title = "Поиск услуг по фрагменту ".to_string() + &q;
        description = "вебсервисы.рф: Поиск услуг по фрагменту ".to_string() + &q;
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
        use crate::models::{Item, Service};

        let page = crate::utils::get_page(&req);
        let _q_standalone = "%".to_owned() + &_q + "%";

        let mut next_page_number = 0;
        let offset: i32;
        let next_item: i32;
        if page > 1 {
            offset = (page - 1) * 20;
            next_item = page * 20 + 1;
        }
        else {
            offset = 0;
            next_item = 21;
        }

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            let is_admin = _request_user.is_superuser();
            let (services_list, services_count) = Item::search_services(&_q_standalone, 20, offset.into(), is_admin, l);

            if Item::search_services(&_q_standalone, 1, next_item.into(), is_admin, l).0.len() > 0 {
                next_page_number = page + 1;
            }
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/search/services.stpl")]
                struct Template {
                    request_user:     User,
                    services_list:    Vec<Service>,
                    services_count:   usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                    template_types:   i16,
                    linguage:         i16,
                    currency:         String,
                    title:            String,
                    description:      String,
                    link:             String,
                    image:            String,
                }
                let body = Template {
                    request_user:     _request_user,
                    services_list:    services_list,
                    services_count:   services_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                    template_types:   t,
                    linguage:         l,
                    currency:         c,
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
                #[template(path = "mobile/search/services.stpl")]
                struct Template {
                    services_list:    Vec<Service>,
                    services_count:   usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                    template_types:   i16,
                    linguage:         i16,
                    currency:         String,
                    title:            String,
                    description:      String,
                    link:             String,
                    image:            String,
                }

                let body = Template {
                    services_list:    services_list,
                    services_count:   services_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                    template_types:   t,
                    linguage:         l,
                    currency:         c,
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
            let (services_list, services_count) = Item::search_services(&_q_standalone, 20, offset.into(), false, l);

            if Item::search_services(&_q_standalone, 1, next_item.into(), false, l).0.len() > 0 {
                next_page_number = page + 1;
            }

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/search/anon_services.stpl")]
                struct Template {
                    services_list:    Vec<Service>,
                    services_count:   usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                    template_types:   i16,
                    linguage:         i16,
                    currency:         String,
                    title:            String,
                    description:      String,
                    link:             String,
                    image:            String,
                }
                let body = Template {
                    services_list:    services_list,
                    services_count:   services_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                    template_types:   t,
                    linguage:         l,
                    currency:         c,
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
                #[template(path = "mobile/search/anon_services.stpl")]
                struct Template {
                    services_list:    Vec<Service>,
                    services_count:   usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                    template_types:   i16,
                    linguage:         i16,
                    currency:         String,
                    title:            String,
                    description:      String,
                    link:             String,
                    image:            String,
                }
                let body = Template {
                    services_list:    services_list,
                    services_count:   services_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                    template_types:   t,
                    linguage:         l,
                    currency:         c,
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

pub async fn search_stores_page(conn: ConnectionInfo, session: Session, req: HttpRequest, q: web::Path<String>) -> actix_web::Result<HttpResponse> {
    use crate::utils::{get_device_and_ajax, get_page};

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let _q = q.clone();
    let (l, t, c) = crate::utils::get_or_create_c_user_return_ltc(conn, &req).await;

    let title: String;
    let description: String;
    let link = "/search_services/".to_string() + &q + &"/".to_string();
    let image = "/static/images/dark/store.jpg".to_string();
    if l == 2 {
        title = "Search for market by fragment ".to_string() + &q;
        description = "Web-services: Search for market by fragment ".to_string() + &q;
    }
    else {
        title = "Поиск товаров по фрагменту ".to_string() + &q;
        description = "вебсервисы.рф: Поиск товаров по фрагменту ".to_string() + &q;
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
        use crate::models::{Item, Store};

        let page = get_page(&req);

        let _q_standalone = "%".to_owned() + &_q + "%";

        let mut next_page_number = 0;
        let offset: i32;
        let next_item: i32;
        if page > 1 {
            offset = (page - 1) * 20;
            next_item = page * 20 + 1;
        }
        else {
            offset = 0;
            next_item = 21;
        }

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            let is_admin = _request_user.is_superuser();
            let (stores_list, stores_count) = Item::search_stores(&_q_standalone, 20, offset.into(), is_admin, l);

            if Item::search_stores(&_q_standalone, 1, next_item.into(), is_admin, l).0.len() > 0 {
                next_page_number = page + 1;
            }

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/search/stores.stpl")]
                struct Template {
                    request_user:     User,
                    stores_list:      Vec<Store>,
                    stores_count:     usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                    template_types:   i16,
                    linguage:         i16,
                    currency:         String,
                    title:            String,
                    description:      String,
                    link:             String,
                    image:            String,
                }
                let body = Template {
                    request_user:     _request_user,
                    stores_list:       stores_list,
                    stores_count:      stores_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                    template_types:   t,
                    linguage:         l,
                    currency:         c,
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
                #[template(path = "mobile/search/stores.stpl")]
                struct Template {
                    stores_list:      Vec<Store>,
                    stores_count:     usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                    template_types:   i16,
                    linguage:         i16,
                    currency:         String,
                    title:            String,
                    description:      String,
                    link:             String,
                    image:            String,
                }
                let body = Template {
                    stores_list:      stores_list,
                    stores_count:     stores_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                    template_types:   t,
                    linguage:         l,
                    currency:         c,
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
            let (stores_list, stores_count) = Item::search_stores(&_q_standalone, 20, offset.into(), false, l);

            if Item::search_stores(&_q_standalone, 1, next_item.into(), false, l).0.len() > 0 {
                next_page_number = page + 1;
            }

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/search/anon_stores.stpl")]
                struct Template {
                    stores_list:      Vec<Store>,
                    stores_count:     usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                    template_types:   i16,
                    linguage:         i16,
                    currency:         String,
                    title:            String,
                    description:      String,
                    link:             String,
                    image:            String,
                }

                let body = Template {
                    stores_list:      stores_list,
                    stores_count:     stores_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                    template_types:   t,
                    linguage:         l,
                    currency:         c,
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
                #[template(path = "mobile/search/anon_stores.stpl")]
                struct Template {
                    stores_list:      Vec<Store>,
                    stores_count:     usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                    template_types:   i16,
                    linguage:         i16,
                    currency:         String,
                    title:            String,
                    description:      String,
                    link:             String,
                    image:            String,
                }
                let body = Template {
                    stores_list:      stores_list,
                    stores_count:     stores_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                    template_types:   t,
                    linguage:         l,
                    currency:         c,
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

pub async fn search_wikis_page(conn: ConnectionInfo, session: Session, req: HttpRequest, q: web::Path<String>) -> actix_web::Result<HttpResponse> {
    use crate::utils::{get_device_and_ajax, get_page};

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let _q = q.clone();
    let (l, t, c) = crate::utils::get_or_create_c_user_return_ltc(conn, &req).await;

    let title: String;
    let description: String;
    let link = "/search_wikis/".to_string() + &q + &"/".to_string();
    let image = "/static/images/dark/store.jpg".to_string();
    if l == 2 {
        title = "Search for wiki by fragment ".to_string() + &q;
        description = "Web-services: Search for wiki by fragment ".to_string() + &q;
    }
    else {
        title = "Поиск статей по фрагменту ".to_string() + &q;
        description = "вебсервисы.рф: Поиск статей по фрагменту ".to_string() + &q;
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
        use crate::models::{Item, Wiki};

        let page = get_page(&req);
        let _q_standalone = "%".to_owned() + &_q + "%";

        let mut next_page_number = 0;
        let offset: i32;
        let next_item: i32;
        if page > 1 {
            offset = (page - 1) * 20;
            next_item = page * 20 + 1;
        }
        else {
            offset = 0;
            next_item = 21;
        }

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            let is_admin = _request_user.is_superuser();
            let (wiki_list, wikis_count) = Item::search_wikis(&_q_standalone, 20, offset.into(), is_admin, l);

            if Item::search_wikis(&_q_standalone, 1, next_item.into(), is_admin, l).0.len() > 0 {
                next_page_number = page + 1;
            }

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/search/wikis.stpl")]
                struct Template {
                    request_user:     User,
                    wikis_list:       Vec<Wiki>,
                    wikis_count:      usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                    template_types:   i16,
                    linguage:         i16,
                    currency:         String,
                    title:            String,
                    description:      String,
                    link:             String,
                    image:            String,
                }
                let body = Template {
                    request_user:     _request_user,
                    wikis_list:       wiki_list,
                    wikis_count:      wikis_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                    template_types:   t,
                    linguage:         l,
                    currency:         c,
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
                #[template(path = "mobile/search/wikis.stpl")]
                struct Template {
                    wikis_list:       Vec<Wiki>,
                    wikis_count:      usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                    template_types:   i16,
                    linguage:         i16,
                    currency:         String,
                    title:            String,
                    description:      String,
                    link:             String,
                    image:            String,
                }
                let body = Template {
                    wikis_list:       wiki_list,
                    wikis_count:      wikis_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                    template_types:   t,
                    linguage:         l,
                    currency:         c,
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
            let (wiki_list, wikis_count) = Item::search_wikis(&_q_standalone, 20, offset.into(), false, l);

            if Item::search_wikis(&_q_standalone, 1, next_item.into(), false, l).0.len() > 0 {
                next_page_number = page + 1;
            }

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/search/anon_wikis.stpl")]
                struct Template {
                    wikis_list:       Vec<Wiki>,
                    wikis_count:      usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                    template_types:   i16,
                    linguage:         i16,
                    currency:         String,
                    title:            String,
                    description:      String,
                    link:             String,
                    image:            String,
                }
                let body = Template {
                    wikis_list:       wiki_list,
                    wikis_count:      wikis_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                    template_types:   t,
                    linguage:         l,
                    currency:         c,
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
                #[template(path = "mobile/search/anon_wikis.stpl")]
                struct Template {
                    wikis_list:       Vec<Wiki>,
                    wikis_count:      usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                    template_types:   i16,
                    linguage:         i16,
                    currency:         String,
                    title:            String,
                    description:      String,
                    link:             String,
                    image:            String,
                }
                let body = Template {
                    wikis_list:       wiki_list,
                    wikis_count:      wikis_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                    template_types:   t,
                    linguage:         l,
                    currency:         c,
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

pub async fn search_works_page(conn: ConnectionInfo, session: Session, req: HttpRequest, q: web::Path<String>) -> actix_web::Result<HttpResponse> {
    use crate::utils::{get_device_and_ajax, get_page};

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let _q = q.clone();
    let (l, t, c) = crate::utils::get_or_create_c_user_return_ltc(conn, &req).await;

    let title: String;
    let description: String;
    let link = "/search_works/".to_string() + &q + &"/".to_string();
    let image = "/static/images/dark/store.jpg".to_string();
    if l == 2 {
        title = "Search for works by fragment ".to_string() + &q;
        description = "Web-services: Search for works by fragment ".to_string() + &q;
    }
    else {
        title = "Поиск работ по фрагменту ".to_string() + &q;
        description = "вебсервисы.рф: Поиск работ по фрагменту ".to_string() + &q;
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
        use crate::models::{Item, Work};

        let page = get_page(&req);
        let _q_standalone = "%".to_owned() + &_q + "%";

        let mut next_page_number = 0;
        let offset: i32;
        let next_item: i32;
        if page > 1 {
            offset = (page - 1) * 20;
            next_item = page * 20 + 1;
        }
        else {
            offset = 0;
            next_item = 21;
        }

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);

            let is_admin = _request_user.is_superuser();
            let (work_list, works_count) = Item::search_works(&_q_standalone, 20, offset.into(), is_admin, l);

            if Item::search_works(&_q_standalone, 1, next_item.into(), is_admin, l).0.len() > 0 {
                next_page_number = page + 1;
            }

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/search/works.stpl")]
                struct Template {
                    request_user:     User,
                    works_list:       Vec<Work>,
                    works_count:      usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                    template_types:   i16,
                    linguage:         i16,
                    currency:         String,
                    title:            String,
                    description:      String,
                    link:             String,
                    image:            String,
                }
                let body = Template {
                    request_user:     _request_user,
                    works_list:       work_list,
                    works_count:      works_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                    template_types:   t,
                    linguage:         l,
                    currency:         c,
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
                #[template(path = "mobile/search/works.stpl")]
                struct Template {
                    works_list:       Vec<Work>,
                    works_count:      usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                    template_types:   i16,
                    linguage:         i16,
                    currency:         String,
                    title:            String,
                    description:      String,
                    link:             String,
                    image:            String,
                }
                let body = Template {
                    works_list:       work_list,
                    works_count:      works_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                    template_types:   t,
                    linguage:         l,
                    currency:         c,
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
            let (work_list, works_count) = Item::search_works(&_q_standalone, 20, offset.into(), false, l);

            if Item::search_works(&_q_standalone, 1, next_item.into(), false, l).0.len() > 0 {
                next_page_number = page + 1;
            }

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/search/anon_works.stpl")]
                struct Template {
                    works_list:       Vec<Work>,
                    works_count:      usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                    template_types:   i16,
                    linguage:         i16,
                    currency:         String,
                    title:            String,
                    description:      String,
                    link:             String,
                    image:            String,
                }
                let body = Template {
                    works_list:       work_list,
                    works_count:      works_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                    template_types:   t,
                    linguage:         l,
                    currency:         c,
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
                #[template(path = "mobile/search/anon_works.stpl")]
                struct Template {
                    works_list:       Vec<Work>,
                    works_count:      usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                    template_types:   i16,
                    linguage:         i16,
                    currency:         String,
                    title:            String,
                    description:      String,
                    link:             String,
                    image:            String,
                }
                let body = Template {
                    works_list:       work_list,
                    works_count:      works_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                    template_types:   t,
                    linguage:         l,
                    currency:         c,
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

pub async fn search_help_page(conn: ConnectionInfo, session: Session, req: HttpRequest, q: web::Path<String>) -> actix_web::Result<HttpResponse> {
    use crate::utils::{get_device_and_ajax, get_page};

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    let (l, t, c) = crate::utils::get_or_create_c_user_return_ltc(conn, &req).await;
    let _q = q.clone();

    let title: String;
    let description: String;
    let link = "/search_help/".to_string() + &q + &"/".to_string();
    let image = "/static/images/dark/store.jpg".to_string();
    if l == 2 {
        title = "Search for help by fragment ".to_string() + &q;
        description = "Web-services: Search for help by fragment ".to_string() + &q;
    }
    else {
        title = "Поиск помощи по фрагменту ".to_string() + &q;
        description = "вебсервисы.рф: Поиск помощи по фрагменту ".to_string() + &q;
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
        use crate::models::{Item, Help};

        let page = get_page(&req);
        let _q_standalone = "%".to_owned() + &_q + "%";

        let mut next_page_number = 0;
        let offset: i32;
        let next_item: i32;
        if page > 1 {
            offset = (page - 1) * 20;
            next_item = page * 20 + 1;
        }
        else {
            offset = 0;
            next_item = 21;
        }

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            let is_admin = _request_user.is_superuser();
            let (_items, items_count) = Item::search_helps(&_q_standalone, 20, offset.into(), is_admin, l);

            if Item::search_helps(&_q_standalone, 1, next_item.into(), is_admin, l).0.len() > 0 {
                next_page_number = page + 1;
            }
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/search/help.stpl")]
                struct Template {
                    request_user:     User,
                    items_list:       Vec<Help>,
                    items_count:      usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                    template_types:   i16,
                    linguage:         i16,
                    currency:         String,
                    title:            String,
                    description:      String,
                    link:             String,
                    image:            String,
                }
                let body = Template {
                    request_user:     _request_user,
                    items_list:       _items,
                    items_count:      items_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                    template_types:   t,
                    linguage:         l,
                    currency:         c,
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
                #[template(path = "mobile/search/help.stpl")]
                struct Template {
                    items_list:       Vec<Help>,
                    items_count:      usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                    template_types:   i16,
                    linguage:         i16,
                    currency:         String,
                    title:            String,
                    description:      String,
                    link:             String,
                    image:            String,
                }
                let body = Template {
                    items_list:       _items,
                    items_count:      items_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                    template_types:   t,
                    linguage:         l,
                    currency:         c,
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
            let (_items, items_count) = Item::search_helps(&_q_standalone, 20, offset.into(), false, l);
            if Item::search_helps(&_q_standalone, 1, next_item.into(), false, l).0.len() > 0 {
                next_page_number = page + 1;
            }
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/search/anon_help.stpl")]
                struct Template {
                    items_list:       Vec<Help>,
                    items_count:      usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                    template_types:   i16,
                    linguage:         i16,
                    currency:         String,
                    title:            String,
                    description:      String,
                    link:             String,
                    image:            String,
                }
                let body = Template {
                    items_list:       _items,
                    items_count:      items_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                    template_types:   t,
                    linguage:         l,
                    currency:         c,
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
                #[template(path = "mobile/search/anon_help.stpl")]
                struct Template {
                    items_list:       Vec<Help>,
                    items_count:      usize,
                    is_ajax:          i32,
                    q:                String,
                    next_page_number: i32,
                    template_types:   i16,
                    linguage:         i16,
                    currency:         String,
                    title:            String,
                    description:      String,
                    link:             String,
                    image:            String,
                }
                let body = Template {
                    items_list:       _items,
                    items_count:      items_count,
                    is_ajax:          is_ajax,
                    q:                _q,
                    next_page_number: next_page_number,
                    template_types:   t,
                    linguage:         l,
                    currency:         c,
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
