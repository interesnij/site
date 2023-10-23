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


pub fn work_routes(config: &mut web::ServiceConfig) {
    config.route("/work_categories/", web::get().to(work_categories_page));
    config.service(web::resource("/work/{cat_slug}/{work_slug}/").route(web::get().to(get_work_page)));
    config.service(web::resource("/works/{slug}/").route(web::get().to(work_category_page)));
}


pub async fn get_work_page(conn: ConnectionInfo, session: Session, req: HttpRequest, param: web::Path<(String,String)>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let (l, t, c) = crate::utils::get_or_create_c_user_return_ltc(conn, &req).await;
    let _item_id: String = param.1.clone();
    let _cat_id: String = param.0.clone();

    let _item = Item::get(&_item_id);

    let title: String;
    let description: String;
    let link = "/work/".to_string() + &_cat_id + &"/".to_string() + &_item_id.to_string() + &"/".to_string();
    let image = _item.get_image();
    if l == 2 {
        title = String::new() + &_item.title_en + &" | Work ".to_string();
        description = String::new() + &_item.title_en + &" | Work: Web-services".to_string();
    }
    else {
        title = String::new() + &_item.title + &" | Работа ".to_string();
        description = String::new() + &_item.title + &" | Работа: вебсервисы.рф".to_string();
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
        let _cats = block(move || Categories::get_categories_for_types(5, l)).await?;
        let _tags = block(move || Categories::get_tags(5, l)).await?;
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
                    &c,
                ).await
            }
            else if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/works/work.stpl")]
                struct Template {
                    request_user:   User,
                    object:         Item,
                    category:       Categories,
                    //help_cats:      Vec<Cat>,
                    prev:           Option<FeaturedItem>,
                    next:           Option<FeaturedItem>,
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
                    object:         _item,
                    category:       _category,
                    //help_cats:      _help_cats,
                    prev:           prev,
                    next:           next,
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
                #[template(path = "mobile/works/work.stpl")]
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
                    currency:       String,
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
                    &c,
                ).await
            }
            else if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/works/anon_work.stpl")]
                struct Template {
                    object:         Item,
                    category:       Categories,
                    //help_cats:      Vec<Cat>,
                    prev:           Option<FeaturedItem>,
                    next:           Option<FeaturedItem>,
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
                    object:         _item,
                    category:       _category,
                    //help_cats:      _help_cats,
                    prev:            prev,
                    next:           next,
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
                #[template(path = "mobile/works/anon_work.stpl")]
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
                    currency:       String,
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

pub async fn work_category_page(conn: ConnectionInfo, session: Session, req: HttpRequest, _id: web::Path<String>) -> actix_web::Result<HttpResponse> {
    let (l, t, c) = crate::utils::get_or_create_c_user_return_ltc(conn, &req).await;
    let _category = Categories::get_detail(_id.clone(), 5, l);

    let title: String;
    let description: String;
    let link = "/works/".to_string() + &_category.slug + &"/".to_string();
    let image = _category.get_image();
    if l == 2 {
        title = String::new() + &_category.name + &" | Category of the works".to_string();
        description = String::new() + &_category.name + &" | Category of the works: Web-services".to_string();
    }
    else {
        title = String::new() + &_category.name + &" | Категория работ".to_string();
        description = String::new() + &_category.name + &" | Категория работ: вебсервисы.рф".to_string();
    }

    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
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
        use crate::models::Work;

        let page = crate::utils::get_page(&req);
        let object_list: Vec<Work>;
        let next_page_number: i32;
        let _cats = block(move || Categories::get_categories_for_types(5, l)).await?;
        let _tags = block(move || Categories::get_tags(5, l)).await?;
        let _help_cats = block(move || Categories::get_categories_for_types(6, l)).await?;

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            let _res = block(move || Categories::get_works_list(_category.id, page, 20, _request_user.perm == 60, l)).await?;
            let _dict = match _res {
                Ok(_ok) => {object_list = _ok.0; next_page_number = _ok.1},
                Err(_error) => {object_list = Vec::new(); next_page_number = 0},
            };
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/works/category.stpl")]
                struct Template {
                    request_user:     User,
                    all_tags:         Vec<SmallTag>,
                    category:         CatDetail,
                    object_list:      Vec<Work>,
                    help_cats:        Vec<Cat>,
                    next_page_number: i32,
                    is_ajax:          i32,
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
                    all_tags:         _tags,
                    category:         _category,
                    object_list:      object_list,
                    help_cats:        _help_cats,
                    next_page_number: next_page_number,
                    is_ajax:          is_ajax,
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
                #[template(path = "mobile/works/category.stpl")]
                struct Template {
                    all_tags:         Vec<SmallTag>,
                    category:         CatDetail,
                    cats:             Vec<Cat>,
                    object_list:      Vec<Work>,
                    help_cats:        Vec<Cat>,
                    next_page_number: i32,
                    is_ajax:          i32,
                    template_types:   i16,
                    linguage:         i16,
                    currency:         String,
                    title:            String,
                    description:      String,
                    link:             String,
                    image:            String,
                }
                let body = Template {
                    all_tags:         _tags,
                    category:         _category,
                    cats:             _cats,
                    object_list:      object_list,
                    help_cats:        _help_cats,
                    next_page_number: next_page_number,
                    is_ajax:          is_ajax,
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
            let _res = block(move || Categories::get_works_list(_category.id, page, 20, false, l)).await?;
            let _dict = match _res {
                Ok(_ok) => {object_list = _ok.0; next_page_number = _ok.1},
                Err(_error) => {object_list = Vec::new(); next_page_number = 0},
            };

            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/works/anon_category.stpl")]
                struct Template {
                    all_tags:         Vec<SmallTag>,
                    category:         CatDetail,
                    object_list:      Vec<Work>,
                    help_cats:        Vec<Cat>,
                    next_page_number: i32,
                    is_ajax:          i32,
                    template_types:   i16,
                    linguage:         i16,
                    currency:         String,
                    title:            String,
                    description:      String,
                    link:             String,
                    image:            String,
                }
                let body = Template {
                    all_tags:         _tags,
                    category:         _category,
                    object_list:      object_list,
                    help_cats:        _help_cats,
                    next_page_number: next_page_number,
                    is_ajax:          is_ajax,
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
                #[template(path = "mobile/works/anon_category.stpl")]
                struct Template {
                    all_tags:         Vec<SmallTag>,
                    category:         CatDetail,
                    cats:             Vec<Cat>,
                    object_list:      Vec<Work>,
                    help_cats:        Vec<Cat>,
                    next_page_number: i32,
                    is_ajax:          i32,
                    template_types:   i16,
                    linguage:         i16,
                    currency:         String,
                    title:            String,
                    description:      String,
                    link:             String,
                    image:            String,
                }
                let body = Template {
                    all_tags:         _tags,
                    category:         _category,
                    cats:             _cats,
                    object_list:      object_list,
                    help_cats:        _help_cats,
                    next_page_number: next_page_number,
                    is_ajax:          is_ajax,
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

pub async fn work_categories_page(conn: ConnectionInfo, session: Session, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let (l, t, c) = crate::utils::get_or_create_c_user_return_ltc(conn, &req).await;

    let title: String;
    let description: String;
    let link = "/work_categories/".to_string();
    let image = "/static/images/dark/store.jpg".to_string();
    if l == 2 {
        title = "Categories of works".to_string();
        description = "Web-services - Categories of works".to_string();
    }
    else {
        title = "Категории работ".to_string();
        description = "вебсервисы.рф - Категории работ".to_string();
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
        let _stat = crate::models::StatPage::get_or_create(91);
        let _cats = block(move || Categories::get_categories_for_types(5, l)).await?;
        let _tags = block(move || Categories::get_tags(5, l)).await?;
        let _help_cats = block(move || Categories::get_categories_for_types(6, l)).await?;

        if is_signed_in(&session) {
            let _request_user = get_request_user_data(&session);
            if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/works/categories.stpl")]
                struct Template {
                    request_user:   User,
                    is_ajax:        i32,
                    cats:           Vec<Cat>,
                    help_cats:      Vec<Cat>,
                    stat:           StatPage,
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
                    cats:           _cats,
                    help_cats:      _help_cats,
                    stat:           _stat,
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
                #[template(path = "mobile/works/categories.stpl")]
                struct Template {
                    is_ajax:        i32,
                    cats:           Vec<Cat>,
                    help_cats:      Vec<Cat>,
                    all_tags:       Vec<SmallTag>,
                    stat:           StatPage,
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
                    cats:           _cats,
                    help_cats:      _help_cats,
                    all_tags:       _tags,
                    stat:           _stat,
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
                #[template(path = "desctop/works/anon_categories.stpl")]
                struct Template {
                    is_ajax:        i32,
                    cats:           Vec<Cat>,
                    help_cats:      Vec<Cat>,
                    stat:           StatPage,
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
                    cats:           _cats,
                    help_cats:      _help_cats,
                    stat:           _stat,
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
                #[template(path = "mobile/works/anon_categories.stpl")]
                struct Template {
                    is_ajax:        i32,
                    cats:           Vec<Cat>,
                    help_cats:      Vec<Cat>,
                    all_tags:       Vec<SmallTag>,
                    stat:           StatPage,
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
                    cats:           _cats,
                    help_cats:      _help_cats,
                    all_tags:       _tags,
                    stat:           _stat,
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
