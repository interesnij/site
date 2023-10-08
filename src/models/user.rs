use crate::schema;
use crate::schema::{
    users,
    cookie_users,
    cookie_stats,
    stat_pages,
};
use crate::diesel::{
    Queryable,
    Insertable,
    QueryDsl,
    ExpressionMethods,
    RunQueryDsl,
    Connection,
};
use serde::{Serialize, Deserialize};
use crate::utils::{establish_connection, NewUserForm};
use crate::errors::Error;
use actix_web::web::Json;


#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct User {
    pub id:       i32,
    pub username: String,
    pub email:    String,
    pub password: String,
    pub bio:      Option<String>,
    pub image:    Option<String>,
    pub perm:     i16,
}

impl User {
    pub fn is_superuser(&self) -> bool {
        return self.perm > 59;
    }
    pub fn create_superuser(user_id: i32) -> Result<(), Error> {
        let _connection = establish_connection();
        _connection.transaction(|| Ok({
            let _u = diesel::update(users::table.filter(users::id.eq(user_id)))
                .set(schema::users::perm.eq(60))
                .execute(&_connection);
        }))
    }
    pub fn get_user_with_username(username: &String) -> Result<User, Error> {
        let _connection = establish_connection();
        return Ok(schema::users::table
            .filter(schema::users::username.eq(username))
            .first::<User>(&_connection)?);
    }
    pub fn create(form: NewUserForm) -> User {
        let _connection = establish_connection();
        let form_user = NewUser {
            username: form.username.clone(),
            email:    form.email.clone(),
            password: crate::utils::hash_password(&form.password),
            bio:      None,
            image:    None,
            perm:     1,
        };

        let _new_user = diesel::insert_into(schema::users::table)
            .values(&form_user)
            .get_result::<User>(&_connection)
            .expect("Error saving user.");
        return _new_user;
    }
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub username: String,
    pub email:    String,
    pub password: String,
    pub bio:      Option<String>,
    pub image:    Option<String>,
    pub perm:     i16,
}

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

#[derive(Debug, AsChangeset)]
#[table_name = "users"]
pub struct UserChange {
    pub username: String,
    pub email:    String,
    pub password: String,
    pub bio:      String,
    pub image:    String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionUser {
    pub id:       i32,
    pub username: String,
}

#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct CookieUser {
    pub id:         i32,
    pub ip:         String,
    pub device:     i16,
    pub city_ru:    Option<String>,
    pub city_en:    Option<String>,
    pub region_ru:  Option<String>,
    pub region_en:  Option<String>,
    pub country_ru: Option<String>,
    pub country_en: Option<String>,
    pub height:     f64,
    pub seconds:    i32,
    pub created:    chrono::NaiveDateTime,
}
impl CookieUser {
    pub fn get(user_id: i32) -> CookieUser {
        let _connection = establish_connection();
        return schema::cookie_users::table
            .filter(schema::cookie_users::id.eq(user_id))
            .first::<CookieUser>(&_connection)
            .expect("Error"); 
    }
    pub fn get_res(user_id: i32) -> Result<CookieUser, Error> {
        let _connection = establish_connection();
        return schema::cookie_users::table
            .filter(schema::cookie_users::id.eq(user_id))
            .first::<CookieUser>(&_connection); 
    }
    pub fn get_users_list(page: i32, limit: i32) -> (Vec<CookieUser>, i32) {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<CookieUser>;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = CookieUser::get_users(limit.into(), step.into());
        }
        else {
            have_next = limit + 1;
            object_list = CookieUser::get_users(limit.into(), 0);
        }
        if CookieUser::get_users(1, have_next.into()).len() > 0 {
            next_page_number = page + 1;
        }

        return (object_list, next_page_number);
    }
    pub fn get_users(limit: i64, offset: i64) -> Vec<CookieUser> {
        use crate::schema::cookie_users::dsl::cookie_users;

        let _connection = establish_connection();
        return cookie_users
            .filter(schema::cookie_users::seconds.ne(0))
            .filter(schema::cookie_users::height.ne(0.0))
            .order(schema::cookie_users::created.desc())
            .limit(limit)
            .offset(offset)
            .load::<CookieUser>(&_connection)
            .expect("E.");
    }
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="cookie_users"]
pub struct NewCookieUser {
    pub ip:         String,
    pub device:     i16,
    pub city_ru:    Option<String>,
    pub city_en:    Option<String>,
    pub region_ru:  Option<String>,
    pub region_en:  Option<String>,
    pub country_ru: Option<String>,
    pub country_en: Option<String>,
    pub height:     f64,
    pub seconds:    i32,
    pub created:    chrono::NaiveDateTime,
}

