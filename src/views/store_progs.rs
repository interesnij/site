use actix_web::{
    web,
    web::block,
    HttpRequest,
    HttpResponse,
    error::InternalError,
    http::StatusCode,
};

use crate::utils::{
    is_signed_in,
    get_request_user_data,
    get_first_load_page,
    get_all_storage,
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


pub fn store_routes(config: &mut web::ServiceConfig) {
    config.route("/store_categories/", web::get().to(store_categories_page));
    config.service(web::resource("/store/{cat_slug}/{store_slug}/").route(web::get().to(get_store_page)));
    config.service(web::resource("/stores/{slug}/").route(web::get().to(store_category_page)));
}


pub async fn get_store_page(session: Session, req: HttpRequest, param: web::Path<(String,String)>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let _item_id: String = param.1.clone();
    let _cat_id: String = param.0.clone();
    let (t, l) = get_all_storage();

    let _item = Item::get(&_item_id);

    let title: String;
    let description: String;
    let link = "/store/".to_string() + &_cat_id + &"/".to_string() + &_item_id.to_string() + &"/".to_string();
    let image = _item.get_image();
    if l == 2 {
        title = String::new() + &_item.title_en + &" | Product ".to_string();
        description = String::new() + &_item.title_en + &" | Product: Web-services".to_string();
    }
    else {
        title = String::new() + &_item.title + &" | Товар ".to_string();
        description = String::new() + &_item.title + &" | Товар: вебсервисы.рф".to_string();
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
        use crate::models::FeaturedItem;

        let _category = Categories::get(&_cat_id, _item.types);
        let _cats = block(move || Categories::get_categories_for_types(3, l)).await?;
        let _tags = block(move || Categories::get_tags(3, l)).await?;
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
                #[template(path = "desctop/stores/store.stpl")]
                struct Template {
                    request_user:   User,
                    object:         Item,
                    category:       Categories,
                    help_cats:      Vec<Cat>,
                    prev:           Option<FeaturedItem>,
                    next:           Option<FeaturedItem>,
                    is_ajax:        i32,
                    template_types: u8,
                    linguage:       u8,
                }
                let body = Template {
                    request_user:   _request_user,
                    object:         _item,
                    category:       _category,
                    help_cats:      _help_cats,
                    prev:           prev,
                    next:           next,
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
                #[template(path = "mobile/stores/store.stpl")]
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
                    template_types: u8,
                    linguage:       u8,
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
                #[template(path = "desctop/stores/anon_store.stpl")]
                struct Template {
                    object:         Item,
                    category:       Categories,
                    help_cats:      Vec<Cat>,
                    prev:           Option<FeaturedItem>,
                    next:           Option<FeaturedItem>,
                    is_ajax:        i32,
                    template_types: u8,
                    linguage:       u8,
                }
                let body = Template {
                    object:         _item,
                    category:       _category,
                    help_cats:      _help_cats,
                    prev:           prev,
                    next:           next,
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
                #[template(path = "mobile/stores/anon_store.stpl")]
                struct Template {
                    object:         Item,
                    category:       Categories,
                    help_cats:      Vec<Cat>,
                    cats:           Vec<Cat>,
                    all_tags:       Vec<SmallTag>,
                    prev:           Option<FeaturedItem>,
                    next:           Option<FeaturedItem>,
                    is_ajax:        i32,
                    template_types: u8,
                    linguage:       u8,
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
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}

pub async fn store_category_page(session: Session, req: HttpRequest, _id: web::Path<String>) -> actix_web::Result<HttpResponse> {
    let (t, l) = get_all_storage();
    let _category = Categories::get_detail(_id.clone(), 3, l);

    let cat_image: String;
    if _category.image.is_some() {
        cat_image = _category.image.as_deref().unwrap().to_string();
    }
    else {
        cat_image = "/static/images/dark/store.jpg".to_string();
    }

    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);

    let title: String;
    let description: String;
    let link = "/stores/".to_string() + &_category.slug + &"/".to_string();
    let image = _category.get_image();
    if l == 2 {
        title = String::new() + &_category.name + &" | Category of the market".to_string();
        description = String::new() + &_category.name + &" | Category of the market: Web-services".to_string();
    }
    else {
        title = String::new() + &_category.name + &" | Категория товаров".to_string();
        description = String::new() + &_category.name + &" | Категория товаров: вебсервисы.рф".to_string();
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
        use crate::models::Store;

        let page = crate::utils::get_page(&req);
        let object_list: Vec<Store>;
        let next_page_number: i32;
        let _cats = block(move || Categories::get_categories_for_types(3, l)).await?;
        let _tags = block(move || Categories::get_tags(3, l)).await?;
        let _help_cats = block(move || Categories::get_categories_for_types(6, l)).await?;

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            let _res = block(move || Categories::get_stores_list(_category.id, page, 20, _request_user.perm == 60, l)).await?;
            let _dict = match _res {
                Ok(_ok) => {object_list = _ok.0; next_page_number = _ok.1},
                Err(_error) => {object_list = Vec::new(); next_page_number = 0},
            };
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/stores/category.stpl")]
                struct Template {
                    request_user:     User,
                    all_tags:         Vec<SmallTag>,
                    category:         CatDetail,
                    cats:             Vec<Cat>,
                    help_cats:        Vec<Cat>,
                    object_list:      Vec<Store>,
                    next_page_number: i32,
                    is_ajax:          i32,
                    template_types:   u8,
                    linguage:         u8,
                }
                let body = Template {
                    request_user:     _request_user,
                    all_tags:         _tags,
                    category:         _category,
                    cats:             _cats,
                    help_cats:        _help_cats,
                    object_list:      object_list,
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
                #[template(path = "mobile/stores/category.stpl")]
                struct Template {
                    all_tags:         Vec<SmallTag>,
                    category:         CatDetail,
                    cats:             Vec<Cat>,
                    help_cats:        Vec<Cat>,
                    object_list:      Vec<Store>,
                    next_page_number: i32,
                    is_ajax:          i32,
                    template_types:   u8,
                    linguage:         u8,
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
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
        else {
            let _res = block(move || Categories::get_stores_list(_category.id, page, 20, false, l)).await?;
            let _dict = match _res {
                Ok(_ok) => {object_list = _ok.0; next_page_number = _ok.1},
                Err(_error) => {object_list = Vec::new(); next_page_number = 0},
            };

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/stores/anon_category.stpl")]
                struct Template {
                    all_tags:         Vec<SmallTag>,
                    category:         CatDetail,
                    cats:             Vec<Cat>,
                    help_cats:        Vec<Cat>,
                    object_list:      Vec<Store>,
                    next_page_number: i32,
                    is_ajax:          i32,
                    template_types:   u8,
                    linguage:         u8,
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
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/stores/anon_category.stpl")]
                struct Template {
                    all_tags:         Vec<SmallTag>,
                    category:         CatDetail,
                    cats:             Vec<Cat>,
                    help_cats:        Vec<Cat>,
                    object_list:      Vec<Store>,
                    next_page_number: i32,
                    is_ajax:          i32,
                    template_types:   u8,
                    linguage:         u8,
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
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
    }
}

pub async fn store_categories_page(session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let (t, l) = get_all_storage();
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);

    let title: String;
    let description: String;
    let link = "/store_categories/".to_string();
    let image = "/static/images/dark/store.jpg".to_string();
    if l == 2 {
        title = "Categories of market".to_string();
        description = "Web-services - Categories of market".to_string();
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
        let _stat = crate::models::StatPage::get_or_create(71);
        let _cats = block(move || Categories::get_categories_for_types(3, l)).await?;
        let _tags = block(move || Categories::get_tags(3, l)).await?;

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/stores/categories.stpl")]
                struct Template {
                    request_user:   User,
                    is_ajax:        i32,
                    cats:           Vec<Cat>,
                    all_tags:       Vec<SmallTag>,
                    stat:           StatPage,
                    template_types: u8,
                    linguage:       u8,
                }
                let body = Template {
                    request_user:   _request_user,
                    is_ajax:        is_ajax,
                    cats:           _cats,
                    all_tags:       _tags,
                    stat:           _stat,
                    template_types: t,
                    linguage:       l,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/stores/categories.stpl")]
                struct Template {
                    request_user: User,
                    is_ajax:        i32,
                    cats:           Vec<Cat>,
                    all_tags:       Vec<SmallTag>,
                    stat:           StatPage,
                    template_types: u8,
                    linguage:       u8,
                }
                let body = Template {
                    request_user:   _request_user,
                    is_ajax:        is_ajax,
                    cats:           _cats,
                    all_tags:       _tags,
                    stat:           _stat,
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
                #[template(path = "desctop/stores/anon_categories.stpl")]
                struct Template {
                    is_ajax:        i32,
                    cats:           Vec<Cat>,
                    all_tags:       Vec<SmallTag>,
                    stat:           StatPage,
                    template_types: u8,
                    linguage:       u8,
                }
                let body = Template {
                    is_ajax:        is_ajax,
                    cats:           _cats,
                    all_tags:       _tags,
                    stat:           _stat,
                    template_types: t,
                    linguage:       l,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/stores/anon_categories.stpl")]
                struct Template {
                    is_ajax:        i32,
                    cats:           Vec<Cat>,
                    all_tags:       Vec<SmallTag>,
                    stat:           StatPage,
                    template_types: u8,
                    linguage:       u8,
                }
                let body = Template {
                    is_ajax:        is_ajax,
                    cats:           _cats,
                    all_tags:       _tags,
                    stat:           _stat,
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
