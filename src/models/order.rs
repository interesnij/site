use crate::schema;
use crate::diesel::{
    Queryable,
    Insertable,
    QueryDsl,
    RunQueryDsl,
    ExpressionMethods,
};
use serde::{Serialize, Deserialize};
use crate::models::{Serve, WebService, ServeVar};
use crate::schema::{
    orders,
    order_files,
};
use crate::utils::{
    establish_connection,
};


#[derive(Debug, Serialize, Identifiable, Queryable)]
#[table_name="orders"]
pub struct Order {
    pub id:             i32,
    pub title:          String,
    pub title_en:       String,
    pub types:          i16,
    pub object_id:      i32,
    pub username:       String,
    pub email:          String,
    pub description:    Option<String>,
    pub description_en: Option<String>,
    pub created:        chrono::NaiveDateTime,
    pub user_id:        i32,
    pub price:          i32,
    pub price_acc:      Option<i32>,
}

impl Order {
    pub fn create(user_id: i32, form: crate::utils::OrderForms, l: i16) -> i16 {
        use crate::schema::serve::dsl::serve;
        use crate::models::{
            NewWebServicesItem,
            NewServeItems,
        };

        let _connection = establish_connection();
        let _order: Order;
        if user_id != 0 {
            if l == 1 {
                let new_order = NewOrder::create (
                    form.title.clone(),
                    "".to_string(),
                    form.types,
                    form.object_id,
                    form.username.clone(),
                    form.email.clone(),
                    form.description.clone(),
                    None,
                    user_id,
                );

                _order = diesel::insert_into(schema::orders::table)
                    .values(&new_order)
                    .get_result::<Order>(&_connection)
                    .expect("E.");
            }
            else {
                let new_order = NewOrder::create (
                    "".to_string(),
                    form.title.clone(),
                    form.types,
                    form.object_id,
                    form.username.clone(),
                    form.email.clone(),
                    None,
                    form.description,
                    user_id,
                );
                _order = diesel::insert_into(schema::orders::table)
                    .values(&new_order)
                    .get_result::<Order>(&_connection)
                    .expect("E.");
            }

            for file in form.files.iter() {
                let new_file = NewOrderFile::create (
                    _order.id,
                    file.to_string()
                );
                diesel::insert_into(schema::order_files::table)
                    .values(&new_file)
                    .execute(&_connection)
                    .expect("E.");
            };

            // создаем опции услуги и записываем id опций в вектор.
            let mut serve_ids = Vec::new();
            for serve_id in form.serve_list.into_iter() {
                let new_serve_form = NewServeItems {
                    serve_id: serve_id,
                    item_id:  form.object_id,
                    types:    form.types,
                };
                diesel::insert_into(schema::serve_items::table)
                    .values(&new_serve_form)
                    .execute(&_connection)
                    .expect("Error.");
                serve_ids.push(serve_id);
            }

            // получаем опции, чтобы создать связи с их тех. категорией.
            // это надо отрисовки тех категорий услуги, которые активны
            let _serves = serve
                .filter(schema::serve::id.eq_any(serve_ids))
                .load::<Serve>(&_connection)
                .expect("E");

            let mut web_service_ids = Vec::new();
            let mut order_price = 0;
            for _serve in _serves.iter() {
                if !web_service_ids.iter().any(|&i| i==_serve.web_service_id) {
                    web_service_ids.push(_serve.web_service_id);
                }
                order_price += _serve.price;
            }

            for id in web_service_ids.iter() {
                let new_cat = NewWebServicesItem {
                    category_id: *id,
                    item_id:     form.object_id,
                    types:       form.types,
                    is_active:   1,
                };
                diesel::insert_into(schema::web_services_items::table)
                    .values(&new_cat)
                    .execute(&_connection)
                    .expect("Error.");
            }

            // фух. Связи созданы все, но надо еще посчитать цену
            // услуги для калькулятора. Как? А это будет сумма всех
            // цен выбранных опций.
            let price_acc = crate::utils::get_price_acc_values(&order_price);
            diesel::update(&_order)
                .set((
                    schema::orders::price.eq(order_price),
                    schema::orders::price_acc.eq(price_acc),
                ))
                .execute(&_connection)
                .expect("Error.");
        }
        return 1;
    }
    pub fn get(id: i32) -> Order {
        let _connection = establish_connection();
        return schema::orders::table
            .filter(schema::orders::id.eq(id))
            .first::<Order>(&_connection)
            .expect("E")
    }
    pub fn delete(user_id: i32, id: i32) -> i16 {
        let _connection = establish_connection();
        let _order = schema::orders::table
            .filter(schema::orders::id.eq(id))
            .first::<Order>(&_connection)
            .expect("E");
        
        if user_id == _order.user_id {
            use crate::schema::{
                serve_items::dsl::serve_items,
                web_services_items::dsl::web_services_items,
            };
        
            diesel::delete (
                serve_items
                    .filter(schema::serve_items::item_id.eq(id))
                    .filter(schema::serve_items::types.eq(7))
                )
                .execute(&_connection)
                .expect("E");
            diesel::delete(
                web_services_items
                    .filter(schema::web_services_items::item_id.eq(id))
                    .filter(schema::web_services_items::types.eq(7))
                )
                .execute(&_connection)
                .expect("E");
            diesel::delete(
                schema::order_files::table
                    .filter(schema::order_files::order_id.eq(id))
                )
                .execute(&_connection)
                .expect("E");
            diesel::delete(&_order).execute(&_connection).expect("E");
            return 1;
        }
        return 0;
    }
    pub fn get_orders_list(page: i32, limit: i32) -> (Vec<Order>, i32) {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Order>;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = Order::get_orders(limit.into(), step.into());
        }
        else {
            have_next = limit + 1;
            object_list = Order::get_orders(limit.into(), 0);
        }
        if Order::get_orders(1, have_next.into()).len() > 0 {
            next_page_number = page + 1;
        }

