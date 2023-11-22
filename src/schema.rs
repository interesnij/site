// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 100]
        name_en -> Varchar,
        #[max_length = 500]
        description -> Nullable<Varchar>,
        #[max_length = 500]
        description_en -> Nullable<Varchar>,
        position -> Int2,
        #[max_length = 500]
        image -> Nullable<Varchar>,
        count -> Int2,
        view -> Int4,
        height -> Float8,
        seconds -> Int4,
        types -> Int2,
        #[max_length = 100]
        slug -> Varchar,
    }
}

diesel::table! {
    category (id) {
        id -> Int4,
        category_id -> Int4,
        item_id -> Int4,
        types -> Int2,
    }
}

diesel::table! {
    chats (id) {
        id -> Int4,
        user_id -> Int4,
        created -> Timestamp,
    }
}

diesel::table! {
    cookie_stats (id) {
        id -> Int4,
        user_id -> Int4,
        page -> Int2,
        #[max_length = 200]
        link -> Varchar,
        #[max_length = 200]
        title -> Varchar,
        height -> Float8,
        seconds -> Int4,
        created -> Timestamp,
        #[max_length = 100]
        template -> Varchar,
    }
}

diesel::table! {
    cookie_users (id) {
        id -> Int4,
        #[max_length = 100]
        ip -> Varchar,
        device -> Int2,
        linguage -> Int2,
        template -> Int2,
        #[max_length = 10]
        currency -> Varchar,
        #[max_length = 150]
        city_ru -> Nullable<Varchar>,
        #[max_length = 150]
        city_en -> Nullable<Varchar>,
        #[max_length = 150]
        region_ru -> Nullable<Varchar>,
        #[max_length = 150]
        region_en -> Nullable<Varchar>,
        #[max_length = 150]
        country_ru -> Nullable<Varchar>,
        #[max_length = 150]
        country_en -> Nullable<Varchar>,
        height -> Float8,
        seconds -> Int4,
        created -> Timestamp,
    }
}

diesel::table! {
    feedbacks (id) {
        id -> Int4,
        #[max_length = 100]
        username -> Varchar,
        #[max_length = 200]
        email -> Varchar,
        #[max_length = 1000]
        message -> Varchar,
    }
}

diesel::table! {
    files (id) {
        id -> Int4,
        user_id -> Int4,
        item_id -> Int4,
        item_types -> Int2,
        types -> Int2,
        #[max_length = 500]
        src -> Varchar,
        #[max_length = 500]
        description -> Nullable<Varchar>,
        #[max_length = 500]
        description_en -> Nullable<Varchar>,
        position -> Int2,
        view -> Int4,
        seconds -> Int4,
    }
}

diesel::table! {
    item_comments (id) {
        id -> Int4,
        #[max_length = 1000]
        comment -> Varchar,
        item_id -> Int4,
        user_id -> Int4,
        parent_id -> Nullable<Int4>,
        created -> Timestamp,
    }
}

diesel::table! {
    items (id) {
        id -> Int4,
        #[max_length = 100]
        title -> Varchar,
        #[max_length = 100]
        title_en -> Varchar,
        #[max_length = 500]
        description -> Nullable<Varchar>,
        #[max_length = 500]
        description_en -> Nullable<Varchar>,
        #[max_length = 30000]
        content -> Nullable<Varchar>,
        #[max_length = 30000]
        content_en -> Nullable<Varchar>,
        #[max_length = 500]
        link -> Nullable<Varchar>,
        #[max_length = 500]
        image -> Nullable<Varchar>,
        is_active -> Bool,
        price -> Int4,
        user_id -> Int4,
        created -> Timestamp,
        position -> Int2,
        view -> Int4,
        height -> Float8,
        seconds -> Int4,
        price_acc -> Nullable<Int4>,
        types -> Int2,
        #[max_length = 100]
        slug -> Varchar,
    }
}

diesel::table! {
    messages (id) {
        id -> Int4,
        user_id -> Int4,
        chat_id -> Int4,
        created -> Timestamp,
        #[max_length = 5000]
        content -> Nullable<Varchar>,
        view -> Int2,
        types -> Int2,
    }
}

diesel::table! {
    order_files (id) {
        id -> Int4,
        order_id -> Int4,
        #[max_length = 500]
        src -> Varchar,
    }
}