/////////////////////////
// Шифры посещаемых страниц
// 1 - главная
// 2 - о сайте
// 3 - контакты
// 4 - команда
// 5 - сотрудничество
// 6 - вход
// 7 - регитрация
// 8 - выход
// 9 - вопросы ответы
// 10 - инфо

// 11 - профиль
// 12 - заказы
// 13 - история
// 14 - статистика

// 21 - общий поиск
// 22 - поиск статей блога
// 23 - поиск услуг
// 24 - поиск товаров
// 25 - поиск статей обучающих
// 26 - поиск работ

// 31 - теги
// 32 - тег
// 33 - тег - статьи блога
// 34 - тег - услуги
// 35 - тег - товары
// 36 - тег - статьи обучающие
// 37 - тег - работы
// 38 - тег - Пооощь

// 41 - категории блога
// 42 - категория блога
// 43 - статья блога

// 51 - категории опций
// 52 - категория опций
// 53 - технологии опций
// 54 - технология опций
// 55 - опция

// 61 - категории услуг
// 62 - категория услуг
// 63 - услуга

// 71 - категории товаров
// 72 - категория товаров
// 73 - товар

// 81 - категории обучения
// 82 - категория обучения
// 83 - статья обучения

// 91 - категории работ
// 92 - категория работ
// 93 - работа
////////////////////

#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct CookieStat { 
    pub id:       i32,
    pub user_id:  i32,
    pub page:     i16,
    pub link:     String,
    pub title:    String,
    pub title_en: String,
    pub height:   f64,
    pub seconds:  i32,
    pub created:  chrono::NaiveDateTime,
    pub template: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryResponse {
    pub id:       i32,
    pub link:     String,
    pub title:    String,
    pub title_en: String,
    pub height:   f64,
    pub seconds:  i32,
    pub template: String,
}

impl CookieStat {
    pub fn get_stat_list(user_id: i32, page: i32, limit: i32) -> Result<(Vec<CookieStat>, i32), Error> {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<CookieStat>;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = CookieStat::get_stat_items(user_id, limit.into(), step.into())?;
        }
        else {
            have_next = limit + 1;
            object_list = CookieStat::get_stat_items(user_id, limit.into(), 0)?;
        }
        if CookieStat::get_stat_items(user_id, 1, have_next.into())?.len() > 0 {
            next_page_number = page + 1;
        }
        let _tuple = (object_list, next_page_number);
        Ok(_tuple)
    }
    pub fn get_stat_items(user_id: i32, limit: i64, offset: i64) -> Result<Vec<CookieStat>, Error> {
        use crate::schema::cookie_stats::dsl::cookie_stats;

        let _connection = establish_connection();
        let list = cookie_stats
            .filter(schema::cookie_stats::user_id.eq(user_id))
            .order(schema::cookie_stats::created.desc())
            .limit(limit)
            .offset(offset)
            .load::<CookieStat>(&_connection)
            .expect("E");
        Ok(list)
    }
    pub fn create (
        data: Json<crate::utils::HistoryData>,
        user: CookieUser, 
        l:    u8,
    ) -> Result<CookieStat, Error> {
        use chrono::Duration;

        let p_object_id = data.object_id;
        let p_page_id = data.page_id;
        let p_height = data.height;

        let p_seconds = data.seconds;
        if p_seconds < 3 {
            return Err(Error::BadRequest("Permission Denied".to_string()));
        }
        let p_link = data.link.clone();
        let p_title = data.title.clone();
        let p_title_en = data.title_en.clone();
        let p_template = data.template.clone();

        let _connection = establish_connection();
        let is_cookie_stats_exists = schema::cookie_stats::table
            .filter(schema::cookie_stats::user_id.eq(user.id))
            .filter(schema::cookie_stats::link.eq(p_link.clone()))
            .select(schema::cookie_stats::id)
            .first::<i32>(&_connection)
            .is_ok();

        if is_cookie_stats_exists {
            diesel::update(&user)
                .set ((
                    schema::cookie_users::height.eq(user.height + p_height),
                    schema::cookie_users::seconds.eq(user.seconds + p_seconds),
                ))
                .execute(&_connection)
                .expect("Error.");
        }
        if p_object_id > 0 {
            match p_page_id {
                42 => {
                    crate::utils::plus_category_stat(p_object_id, p_height, p_seconds, is_cookie_stats_exists)
                },
                43 => {
                    crate::utils::plus_item_stat(p_object_id, p_height, p_seconds, is_cookie_stats_exists)
                },
                62 => {
                    crate::utils::plus_category_stat(p_object_id, p_height, p_seconds, is_cookie_stats_exists)
                },
                63 => {
                    crate::utils::plus_item_stat(p_object_id, p_height, p_seconds, is_cookie_stats_exists)
                },
                72 => {
                    crate::utils::plus_category_stat(p_object_id, p_height, p_seconds, is_cookie_stats_exists)
                },
                73 => {
                    crate::utils::plus_item_stat(p_object_id, p_height, p_seconds, is_cookie_stats_exists)
                },
                82 => {
                    crate::utils::plus_category_stat(p_object_id, p_height, p_seconds, is_cookie_stats_exists)
                },
                83 => {
                    crate::utils::plus_item_stat(p_object_id, p_height, p_seconds, is_cookie_stats_exists)
                },
                92 => {
                    crate::utils::plus_category_stat(p_object_id, p_height, p_seconds, is_cookie_stats_exists)
                },
                93 => {
                    crate::utils::plus_item_stat(p_object_id, p_height, p_seconds, is_cookie_stats_exists)
                },
                32 => {
                    crate::utils::plus_tag_stat(p_object_id, p_height, p_seconds, is_cookie_stats_exists)
                },
                9 => {
                    crate::utils::plus_category_stat(p_object_id, p_height, p_seconds, is_cookie_stats_exists)
                },
                _ => (),
            };
        }
        else {
            crate::utils::plus_page_stat(p_page_id, p_height, p_seconds, is_cookie_stats_exists)
        }

        let _connection = establish_connection();
        if l == 1 {
            let _h = NewCookieStat {
                user_id:  user.id,
                page:     p_page_id,
                link:     p_link,
                title:    p_title,
                title_en: "".to_string(),
                height:   p_height,
                seconds:  p_seconds,
                created:  chrono::Local::now().naive_utc() + Duration::hours(3),
                template: p_template,
            };
            let new = diesel::insert_into(schema::cookie_stats::table)
                .values(&_h)
                .get_result::<CookieStat>(&_connection)?;
                return Ok(new);
        }
        else if l == 2 {
            let _h = NewCookieStat {
                user_id:  user.id,
                page:     p_page_id,
                link:     p_link,
                title:    "".to_string(),
                title_en: p_title_en,
                height:   p_height,
                seconds:  p_seconds,
                created:  chrono::Local::now().naive_utc() + Duration::hours(3),
                template: p_template,
            };
            let new = diesel::insert_into(schema::cookie_stats::table)
                .values(&_h)
                .get_result::<CookieStat>(&_connection)?;
            return Ok(new);
        }
        return Err(Error::BadRequest("Permission Denied".to_string()));
    }
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="cookie_stats"]
pub struct NewCookieStat {
    pub user_id:  i32,
    pub page:     i16,
    pub link:     String,
    pub title:    String,
    pub title_en: String,
    pub height:   f64,
    pub seconds:  i32,
    pub created:  chrono::NaiveDateTime,
    pub template: String,
}