        return (object_list, next_page_number);
    }
    pub fn get_orders(limit: i64, offset: i64) -> Vec<Order> {
        use crate::schema::orders::dsl::orders;

        let _connection = establish_connection();
        return orders
            .order(schema::orders::created.desc())
            .limit(limit)
            .offset(offset)
            .load::<Order>(&_connection)
            .expect("E.");
    }
    pub fn get_user_orders_list(user_id: i32, page: i32, limit: i32) -> (Vec<Order>, i32) {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Order>;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = Order::get_user_orders(user_id, limit.into(), step.into());
        }
        else {
            have_next = limit + 1;
            object_list = Order::get_user_orders(user_id, limit.into(), 0);
        }
        if Order::get_user_orders(user_id, 1, have_next.into()).len() > 0 {
            next_page_number = page + 1;
        }

        return (object_list, next_page_number);
    }
    pub fn get_user_orders(user_id: i32, limit: i64, offset: i64) -> Vec<Order> {
        use crate::schema::orders::dsl::orders;

        let _connection = establish_connection();
        return orders
            .filter(schema::orders::user_id.eq(user_id))
            .order(schema::orders::created.desc())
            .limit(limit)
            .offset(offset)
            .load::<Order>(&_connection)
            .expect("E.");
    }
    pub fn get_serves(&self, l: i16) -> Vec<ServeVar> {
        use schema::serve_items::dsl::serve_items;
        use schema::serve::dsl::serve;

        let _connection = establish_connection();
        let _serve_items = serve_items
            .filter(schema::serve_items::item_id.eq(&self.id))
            .filter(schema::serve_items::types.eq(7))
            .select(schema::serve_items::serve_id)
            .load::<i32>(&_connection)
            .expect("E");
        if l == 1 {
            return serve 
                .filter(schema::serve::id.eq_any(_serve_items))
                .order(schema::serve::position)
                .select((
                    schema::serve::id,
                    schema::serve::name,
                    schema::serve::price,
                    schema::serve::man_hours,
                    schema::serve::is_default,
                    schema::serve::serve_id,
                ))
                .load::<ServeVar>(&_connection)
                .expect("E");
        }
        else if l == 2 {
            let mut list = serve 
                .filter(schema::serve::id.eq_any(_serve_items))
                .order(schema::serve::position)
                .select((
                    schema::serve::id,
                    schema::serve::name_en,
                    schema::serve::price,
                    schema::serve::man_hours,
                    schema::serve::is_default,
                    schema::serve::serve_id,
                ))
                .load::<ServeVar>(&_connection)
                .expect("E");
            for i in &mut list {
                i.price = i.price / 100;
            }
            return list;
        }
        return Vec::new();
    }
    pub fn get_serves_ids(&self) -> Vec<i32> {
        use schema::serve_items::dsl::serve_items;

        let _connection = establish_connection();
        return serve_items
            .filter(schema::serve_items::item_id.eq(&self.id))
            .filter(schema::serve_items::types.eq(7))
            .select(schema::serve_items::serve_id)
            .load::<i32>(&_connection)
            .expect("E");
    }
    pub fn get_open_web_services(&self) -> Vec<WebService> {
        // получаем открытые тех.категории элемента
        use schema::{
            web_services_items::dsl::web_services_items,
            web_services::dsl::web_services,
        };

        let _connection = establish_connection();
        let ids = web_services_items
            .filter(schema::web_services_items::item_id.eq(&self.id))
            .filter(schema::web_services_items::types.eq(7))
            .filter(schema::web_services_items::is_active.eq(1))
            .select(schema::web_services_items::category_id)
            .load::<i32>(&_connection)
            .expect("E");

        return web_services
            .filter(schema::web_services::id.eq_any(ids))
            .order(schema::web_services::position.desc())
            .load::<WebService>(&_connection)
            .expect("E");
    }
}