diesel::table! {
    orders (id) {
        id -> Int4,
        #[max_length = 100]
        title -> Varchar,
        #[max_length = 100]
        title_en -> Varchar,
        types -> Int2,
        object_id -> Int4,
        #[max_length = 200]
        username -> Varchar,
        #[max_length = 200]
        email -> Varchar,
        #[max_length = 1000]
        description -> Nullable<Varchar>,
        #[max_length = 1000]
        description_en -> Nullable<Varchar>,
        created -> Timestamp,
        user_id -> Int4,
        price -> Int4,
        price_acc -> Nullable<Int4>,
    }
}

diesel::table! {
    price_corrects (id) {
        id -> Int4,
        #[max_length = 10]
        currency -> Varchar,
        ratio -> Float8,
        adder -> Int4,
        created -> Timestamp,
    }
}

diesel::table! {
    serve (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 100]
        name_en -> Varchar,
        #[max_length = 10000]
        description -> Nullable<Varchar>,
        #[max_length = 10000]
        description_en -> Nullable<Varchar>,
        position -> Int2,
        category_id -> Int4,
        price -> Int4,
        man_hours -> Int2,
        is_default -> Bool,
        user_id -> Int4,
        web_service_id -> Int4,
        height -> Float8,
        seconds -> Int4,
        serve_id -> Nullable<Int4>,
        view -> Int4,
    }
}

diesel::table! {
    serve_categories (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 100]
        name_en -> Varchar,
        #[max_length = 10000]
        description -> Nullable<Varchar>,
        #[max_length = 10000]
        description_en -> Nullable<Varchar>,
        category_id -> Int4,
        position -> Int2,
        count -> Int2,
        default_price -> Int4,
        user_id -> Int4,
        view -> Int4,
        height -> Float8,
        seconds -> Int4,
    }
}

diesel::table! {
    serve_items (id) {
        id -> Int4,
        serve_id -> Int4,
        item_id -> Int4,
        types -> Int2,
    }
}

diesel::table! {
    stat_pages (id) {
        id -> Int4,
        types -> Int2,
        view -> Int4,
        height -> Float8,
        seconds -> Int4,
    }
}

diesel::table! {
    tags (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 100]
        name_en -> Varchar,
        position -> Int2,
        count -> Int2,
        user_id -> Int4,
        view -> Int4,
        height -> Float8,
        seconds -> Int4,
    }
}

diesel::table! {
    tags_items (id) {
        id -> Int4,
        tag_id -> Int4,
        item_id -> Int4,
        types -> Int2,
        created -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 100]
        username -> Varchar,
        #[max_length = 100]
        email -> Varchar,
        #[max_length = 1000]
        password -> Varchar,
        #[max_length = 500]
        bio -> Nullable<Varchar>,
        #[max_length = 500]
        image -> Nullable<Varchar>,
        perm -> Int2,
    }
}

diesel::table! {
    web_services (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 100]
        name_en -> Varchar,
        #[max_length = 10000]
        description -> Nullable<Varchar>,
        #[max_length = 10000]
        description_en -> Nullable<Varchar>,
        position -> Int2,
        count -> Int2,
        level -> Int2,
        user_id -> Int4,
        view -> Int4,
        height -> Float8,
        seconds -> Int4,
    }
}

diesel::table! {
    web_services_items (id) {
        id -> Int4,
        category_id -> Int4,
        item_id -> Int4,
        types -> Int2,
        is_active -> Int2,
    }
}

diesel::joinable!(category -> categories (category_id));
diesel::joinable!(category -> items (item_id));
diesel::joinable!(chats -> users (user_id));
diesel::joinable!(cookie_stats -> cookie_users (user_id));
diesel::joinable!(item_comments -> items (item_id));
diesel::joinable!(item_comments -> users (user_id));
diesel::joinable!(items -> users (user_id));
diesel::joinable!(messages -> users (user_id));
diesel::joinable!(order_files -> orders (order_id));
diesel::joinable!(serve -> serve_categories (category_id));
diesel::joinable!(serve -> users (user_id));
diesel::joinable!(serve_categories -> web_services (category_id));
diesel::joinable!(tags -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    category,
    chats,
    cookie_stats,
    cookie_users,
    feedbacks,
    files,
    item_comments,
    items,
    messages,
    order_files,
    orders,
    price_corrects,
    serve,
    serve_categories,
    serve_items,
    stat_pages,
    tags,
    tags_items,
    users,
    web_services,
    web_services_items,
);
