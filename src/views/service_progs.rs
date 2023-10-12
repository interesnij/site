use actix_web::{
    web,
    web::block,
    HttpRequest,
    HttpResponse,
    error::InternalError,
    http::StatusCode,
    dev::ConnectionInfo,
};

use crate::utils::{
    is_signed_in,
    get_request_user_data,
};
use actix_session::Session;
use crate::models::{
    Categories,
    Item, StatPage,
    User,
    Cat,
    SmallTag,
    CatDetail,
};
use sailfish::TemplateOnce;


pub fn service_routes(config: &mut web::ServiceConfig) {
    config.route("/service_categories/", web::get().to(service_categories_page));
    config.service(web::resource("/service/{cat_slug}/{service_slug}/").route(web::get().to(get_service_page)));
    config.service(web::resource("/services/{slug}/").route(web::get().to(service_category_page)));
}


pub async fn get_service_page(conn: ConnectionInfo, session: Session, req: HttpRequest, param: web::Path<(String,String)>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let (l, t) = crate::utils::get_or_create_c_user_return_lt(conn, &req).await;
    let _item_id: String = param.1.clone();
    let _cat_id: String = param.0.clone();

    let _item = Item::get(&_item_id);

    let title: String;
    let description: String;
    let link = "/service/".to_string() + &_cat_id + &"/".to_string() + &_item_id.to_string() + &"/".to_string();
    let image = _item.get_image();
    if l == 2 {
        title = String::new() + &_item.title_en + &" | Service ".to_string();
        description = String::new() + &_item.title_en + &" | Service: Web-services".to_string();
    }
    else {
        title = String::new() + &_item.title + &" | Услуга ".to_string();
        description = String::new() + &_item.title + &" | Услуга: вебсервисы.рф".to_string();
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
        use crate::models::{TechCategories, FeaturedItem};

        let _tech_categories = TechCategories::get_all(); 

        let _category = Categories::get(&_cat_id, _item.types);
        let _cats = block(move || Categories::get_categories_for_types(2, l)).await?;
        let _tags = block(move || Categories::get_tags(2, l)).await?;
        let _help_cats = block(move || Categories::get_categories_for_types(6, l)).await?;
        let (prev, next) = _category.get_featured_items(_item.id, _item.types, l);

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            if !_item.is_active && _request_user.perm < 10 {
                crate::utils::get_private_page (
                    is_ajax,
                    _request_user,
                    is_desctop,
                    &title,
                    &description,
                    &link,
                    &image,
                    t, 
                    l,
                ).await
            }
            else if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/services/service.stpl")]
                struct Template {
                    request_user:   User,
                    object:         Item,
                    category:       Categories,
                    help_cats:      Vec<Cat>,
                    cats:           Vec<Cat>,
                    all_tags:       Vec<SmallTag>,
                    prev:           Option<FeaturedItem>,
                    next:           Option<FeaturedItem>,
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
                    object:         _item,
                    category:       _category,
                    help_cats:      _help_cats,
                    cats:           _cats,
                    all_tags:       _tags,
                    prev:           prev,
                    next:           next,
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
                #[template(path = "mobile/services/service.stpl")]
                struct Template {
                    request_user:   User,
                    object:         Item,
                    category:       Categories,
                    help_cats:      Vec<Cat>,
                    cats:           Vec<Cat>,
                    all_tags:       Vec<SmallTag>,
                    prev:           Option<FeaturedItem>,
                    next:           Option<FeaturedItem>,
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
                    object:         _item,
                    category:       _category,
                    help_cats:      _help_cats,
                    cats:           _cats,
                    all_tags:       _tags,
                    prev:           prev,
                    next:           next,
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
            if !_item.is_active {
                crate::utils::get_anon_private_page (
                    is_ajax, 
                    is_desctop,
                    &title,
                    &description,
                    &link,
                    &image,
                    t,
                    l,
                ).await
            }
            else if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/services/anon_service.stpl")]
                struct Template {
                    object:         Item,
                    category:       Categories,
                    help_cats:      Vec<Cat>,
                    cats:           Vec<Cat>,
                    all_tags:       Vec<SmallTag>,
                    prev:           Option<FeaturedItem>,
                    next:           Option<FeaturedItem>,
                    is_ajax:        i32,
                    template_types: i16,
                    linguage:       i16,
                    title:          String,
                    description:    String,
                    link:           String,
                    image:          String,
                }
                let body = Template {
                    object:         _item,
                    category:       _category,
                    help_cats:      _help_cats,
                    cats:           _cats,
                    all_tags:       _tags,
                    prev:           prev,
                    next:           next,
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
                #[template(path = "mobile/services/anon_service.stpl")]
                struct Template {
                    object:         Item,
                    category:       Categories,
                    help_cats:      Vec<Cat>,
                    cats:           Vec<Cat>,
                    all_tags:       Vec<SmallTag>,
                    prev:           Option<FeaturedItem>,
                    next:           Option<FeaturedItem>,
                    is_ajax:        i32,
                    template_types: i16,
                    linguage:       i16,
                    title:          String,
                    description:    String,
                    link:           String,
                    image:          String,
                }
                let body = Template {
                    object:         _item,
                    category:       _category,
                    help_cats:      _help_cats,
                    cats:           _cats,
                    all_tags:       _tags,
                    prev:           prev,
                    next:           next,
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

pub async fn service_category_page(conn: ConnectionInfo, session: Session, req: HttpRequest, _id: web::Path<String>) -> actix_web::Result<HttpResponse> {
    let (l, t) = crate::utils::get_or_create_c_user_return_lt(conn, &req).await;
    let _category = Categories::get_detail(_id.clone(), 2, l); 

    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);

    let title: String;
    let description: String;
    let link = "/services/".to_string() + &_category.slug + &"/".to_string();
    let image = _category.get_image();
    if l == 2 {
        title = String::new() + &_category.name + &" | Category of the services".to_string();
        description = String::new() + &_category.name + &" | Category of the services: Web-services".to_string();
    }
    else {
        title = String::new() + &_category.name + &" | Категория услуг".to_string();
        description = String::new() + &_category.name + &" | Категория услуг: вебсервисы.рф".to_string();
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
        use crate::models::Service;

        let page = crate::utils::get_page(&req);
        let object_list: Vec<Service>;
        let next_page_number: i32;
        let _cats = block(move || Categories::get_categories_for_types(2, l)).await?;
        let _tags = block(move || Categories::get_tags(2, l)).await?;
        let _help_cats = block(move || Categories::get_categories_for_types(6, l)).await?;

        if is_signed_in(&session) { 
            let _request_user = get_request_user_data(&session);
            let _res = block(move || Categories::get_services_list(_category.id, page, 20, _request_user.perm == 60, l)).await?;
            let _dict = match _res {
                Ok(_ok) => {object_list = _ok.0; next_page_number = _ok.1},
                Err(_error) => {object_list = Vec::new(); next_page_number = 0},
            };
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/services/category.stpl")]
                struct Template {
                    request_user:     User,
                    category:         CatDetail,
                    object_list:      Vec<Service>,
                    help_cats:        Vec<Cat>,
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
                    category:         _category,
                    object_list:      object_list,
                    help_cats:        _help_cats,
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
                #[template(path = "mobile/services/category.stpl")]
                struct Template {
                    all_tags:         Vec<SmallTag>,
                    category:         CatDetail,
                    cats:             Vec<Cat>,
                    help_cats:        Vec<Cat>,
                    object_list:      Vec<Service>,
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
                    all_tags:         _tags,
                    category:         _category,
                    cats:             _cats,
                    help_cats:        _help_cats,
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
            let _res = block(move || Categories::get_services_list(_category.id, page, 20, false, l)).await?;
            let _dict = match _res {
                Ok(_ok) => {object_list = _ok.0; next_page_number = _ok.1},
                Err(_error) => {object_list = Vec::new(); next_page_number = 0},
            };

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/services/anon_category.stpl")]
                struct Template {
                    category:         CatDetail,
                    object_list:      Vec<Service>,
                    help_cats:        Vec<Cat>,
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
                    category:         _category,
                    object_list:      object_list,
                    help_cats:        _help_cats,
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
                #[template(path = "mobile/services/anon_category.stpl")]
                struct Template {
                    all_tags:         Vec<SmallTag>,
                    category:         CatDetail,
                    cats:             Vec<Cat>,
                    help_cats:        Vec<Cat>,
                    object_list:      Vec<Service>,
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
                    all_tags:         _tags,
                    category:         _category,
                    cats:             _cats,
                    help_cats:        _help_cats,
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

pub async fn service_categories_page(conn: ConnectionInfo, session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let (l, t) = crate::utils::get_or_create_c_user_return_lt(conn, &req).await;
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);

    let title: String;
    let description: String;
    let link = "/service_categories/".to_string();
    let image = "/static/images/dark/store.jpg".to_string();
    if l == 2 {
        title = "Categories of services".to_string();
        description = "Web-services - Categories of services".to_string();
    }
    else {
        title = "Категории услуг".to_string();
        description = "вебсервисы.рф - Категории услуг".to_string();
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
        let _stat = crate::models::StatPage::get_or_create(61);
        let _cats = block(move || Categories::get_categories_for_types(2, l)).await?;
        let _tags = block(move || Categories::get_tags(1, l)).await?;
        let _help_cats = block(move || Categories::get_categories_for_types(6, l)).await?;

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/services/categories.stpl")]
                struct Template {
                    request_user:   User,
                    is_ajax:        i32,
                    cats:           Vec<Cat>,
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
                    cats:           _cats,
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
                #[template(path = "mobile/services/categories.stpl")]
                struct Template {
                    is_ajax:        i32,
                    cats:           Vec<Cat>,
                    help_cats:      Vec<Cat>,
                    all_tags:       Vec<SmallTag>,
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
                    cats:           _cats,
                    help_cats:      _help_cats,
                    all_tags:       _tags,
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
                #[template(path = "desctop/services/anon_categories.stpl")]
                struct Template {
                    is_ajax:        i32,
                    cats:           Vec<Cat>,
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
                    cats:           _cats,
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
                #[template(path = "mobile/services/anon_categories.stpl")]
                struct Template {
                    is_ajax:        i32,
                    cats:           Vec<Cat>,
                    help_cats:      Vec<Cat>,
                    all_tags:       Vec<SmallTag>,
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
                    cats:           _cats,
                    help_cats:      _help_cats,
                    all_tags:       _tags,
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
}