#[derive(Insertable)]
#[table_name="orders"]
pub struct NewOrder {
    pub title:          String,
    pub title_en:       String,
    pub types:          i16,
    pub object_id:      i32,
    pub username:       String,
    pub email:          String,
    pub description:    Option<String>,
    pub description_en: Option<String>,
    pub created:        chrono::NaiveDateTime,
    pub user_id:        i32,
    pub price:          i32,
}
impl NewOrder {
    pub fn create (
        title:          String,
        title_en:       String,
        types:          i16,
        object_id:      i32,
        username:       String,
        email:          String,
        description:    Option<String>,
        description_en: Option<String>,
        user_id:        i32,
    ) -> Self {
        use chrono::Duration;

        NewOrder {
            title:          title,
            title_en:       title_en,
            types:          types,
            object_id:      object_id,
            username:       username,
            email:          email,
            description:    description,
            description_en: description_en,
            created:        chrono::Local::now().naive_utc() + Duration::hours(3),
            user_id:        user_id,
            price:          0,
        }
    }
}
#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="orders"]
pub struct EditOrder {
    pub username:       String,
    pub email:          String,
    pub description:    Option<String>,
    pub description_en: Option<String>,
}


#[derive(Debug, Serialize, Queryable, Identifiable)]
pub struct OrderFile {
    pub id:       i32,
    pub order_id: i32,
    pub src:      String,
}
impl OrderFile { 
    pub fn get_object_files(id: i32) -> Vec<OrderFile> {
        let _connection = establish_connection();
        return schema::order_files::table
            .filter(schema::order_files::order_id.eq(id))
            .load::<OrderFile>(&_connection)
            .expect("E");
    }
    pub fn delete(user_id: i32, id: i32) -> i16 {
        let _connection = establish_connection();
        let _order = schema::orders::table
            .filter(schema::orders::id.eq(id))
            .first::<Order>(&_connection)
            .expect("E");
        
        if user_id == _order.user_id {
            use crate::schema::{
                serve_items::dsl::serve_items,
                web_services_items::dsl::web_services_items,
            };
        
            diesel::delete (
                serve_items
                    .filter(schema::serve_items::item_id.eq(id))
                    .filter(schema::serve_items::types.eq(7))
                )
                .execute(&_connection)
                .expect("E");
            diesel::delete(
                web_services_items
                    .filter(schema::web_services_items::item_id.eq(id))
                    .filter(schema::web_services_items::types.eq(7))
                )
                .execute(&_connection)
                .expect("E");
            diesel::delete(
                schema::order_files::table
                    .filter(schema::order_files::order_id.eq(id))
                )
                .execute(&_connection)
                .expect("E");
            diesel::delete(&_order).execute(&_connection).expect("E");
            return 1;
        }
        return 0;
    }
    pub fn create(user_id: i32, form: crate::utils::OrderForms, l: i16) -> i16 {
        use crate::models::{
            NewWebServicesItem,
            NewServeItems,
        };

        let _connection = establish_connection();
        let _order: Order;
        if l == 1 {
            let new_order = NewOrder::create (
                form.title.clone(),
                "".to_string(),
                form.types,
                form.object_id,
                form.username.clone(),
                form.email.clone(),
                form.description.clone(),
                None,
                user_id,
            );
            _order = diesel::insert_into(schema::orders::table)
                .values(&new_order)
                .get_result::<Order>(&_connection)
                .expect("E.");
        }
        else if l == 2 {
            let new_order = NewOrder::create (
                "".to_string(),
                form.title.clone(),
                form.types,
                form.object_id,
                form.username.clone(),
                form.email.clone(),
                None,
                form.description.clone(),
                user_id,
            );
            _order = diesel::insert_into(schema::orders::table)
                .values(&new_order)
                .get_result::<Order>(&_connection)
                .expect("E.");
        }
        else {
            return 0;
        }

        for file in form.files.iter() {
            let new_file = NewOrderFile::create (
                _order.id,
                file.to_string()
            );
            diesel::insert_into(schema::order_files::table)
                .values(&new_file)
                .execute(&_connection)
                .expect("E.");
        };

        // создаем опции услуги и записываем id опций в вектор.
        let mut serve_ids = Vec::new();
        for serve_id in form.serve_list.into_iter() {
            let new_serve_form = NewServeItems {
                serve_id: serve_id,
                item_id:  form.object_id,
                types:    form.types,
            };
            diesel::insert_into(schema::serve_items::table)
                .values(&new_serve_form)
                .execute(&_connection)
                .expect("Error.");
            serve_ids.push(serve_id);
        }

        // получаем опции, чтобы создать связи с их тех. категорией.
        // это надо отрисовки тех категорий услуги, которые активны
        let _serves = schema::serve::table
            .filter(schema::serve::id.eq_any(serve_ids))
            .load::<Serve>(&_connection)
            .expect("E");

        let mut web_service_ids = Vec::new();
        let mut order_price = 0;
        for _serve in _serves.iter() {
            if !web_service_ids.iter().any(|&i| i==_serve.web_service_id) {
                web_service_ids.push(_serve.web_service_id);
            }
            order_price += _serve.price;
        }

        for id in web_service_ids.iter() {
            let new_cat = NewWebServicesItem {
                category_id: *id,
                item_id:     form.object_id,
                types:       form.types,
                is_active:   1,
            };
            diesel::insert_into(schema::web_services_items::table)
                .values(&new_cat)
                .execute(&_connection)
                .expect("Error.");
        }

        // фух. Связи созданы все, но надо еще посчитать цену
        // услуги для калькулятора. Как? А это будет сумма всех
        // цен выбранных опций.
        let price_acc = crate::utils::get_price_acc_values(&order_price);
        diesel::update(&_order)
            .set((
                schema::orders::price.eq(order_price),
                schema::orders::price_acc.eq(price_acc),
            ))
            .execute(&_connection)
            .expect("Error.");

        return 1;
    }
}

#[derive(Serialize, Insertable)]
#[table_name="order_files"]
pub struct NewOrderFile {
    pub order_id: i32,
    pub src:      String,
}

impl NewOrderFile {
    pub fn create (order_id: i32, src: String) -> Self {
        NewOrderFile {
            order_id: order_id,
            src:      src,
        }
    }
}