////////////////////
// Шифры посещаемых страниц
// 1 - главная
// 2 - о сайте
// 3 - контакты
// 4 - команда
// 5 - сотрудничество
// 6 - вход
// 7 - регитрация
// 8 - выход
// 9 - вопросы ответы
// 10 - инфо

// 11 - профиль
// 12 - заказы
// 13 - история
// 14 - статистика

// 21 - общий поиск
// 22 - поиск статей блога
// 23 - поиск услуг
// 24 - поиск товаров
// 25 - поиск статей обучающих
// 26 - поиск работ

// 31 - теги
// 32 - тег
// 33 - тег - статьи блога
// 34 - тег - услуги
// 35 - тег - товары
// 36 - тег - статьи обучающие
// 37 - тег - работы

// 41 - категории блога
// 51 - категории опций
// 53 - технологии опций
// 61 - категории услуг
// 71 - категории товаров
// 81 - категории обучения
// 91 - категории работ

#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct StatPage {
    pub id:      i32,
    pub types:   i16,
    pub view:    i32,
    pub height:  f64,
    pub seconds: i32,
}
impl StatPage {
    pub fn get_or_create(types: i16) -> StatPage {
        let _connection = establish_connection();
        let _stats = schema::stat_pages::table
            .filter(schema::stat_pages::types.eq(types))
            .first::<StatPage>(&_connection);
        if _stats.is_ok() {
            return _stats.expect("E");
        }
        else { 
            let form = NewStatPage {
                types:   types,
                view:    0,
                height:  0.0,
                seconds: 0,
            };
            let _stat = diesel::insert_into(schema::stat_pages::table)
                .values(&form)
                .get_result::<StatPage>(&_connection)
                .expect("Error.");
            return _stat;
        }
    }
}

////////////////////
#[derive(Debug, Deserialize, Insertable)]
#[table_name="stat_pages"]
pub struct NewStatPage {
    pub types:   i16,
    pub view:    i32,
    pub height:  f64,
    pub seconds: i32,
}
