use crate::schema;
use crate::diesel::{
    Queryable,
    Insertable,
    QueryDsl,
    RunQueryDsl,
    ExpressionMethods,
    NullableExpressionMethods,
    PgTextExpressionMethods,
};
use serde::{Serialize,Deserialize};
use crate::models::{
    Tag, TechCategories, Serve,
    SmallTag, SmallFile, User,
};

use crate::schema::{
    categories,
    items,
    category,
    item_comments,
};
use crate::utils::{
    establish_connection,
    get_linguage_storage,
    CategoriesForm
};
use crate::errors::Error;


///////////
// types:
// 1. блог
// 2. услуга
// 3. товар
// 4. wiki
// 5. работа 
// 6. помощь
// 7. заказ

#[derive(Serialize, Queryable)]
pub struct CatDetail { 
    pub name:    String,
    pub slug:    String,
    pub count:   i16,
    pub id:      i32,
    pub image:   Option<String>,
    pub view:    i32,
    pub height:  f64,
    pub seconds: i32,
}
impl CatDetail {
    pub fn get_image(&self) -> String {
        if self.image.is_some() {
            return self.image.as_deref().unwrap().to_string();
        }
        else {
            return "/static/images/img.jpg".to_string();
        }
    }
}

#[derive(Serialize, Queryable)]
pub struct Cat {
    pub name:    String,
    pub slug:    String,
    pub count:   i16,
    pub id:      i32,
    pub image:   Option<String>,
}
impl Cat {
    pub fn get_image(&self) -> String {
        if self.image.is_some() {
            return self.image.as_deref().unwrap().to_string();
        }
        else {
            return "/static/images/img.jpg".to_string();
        }
    }
} 

#[derive(Serialize, Queryable)]
pub struct SmallCat {
    pub name:  String,
    pub slug:  String,
    pub count: i16,
}
#[derive(Serialize, Queryable)]
pub struct Blog {
    pub id:          i32,
    pub slug:        String,
    pub image:       Option<String>,
    pub is_active:   bool,
    pub title:       String,
    pub created:     chrono::NaiveDateTime,
    pub description: Option<String>,
}
impl Blog {
    pub fn get_image(&self) -> String {
        if self.image.is_some() {
            return self.image.as_deref().unwrap().to_string();
        }
        else {
            return "/static/images/img.jpg".to_string();
        }
    }
    pub fn get_tags(&self, l: u8) -> Vec<SmallTag> {
        let _connection = establish_connection();
        let _tag_items = schema::tags_items::table
            .filter(schema::tags_items::item_id.eq(self.id))
            .filter(schema::tags_items::types.eq(1))
            .select(schema::tags_items::tag_id)
            .load::<i32>(&_connection)
            .expect("E");
        return Tag::get_tags_with_ids(_tag_items, l);
    }
}

#[derive(Serialize, Queryable)]
pub struct Service {
    pub id:          i32,
    pub slug:        String,
    pub image:       Option<String>,
    pub is_active:   bool,
    pub title:       String,
    pub description: Option<String>,
}
impl Service {
    pub fn get_image(&self) -> String {
        if self.image.is_some() {
            return self.image.as_deref().unwrap().to_string();
        }
        else {
            return "/static/images/img.jpg".to_string();
        }
    }
    pub fn get_tags(&self, l: u8) -> Vec<SmallTag> {
        let _connection = establish_connection();

        let _tag_items = schema::tags_items::table
            .filter(schema::tags_items::item_id.eq(self.id))
            .filter(schema::tags_items::types.eq(2))
            .select(schema::tags_items::tag_id)
            .load::<i32>(&_connection)
            .expect("E");
        return Tag::get_tags_with_ids(_tag_items, l);
    }
}

#[derive(Serialize, Queryable)]
pub struct Store {
    pub id:          i32,
    pub slug:        String,
    pub image:       Option<String>,
    pub is_active:   bool,
    pub title:       String,
    pub description: Option<String>,
    pub price:       i32,
    pub price_acc:   Option<i32>,
}
impl Store {
    pub fn get_image(&self) -> String {
        if self.image.is_some() {
            return self.image.as_deref().unwrap().to_string();
        }
        else {
            return "/static/images/img.jpg".to_string();
        }
    }
    pub fn get_tags(&self, l: u8) -> Vec<SmallTag> {
        let _connection = establish_connection();

        let _tag_items = schema::tags_items::table
            .filter(schema::tags_items::item_id.eq(self.id))
            .filter(schema::tags_items::types.eq(3))
            .select(schema::tags_items::tag_id)
            .load::<i32>(&_connection)
            .expect("E");
        return Tag::get_tags_with_ids(_tag_items, l);
    }
}

#[derive(Serialize, Queryable)]
pub struct Wiki {
    pub id:          i32,
    pub slug:        String,
    pub image:       Option<String>,
    pub is_active:   bool,
    pub title:       String,
    pub description: Option<String>,
    pub created:     chrono::NaiveDateTime,
}
impl Wiki {
    pub fn get_image(&self) -> String {
        if self.image.is_some() {
            return self.image.as_deref().unwrap().to_string();
        }
        else {
            return "/static/images/img.jpg".to_string();
        }
    }
    pub fn get_tags(&self, l: u8) -> Vec<SmallTag> {
        let _connection = establish_connection();

        let _tag_items = schema::tags_items::table
            .filter(schema::tags_items::item_id.eq(self.id))
            .filter(schema::tags_items::types.eq(4))
            .select(schema::tags_items::tag_id)
            .load::<i32>(&_connection)
            .expect("E");
        return Tag::get_tags_with_ids(_tag_items, l);
    }
}

#[derive(Serialize, Queryable)]
pub struct Work {
    pub id:          i32,
    pub slug:        String,
    pub image:       Option<String>,
    pub is_active:   bool,
    pub title:       String,
    pub description: Option<String>,
}
impl Work {
    pub fn get_image(&self) -> String {
        if self.image.is_some() {
            return self.image.as_deref().unwrap().to_string();
        }
        else {
            return "/static/images/img.jpg".to_string();
        }
    }
    pub fn get_tags(&self, l: u8) -> Vec<SmallTag> {
        let _connection = establish_connection();

        let _tag_items = schema::tags_items::table
            .filter(schema::tags_items::item_id.eq(self.id))
            .filter(schema::tags_items::types.eq(5))
            .select(schema::tags_items::tag_id)
            .load::<i32>(&_connection)
            .expect("E");
        return Tag::get_tags_with_ids(_tag_items, l);
    }
}

#[derive(Serialize, Queryable)]
pub struct Help {
    pub id:        i32,
    pub is_active: bool,
    pub title:     String,
    pub content:   Option<String>,
}
impl Help {
    pub fn get_category(&self) -> SmallCat {
        use crate::schema::{
            category::dsl::category,
            categories::dsl::categories,
        };
        let _connection = establish_connection();
        let _id = category
            .filter(schema::category::item_id.eq(self.id))
            .filter(schema::category::types.eq(6))
            .select(schema::category::category_id)
            .first::<i32>(&_connection)
            .expect("E");

        let _category = categories
            .filter(schema::categories::id.eq(_id))
            .select((
                schema::categories::name,
                schema::categories::slug,
                schema::categories::count
            ))
            .first::<SmallCat>(&_connection)
            .expect("E");
        return _category;
    }
    pub fn get_tags(&self, l: u8) -> Vec<SmallTag> {
        let _connection = establish_connection();

        let _tag_items = schema::tags_items::table
            .filter(schema::tags_items::item_id.eq(self.id))
            .filter(schema::tags_items::types.eq(6))
            .select(schema::tags_items::tag_id)
            .load::<i32>(&_connection)
            .expect("E");
        return Tag::get_tags_with_ids(_tag_items, l);
    }
}

#[derive(Serialize, Queryable)]
pub struct FeaturedItem {
    pub slug:  String,
    pub title: String,
}

#[derive(Debug, Serialize, Queryable, Identifiable)]
#[table_name="categories"]
pub struct Categories {
    pub id:             i32,
    pub name:           String,
    pub name_en:        String,
    pub description:    Option<String>,
    pub description_en: Option<String>,
    pub position:       i16,
    pub image:          Option<String>,
    pub count:          i16,
    pub view:           i32,
    pub height:         f64,
    pub seconds:        i32,
    pub types:          i16,
    pub slug:           String,
}

impl Categories {
    pub fn delete(user: User, item_id: i32) -> i16 {
        let _connection = establish_connection();
        let _item = Categories::get_with_id(item_id);
        if user.perm < 60 {
            return 0;
        }
        if _item.image.is_some() {
            std::fs::remove_file(_item.image.unwrap()).expect("E");
        }
        diesel::delete(schema::categories::table.filter(schema::categories::id.eq(item_id)))
            .execute(&_connection)
            .expect("E");
        return 1;
    }
    pub fn get_all() -> Vec<Categories> {
        let _connection = establish_connection();
        return schema::categories::table 
            .load::<Categories>(&_connection)
            .expect("E");
    }
    pub fn get(slug: &String, types: i16) -> Categories {
        let _connection = establish_connection();
        return schema::categories::table 
            .filter(schema::categories::slug.eq(slug))
            .filter(schema::categories::types.eq(types))
            .first::<Categories>(&_connection)
            .expect("E.");
    }
    pub fn get_with_types(types: i16) -> Vec<Categories> {
        let _connection = establish_connection();
        return schema::categories::table 
            .filter(schema::categories::types.eq(types))
            .load::<Categories>(&_connection)
            .expect("E.");
    }
    pub fn get_with_id(id: i32) -> Categories {
        let _connection = establish_connection();
        return schema::categories::table 
            .filter(schema::categories::id.eq(id)) 
            .first::<Categories>(&_connection)
            .expect("E.");
    }
    pub fn update_category_with_id(user: User, id: i32, form: CategoriesForm) -> i16 {
        let _connection = establish_connection();
        let l = get_linguage_storage();
        let cat = schema::categories::table
            .filter(schema::categories::id.eq(id))
            .first::<Categories>(&_connection)
            .expect("E.");
        if user.perm < 60 {
            return 0;
        }
        if l == 1 { 
            diesel::update(&cat)
                .set((
                    schema::categories::name.eq(&form.name),
                    schema::categories::description.eq(&form.description),
                    schema::categories::position.eq(form.position),
                    schema::categories::image.eq(&form.image),
                    schema::categories::slug.eq(&form.slug),
                ))
                .execute(&_connection)
                .expect("E");
        }
        else if l == 2 {
            diesel::update(&cat)
                .set((
                    schema::categories::name_en.eq(&form.name),
                    schema::categories::description_en.eq(&form.description),
                    schema::categories::position.eq(form.position),
                    schema::categories::image.eq(&form.image),
                    schema::categories::slug.eq(&form.slug),
                ))
                .execute(&_connection)
                .expect("E");
        }
        return 1;
    }
    pub fn create(form: CategoriesForm) -> i16 {
        let _connection = establish_connection();
        let l = get_linguage_storage();
        if l == 1 {
            let new_cat = NewCategories {
                name:           form.name.clone(),
                name_en:        "".to_string(),
                description:    Some(form.description.clone()),
                description_en: Some("".to_string()),
                position:       form.position,
                image:          Some(form.image.clone()),
                count:          0,
                view:           0,
                height:         0.0,
                seconds:        0,
                types:          form.types,
                slug:           form.slug,
            };
            diesel::insert_into(schema::categories::table)
                .values(&new_cat)
                .execute(&_connection)
                .expect("E.");
        }
        else if l == 2 {
            let new_cat = NewCategories {
                name:           "".to_string(),
                name_en:        form.name.clone(),
                description:    Some("".to_string()),
                description_en: Some(form.description.clone()),
                position:       form.position,
                image:          Some(form.image.clone()),
                count:          0,
                view:           0,
                height:         0.0,
                seconds:        0,
                types:          form.types,
                slug:           form.slug,
            };
            diesel::insert_into(schema::categories::table)
                .values(&new_cat)
                .execute(&_connection)
                .expect("E.");
        }
        return 1;
    }
    pub fn get_tags(types: i16, l: u8) -> Vec<SmallTag> {
        let _connection = establish_connection();

        let _tag_items = schema::tags_items::table
            .filter(schema::tags_items::types.eq(types))
            .select(schema::tags_items::tag_id)
            .load::<i32>(&_connection)
            .expect("E."); 
        return Tag::get_tags_with_ids(_tag_items, l);
    }
    pub fn get_featured_items ( 
        &self,
        item_id:    i32,
        item_types: i16,
        l:          u8,
    ) -> (Option<FeaturedItem>, Option<FeaturedItem>) {
        use crate::schema::items::dsl::items;

        let _connection = establish_connection();

        let mut prev: Option<FeaturedItem> = None;
        let mut next: Option<FeaturedItem> = None;

        let _category_items = schema::category::table
            .filter(schema::category::category_id.eq(self.id))
            .filter(schema::category::types.eq(item_types))
            .select(schema::category::item_id)
            .load::<i32>(&_connection)
            .expect("E");
        let _category_items_len = _category_items.len();
        for (i, item) in _category_items.iter().enumerate().rev() {
            if item == &item_id {
                if (i + 1) != _category_items_len {
                    let _next = Some(&_category_items[i + 1]);
                    if l == 1 {
                        next = Some(items
                            .filter(schema::items::id.eq(_next.unwrap()))
                            .filter(schema::items::types.eq(item_types))
                            .filter(schema::items::is_active.eq(true))
                            .select((
                                schema::items::slug,
                                schema::items::title,
                            ))
                            .first::<FeaturedItem>(&_connection)
                            .expect("E."));
                    }
                    else if l == 2 {
                        next = Some(items
                            .filter(schema::items::id.eq(_next.unwrap()))
                            .filter(schema::items::types.eq(item_types))
                            .filter(schema::items::is_active.eq(true))
                            .select((
                                schema::items::slug,
                                schema::items::title_en,
                            ))
                            .first::<FeaturedItem>(&_connection)
                            .expect("E."));
                    }
                };
                if i != 0 {
                    let _prev = Some(&_category_items[i - 1]);
                    if l == 1 {
                        prev = Some(items
                            .filter(schema::items::id.eq(_prev.unwrap()))
                            .filter(schema::items::types.eq(item_types))
                            .filter(schema::items::is_active.eq(true))
                            .select((
                                schema::items::slug,
                                schema::items::title,
                            ))
                            .first::<FeaturedItem>(&_connection)
                            .expect("E."));
                    }
                    else if l == 2 {
                        prev = Some(items
                            .filter(schema::items::id.eq(_prev.unwrap()))
                            .filter(schema::items::types.eq(item_types))
                            .filter(schema::items::is_active.eq(true))
                            .select((
                                schema::items::slug,
                                schema::items::title_en,
                            ))
                            .first::<FeaturedItem>(&_connection)
                            .expect("E."));
                    }
                };
                break;
            }
        };
        return (prev, next);
    }
    pub fn get_type(&self) -> String {
        return match self.types {
            1 => "блог".to_string(),
            2 => "услуга".to_string(),
            3 => "товар".to_string(),
            4 => "wiki".to_string(),
            5 => "работа".to_string(),
            6 => "помощь".to_string(),
            _ => "Непонятно".to_string(),
        };
    }

    pub fn get_blogs_list (
        cat_id:   i32,
        page:     i32,
        limit:    i32,
        is_admin: bool,
        l:        u8,
    ) -> Result<(Vec<Blog>, i32), Error> {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Blog>;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = Categories::get_blogs(cat_id, limit.into(), step.into(), is_admin, l)?;
        }
        else {
            have_next = limit + 1;
            object_list = Categories::get_blogs(cat_id, limit.into(), 0, is_admin, l)?;
        }
        if Categories::get_blogs(cat_id, 1, have_next.into(), is_admin, l)?.len() > 0 {
            next_page_number = page + 1;
        }
        let _tuple = (object_list, next_page_number);
        Ok(_tuple)
    }
    pub fn get_blogs (
        cat_id:   i32,
        limit:    i64,
        offset:   i64,
        is_admin: bool,
        l:        u8,
    ) -> Result<Vec<Blog>, Error> {
        let _connection = establish_connection();
        let ids = schema::category::table
            .filter(schema::category::category_id.eq(cat_id))
            .filter(schema::category::types.eq(1))
            .select(schema::category::item_id)
            .load::<i32>(&_connection)
            .expect("E");
        if is_admin {
            if l == 1 {
                return Ok(schema::items::table
                    .filter(schema::items::id.eq_any(ids))
                    .order(schema::items::created.desc())
                    .limit(limit)
                    .offset(offset)
                    .select((
                        schema::items::id,
                        schema::items::slug,
                        schema::items::image.nullable(),
                        schema::items::is_active,
                        schema::items::title,
                        schema::items::created,
                        schema::items::description.nullable(),
                    ))
                    .load::<Blog>(&_connection)
                    .expect("E."));
            }
            else if l == 2 {
                return Ok(schema::items::table
                    .filter(schema::items::id.eq_any(ids))
                    .order(schema::items::created.desc())
                    .limit(limit)
                    .offset(offset)
                    .select((
                        schema::items::id,
                        schema::items::slug,
                        schema::items::image.nullable(),
                        schema::items::is_active,
                        schema::items::title_en,
                        schema::items::created,
                        schema::items::description_en.nullable(),
                    ))
                    .load::<Blog>(&_connection)
                    .expect("E."));
            }
        } else {
            if l == 2 {
                return Ok(schema::items::table
                    .filter(schema::items::id.eq_any(ids))
                    .filter(schema::items::is_active.eq(true))
                    .order(schema::items::created.desc())
                    .limit(limit)
                    .offset(offset)
                    .select((
                        schema::items::id,
                        schema::items::slug,
                        schema::items::image.nullable(),
                        schema::items::is_active,
                        schema::items::title,
                        schema::items::created,
                        schema::items::description.nullable(),
                    ))
                    .load::<Blog>(&_connection)
                    .expect("E."));
            }
            else if l == 2 {
                return Ok(schema::items::table
                    .filter(schema::items::id.eq_any(ids))
                    .filter(schema::items::is_active.eq(true))
                    .order(schema::items::created.desc())
                    .limit(limit)
                    .offset(offset)
                    .select((
                        schema::items::id,
                        schema::items::slug,
                        schema::items::image.nullable(),
                        schema::items::is_active,
                        schema::items::title_en,
                        schema::items::created,
                        schema::items::description_en.nullable(),
                    ))
                    .load::<Blog>(&_connection)
                    .expect("E."));
            }
        }
        return Ok(Vec::new());
    }
    pub fn get_services_list (
        cat_id:   i32,
        page:     i32,
        limit:    i32,
        is_admin: bool,
        l:        u8,
    ) -> Result<(Vec<Service>, i32), Error> {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Service>;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = Categories::get_services(cat_id, limit.into(), step.into(), is_admin, l)?;
        }
        else {
            have_next = limit + 1;
            object_list = Categories::get_services(cat_id, limit.into(), 0, is_admin, l)?;
        }
        if Categories::get_services(cat_id, 1, have_next.into(), is_admin, l)?.len() > 0 {
            next_page_number = page + 1;
        }

        return Ok((object_list, next_page_number));
    }
    pub fn get_services (
        cat_id:   i32,
        limit:    i64,
        offset:   i64,
        is_admin: bool,
        l:        u8,
    ) -> Result<Vec<Service>, Error> {
        use crate::schema::{
            items::dsl::items,
            category::dsl::category,
        };

        let _connection = establish_connection();
        let ids = category
            .filter(schema::category::category_id.eq(cat_id))
            .filter(schema::category::types.eq(2))
            .select(schema::category::item_id)
            .load::<i32>(&_connection)
            .expect("E");
        if is_admin {
            if l == 1 {
                return Ok(items
                    .filter(schema::items::id.eq_any(ids))
                    .order(schema::items::created.desc())
                    .limit(limit)
                    .offset(offset)
                    .select((
                        schema::items::id,
                        schema::items::slug,
                        schema::items::image.nullable(),
                        schema::items::is_active,
                        schema::items::title,
                        schema::items::description.nullable(),
                    ))
                    .load::<Service>(&_connection)
                    .expect("E."));
            }
            else if l == 2 {
                return Ok(items
                    .filter(schema::items::id.eq_any(ids))
                    .order(schema::items::created.desc())
                    .limit(limit)
                    .offset(offset)
                    .select((
                        schema::items::id,
                        schema::items::slug,
                        schema::items::image.nullable(),
                        schema::items::is_active,
                        schema::items::title_en,
                        schema::items::description_en.nullable(),
                    ))
                    .load::<Service>(&_connection)
                    .expect("E."));
            }
        } else {
            if l == 1 {
                return Ok(items
                    .filter(schema::items::id.eq_any(ids))
                    .filter(schema::items::is_active.eq(true))
                    .order(schema::items::created.desc())
                    .limit(limit)
                    .offset(offset)
                    .select((
                        schema::items::id,
                        schema::items::slug,
                        schema::items::image.nullable(),
                        schema::items::is_active,
                        schema::items::title,
                        schema::items::description.nullable(),
                    ))
                    .load::<Service>(&_connection)
                    .expect("E."));
            }
            else if l == 2 {
                return Ok(items
                    .filter(schema::items::id.eq_any(ids))
                    .filter(schema::items::is_active.eq(true))
                    .order(schema::items::created.desc())
                    .limit(limit)
                    .offset(offset)
                    .select((
                        schema::items::id,
                        schema::items::slug,
                        schema::items::image.nullable(),
                        schema::items::is_active,
                        schema::items::title_en,
                        schema::items::description_en.nullable(),
                    ))
                    .load::<Service>(&_connection)
                    .expect("E."));
            }
        }
        return Ok(Vec::new());
    }

    pub fn get_stores_list (
        cat_id:   i32,
        page:     i32,
        limit:    i32,
        is_admin: bool,
        l:        u8,
    ) -> Result<(Vec<Store>, i32), Error> {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Store>;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = Categories::get_stores(cat_id, limit.into(), step.into(), is_admin, l)?;
        }
        else {
            have_next = limit + 1;
            object_list = Categories::get_stores(cat_id, limit.into(), 0, is_admin, l)?;
        }
        if Categories::get_stores(cat_id, 1, have_next.into(), is_admin, l)?.len() > 0 {
            next_page_number = page + 1;
        }

        return Ok((object_list, next_page_number));
    }
    pub fn get_stores (
        cat_id:   i32,
        limit:    i64,
        offset:   i64,
        is_admin: bool,
        l:        u8,
    ) -> Result<Vec<Store>, Error> {
        use crate::schema::{
            items::dsl::items,
            category::dsl::category,
        };

        let _connection = establish_connection();
        let ids = category
            .filter(schema::category::category_id.eq(cat_id))
            .filter(schema::category::types.eq(3))
            .select(schema::category::item_id)
            .load::<i32>(&_connection)
            .expect("E");
        if is_admin {
            if l == 1 {
                return Ok(items
                    .filter(schema::items::id.eq_any(ids))
                    .order(schema::items::created.desc())
                    .limit(limit)
                    .offset(offset)
                    .select((
                        schema::items::id,
                        schema::items::slug,
                        schema::items::image.nullable(),
                        schema::items::is_active,
                        schema::items::title,
                        schema::items::description.nullable(),
                        schema::items::price,
                        schema::items::price_acc.nullable(),
                    ))
                    .load::<Store>(&_connection)
                    .expect("E."));
            }
            else if l == 2 {
                return Ok(items
                    .filter(schema::items::id.eq_any(ids))
                    .order(schema::items::created.desc())
                    .limit(limit)
                    .offset(offset)
                    .select((
                        schema::items::id,
                        schema::items::slug,
                        schema::items::image.nullable(),
                        schema::items::is_active,
                        schema::items::title_en,
                        schema::items::description_en.nullable(),
                        schema::items::price,
                        schema::items::price_acc.nullable(),
                    ))
                    .load::<Store>(&_connection)
                    .expect("E."));
            }
        } else {
            if l == 1 {
                return Ok(items
                    .filter(schema::items::id.eq_any(ids))
                    .filter(schema::items::is_active.eq(true))
                    .order(schema::items::created.desc())
                    .limit(limit)
                    .offset(offset)
                    .select((
                        schema::items::id,
                        schema::items::slug,
                        schema::items::image.nullable(),
                        schema::items::is_active,
                        schema::items::title,
                        schema::items::description.nullable(),
                        schema::items::price,
                        schema::items::price_acc.nullable(),
                    ))
                    .load::<Store>(&_connection)
                    .expect("E."));
            }
            else if l == 2 {
                return Ok(items
                    .filter(schema::items::id.eq_any(ids))
                    .filter(schema::items::is_active.eq(true))
                    .order(schema::items::created.desc())
                    .limit(limit)
                    .offset(offset)
                    .select((
                        schema::items::id,
                        schema::items::slug,
                        schema::items::image.nullable(),
                        schema::items::is_active,
                        schema::items::title_en,
                        schema::items::description_en.nullable(),
                        schema::items::price,
                        schema::items::price_acc.nullable(),
                    ))
                    .load::<Store>(&_connection)
                    .expect("E."));
            }
        }
        return Ok(Vec::new());
    }

    pub fn get_wikis_list (
        cat_id:   i32,
        page:     i32,
        limit:    i32,
        is_admin: bool,
        l:        u8,
    ) -> Result<(Vec<Wiki>, i32), Error> {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Wiki>;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = Categories::get_wikis(cat_id, limit.into(), step.into(), is_admin, l)?;
        }
        else {
            have_next = limit + 1;
            object_list = Categories::get_wikis(cat_id, limit.into(), 0, is_admin, l)?;
        }
        if Categories::get_wikis(cat_id, 1, have_next.into(), is_admin, l)?.len() > 0 {
            next_page_number = page + 1;
        }

        return Ok((object_list, next_page_number));
    }
    pub fn get_wikis (
        cat_id:   i32,
        limit:    i64,
        offset:   i64,
        is_admin: bool,
        l:        u8,
    ) -> Result<Vec<Wiki>, Error> {
        use crate::schema::{
            items::dsl::items,
            category::dsl::category,
        };

        let _connection = establish_connection();
        let ids = category
            .filter(schema::category::category_id.eq(cat_id))
            .filter(schema::category::types.eq(4))
            .select(schema::category::item_id)
            .load::<i32>(&_connection)
            .expect("E");
        if is_admin {
            if l == 1 {
                return Ok(items
                    .filter(schema::items::id.eq_any(ids))
                    .order(schema::items::created.desc())
                    .limit(limit)
                    .offset(offset)
                    .select((
                        schema::items::id,
                        schema::items::slug,
                        schema::items::image.nullable(),
                        schema::items::is_active,
                        schema::items::title,
                        schema::items::description.nullable(),
                        schema::items::created
                    ))
                    .load::<Wiki>(&_connection)
                    .expect("E."));
            }
            else if l == 2 {
                return Ok(items
                    .filter(schema::items::id.eq_any(ids))
                    .order(schema::items::created.desc())
                    .limit(limit)
                    .offset(offset)
                    .select((
                        schema::items::id,
                        schema::items::slug,
                        schema::items::image.nullable(),
                        schema::items::is_active,
                        schema::items::title_en,
                        schema::items::description_en.nullable(),
                        schema::items::created
                    ))
                    .load::<Wiki>(&_connection)
                    .expect("E."));
            }
        } else {
            if l == 1 {
                return Ok(items
                    .filter(schema::items::id.eq_any(ids))
                    .filter(schema::items::is_active.eq(true))
                    .order(schema::items::created.desc())
                    .limit(limit)
                    .offset(offset)
                    .select((
                        schema::items::id,
                        schema::items::slug,
                        schema::items::image.nullable(),
                        schema::items::is_active,
                        schema::items::title,
                        schema::items::description.nullable(),
                        schema::items::created
                    ))
                    .load::<Wiki>(&_connection)
                    .expect("E."));
            }
            else if l == 2 {
                return Ok(items
                    .filter(schema::items::id.eq_any(ids))
                    .filter(schema::items::is_active.eq(true))
                    .order(schema::items::created.desc())
                    .limit(limit)
                    .offset(offset)
                    .select((
                        schema::items::id,
                        schema::items::slug,
                        schema::items::image.nullable(),
                        schema::items::is_active,
                        schema::items::title_en,
                        schema::items::description_en.nullable(),
                        schema::items::created
                    ))
                    .load::<Wiki>(&_connection)
                    .expect("E."));
            }
        }
        return Ok(Vec::new());
    }

    pub fn get_works_list (
        cat_id:   i32,
        page:     i32,
        limit:    i32,
        is_admin: bool,
        l:        u8,
    ) -> Result<(Vec<Work>, i32), Error> {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Work>;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = Categories::get_works(cat_id, limit.into(), step.into(), is_admin, l)?;
        }
        else {
            have_next = limit + 1;
            object_list = Categories::get_works(cat_id, limit.into(), 0, is_admin, l)?;
        }
        if Categories::get_works(cat_id, 1, have_next.into(), is_admin, l)?.len() > 0 {
            next_page_number = page + 1;
        }

        return Ok((object_list, next_page_number));
    }
    pub fn get_works (
        cat_id:   i32,
        limit:    i64,
        offset:   i64,
        is_admin: bool,
        l:        u8,
    ) -> Result<Vec<Work>, Error> {
        use crate::schema::{
            items::dsl::items,
            category::dsl::category,
        };

        let _connection = establish_connection();
        let ids = category
            .filter(schema::category::category_id.eq(cat_id))
            .filter(schema::category::types.eq(5))
            .select(schema::category::item_id)
            .load::<i32>(&_connection)
            .expect("E");
        if is_admin {
            if l == 1 {
                return Ok(items
                    .filter(schema::items::id.eq_any(ids))
                    .order(schema::items::created.desc())
                    .limit(limit)
                    .offset(offset)
                    .select((
                        schema::items::id,
                        schema::items::slug,
                        schema::items::image.nullable(),
                        schema::items::is_active,
                        schema::items::title,
                        schema::items::description.nullable(),
                    ))
                    .load::<Work>(&_connection)
                    .expect("E."));
            }
            else if l == 2 {
                return Ok(items
                    .filter(schema::items::id.eq_any(ids))
                    .order(schema::items::created.desc())
                    .limit(limit)
                    .offset(offset)
                    .select((
                        schema::items::id,
                        schema::items::slug,
                        schema::items::image.nullable(),
                        schema::items::is_active,
                        schema::items::title_en,
                        schema::items::description_en.nullable(),
                    ))
                    .load::<Work>(&_connection)
                    .expect("E."));
            }
        } else {
            if l == 1 {
                return Ok(items
                    .filter(schema::items::id.eq_any(ids))
                    .filter(schema::items::is_active.eq(true))
                    .order(schema::items::created.desc())
                    .limit(limit)
                    .offset(offset)
                    .select((
                        schema::items::id,
                        schema::items::slug,
                        schema::items::image.nullable(),
                        schema::items::is_active,
                        schema::items::title,
                        schema::items::description.nullable(),
                    ))
                    .load::<Work>(&_connection)
                    .expect("E."));
            }
            else if l == 2 {
                return Ok(items
                    .filter(schema::items::id.eq_any(ids))
                    .filter(schema::items::is_active.eq(true))
                    .order(schema::items::created.desc())
                    .limit(limit)
                    .offset(offset)
                    .select((
                        schema::items::id,
                        schema::items::slug,
                        schema::items::image.nullable(),
                        schema::items::is_active,
                        schema::items::title_en,
                        schema::items::description_en.nullable(),
                    ))
                    .load::<Work>(&_connection)
                    .expect("E."));
            }
        }
        return Ok(Vec::new());
    }

    pub fn get_helps_list (
        cat_id:   i32,
        page:     i32,
        limit:    i32,
        is_admin: bool,
        l:        u8,
    ) -> Result<(Vec<Help>, i32), Error> {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<Help>;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = Categories::get_helps(cat_id, limit.into(), step.into(), is_admin, l)?;
        }
        else {
            have_next = limit + 1;
            object_list = Categories::get_helps(cat_id, limit.into(), 0, is_admin, l)?;
        }
        if Categories::get_helps(cat_id, 1, have_next.into(), is_admin, l)?.len() > 0 {
            next_page_number = page + 1;
        }

        return Ok((object_list, next_page_number));
    }
    pub fn get_helps (  
        cat_id:   i32,
        limit:    i64,
        offset:   i64,
        is_admin: bool,
        l:        u8,
    ) -> Result<Vec<Help>, Error> {
        use crate::schema::{
            items::dsl::items,
            category::dsl::category,
        };

        let _connection = establish_connection();
        let _items: Vec<Help>;
        let ids = category
            .filter(schema::category::category_id.eq(cat_id))
            .filter(schema::category::types.eq(6))
            .select(schema::category::item_id)
            .load::<i32>(&_connection)
            .expect("E");
        if is_admin {
            if l == 1 {
                return Ok(items
                    .filter(schema::items::id.eq_any(ids))
                    .order(schema::items::position.asc())
                    .limit(limit)
                    .offset(offset)
                    .select((
                        schema::items::id,
                        schema::items::is_active,
                        schema::items::title,
                        schema::items::content,
                    ))
                    .load::<Help>(&_connection)
                    .expect("E."));
            }
            else if l == 2 {
                return Ok(items
                    .filter(schema::items::id.eq_any(ids))
                    .order(schema::items::position.asc())
                    .limit(limit)
                    .offset(offset)
                    .select((
                        schema::items::id,
                        schema::items::is_active,
                        schema::items::title_en,
                        schema::items::content_en,
                    ))
                    .load::<Help>(&_connection)
                    .expect("E."));
            }
        } else {
            if l == 1 {
                return Ok(items
                    .filter(schema::items::id.eq_any(ids))
                    .filter(schema::items::is_active.eq(true))
                    .order(schema::items::position.asc())
                    .limit(limit)
                    .offset(offset)
                    .select((
                        schema::items::id,
                        schema::items::is_active,
                        schema::items::title,
                        schema::items::content,
                    ))
                    .load::<Help>(&_connection)
                    .expect("E."));
            }
            else if l == 2 {
                return Ok(items
                    .filter(schema::items::id.eq_any(ids))
                    .filter(schema::items::is_active.eq(true))
                    .order(schema::items::position.asc())
                    .limit(limit)
                    .offset(offset)
                    .select((
                        schema::items::id,
                        schema::items::is_active,
                        schema::items::title_en,
                        schema::items::content_en,
                    ))
                    .load::<Help>(&_connection)
                    .expect("E."));
            }
        }
        return Ok(Vec::new());
    }

    pub fn get_image(&self) -> String {
        if self.image.is_some() {
            return self.image.as_deref().unwrap().to_string();
        }
        else {
            return "/static/images/img.jpg".to_string();
        }
    }
    pub fn get_detail(slug: String, types: i16, l: u8) -> CatDetail {
        let _connection = establish_connection();
        if l == 1 {
            return schema::categories::table
                .filter(schema::categories::slug.eq(&slug))
                .filter(schema::categories::types.eq(types))
                .select((
                    schema::categories::name,
                    schema::categories::slug,
                    schema::categories::count,
                    schema::categories::id,
                    schema::categories::image,
                    schema::categories::view,
                    schema::categories::height,
                    schema::categories::seconds,
                ))
                .first::<CatDetail>(&_connection)
                .expect("E");
        }
        else if l == 2 {
            return schema::categories::table
                .filter(schema::categories::slug.eq(&slug))
                .filter(schema::categories::types.eq(types))
                .select((
                    schema::categories::name_en,
                    schema::categories::slug,
                    schema::categories::count,
                    schema::categories::id,
                    schema::categories::image,
                    schema::categories::view,
                    schema::categories::height,
                    schema::categories::seconds,
                ))
                .first::<CatDetail>(&_connection)
                .expect("E");
        }
        return CatDetail { 
            name:    "".to_string(),
            slug:    "".to_string(),
            count:   0,
            id:      0,
            image:   None,
            view:    0,
            height:  0.0,
            seconds: 0,
        }
    }
    pub fn get_categories_for_types(types: i16, l: u8) -> Vec<Cat> {
        let _connection = establish_connection();
        if l == 1 {
            return schema::categories::table
                .filter(schema::categories::types.eq(types))
                .select((
                    schema::categories::name,
                    schema::categories::slug,
                    schema::categories::count,
                    schema::categories::id,
                    schema::categories::image
                ))
                .load::<Cat>(&_connection)
                .expect("E");
        }
        else if l == 2 {
            return schema::categories::table
                .filter(schema::categories::types.eq(types))
                .select((
                    schema::categories::name_en,
                    schema::categories::slug,
                    schema::categories::count,
                    schema::categories::id,
                    schema::categories::image
                ))
                .load::<Cat>(&_connection)
                .expect("E");
        }
        return Vec::new();
    }
}

#[derive(Insertable)]
#[table_name="categories"]
pub struct NewCategories {
    pub name:           String,
    pub name_en:        String,
    pub description:    Option<String>,
    pub description_en: Option<String>,
    pub position:       i16,
    pub image:          Option<String>,
    pub count:          i16,
    pub view:           i32,
    pub height:         f64,
    pub seconds:        i32,
    pub types:          i16,
    pub slug:           String,
}

#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="categories"]
pub struct EditCategories {
    pub name:           String,
    pub name_en:        String,
    pub description:    Option<String>,
    pub description_en: Option<String>,
    pub position:       i16,
    pub image:          Option<String>,
    pub slug:           String,
}

#[derive(Debug, Serialize, Clone, Queryable, Identifiable)]
pub struct Item {
    pub id:             i32,
    pub title:          String,
    pub title_en:       String,
    pub description:    Option<String>,
    pub description_en: Option<String>,
    pub content:        Option<String>,
    pub content_en:     Option<String>,
    pub link:           Option<String>,
    pub image:          Option<String>,
    pub is_active:      bool,
    pub price:          i32,
    pub user_id:        i32,
    pub created:        chrono::NaiveDateTime,
    pub position:       i16,
    pub view:           i32,
    pub height:         f64,
    pub seconds:        i32,
    pub price_acc:      Option<i32>,
    pub types:          i16,
    pub slug:           String,
}

impl Item {
    pub fn publish(user: User, id: i32) -> i16 {
        let _connection = establish_connection();
        let _item = Item::get_with_id(id);
        if user.perm < 60 && _item.user_id != user.id {
            return 0;
        }
        diesel::update(&_item)
            .set(schema::items::is_active.eq(true))
            .execute(&_connection)
            .expect("Error.");

        let _categories = _item.get_categories_obj();
        let _tags = _item.get_tags_obj();
        for _category in _categories.iter() {
            diesel::update(_category)
                .set(schema::categories::count.eq(_category.count + 1))
                .execute(&_connection)
                .expect("Error.");
        }
        for _tag in _tags.iter() {
            diesel::update(_tag)
                .set(schema::tags::count.eq(_tag.count + 1))
                .execute(&_connection)
                .expect("Error.");
        }
        return 1;
    }
    pub fn hide(user: User, id: i32) -> i16 {
        let _connection = establish_connection();
        let _item = Item::get_with_id(id);
        if user.perm < 60 && _item.user_id != user.id {
            return 0;
        }
        diesel::update(&_item)
            .set(schema::items::is_active.eq(false))
            .execute(&_connection)
            .expect("Error.");

        let _categories = _item.get_categories_obj();
        let _tags = _item.get_tags_obj();
        for _category in _categories.iter() {
            diesel::update(_category)
                .set(schema::categories::count.eq(_category.count - 1))
                .execute(&_connection)
                .expect("Error.");
        }
        for _tag in _tags.iter() {
            diesel::update(_tag)
                .set(schema::tags::count.eq(_tag.count - 1))
                .execute(&_connection)
                .expect("Error.");
        }
        return 1;
    }
    pub fn create(user_id: i32, form: crate::utils::ItemForms) -> i16 {
        use crate::models::{
            NewTechCategoriesItem,
            Serve,
            NewServeItems,
            NewCategory,
            NewTagItems,
        }; 

        let _connection = establish_connection();
        let l = get_linguage_storage();
        let types = form.types;
        let _item: Item;
        if l == 1 {
            let new_item = NewItem::create ( 
                form.title.clone(),
                "".to_string(),
                form.description.clone(),
                None,
                form.link.clone(),
                form.main_image.clone(),
                user_id,
                form.position,
                types,
                form.slug.clone(),
            );

            _item = diesel::insert_into(schema::items::table)
                .values(&new_item)
                .get_result::<Item>(&_connection)
                .expect("E.");
        }
        else if l == 2 {
            let new_item = NewItem::create ( 
                "".to_string(),
                form.title.clone(),
                None,
                form.description.clone(),
                form.link.clone(),
                form.main_image.clone(),
                user_id,
                form.position,
                types,
                form.slug.clone(),
            );

            _item = diesel::insert_into(schema::items::table)
                .values(&new_item)
                .get_result::<Item>(&_connection)
                .expect("E.");
        }
        else {
            return 0;
        }

        for category_id in form.category_list.into_iter() {
            let new_category = NewCategory {
                category_id: category_id,
                item_id:     _item.id,
                types:       types,
            };
            diesel::insert_into(schema::category::table)
                .values(&new_category)
                .execute(&_connection)
                .expect("E.");
        };
        for tag_id in form.tags_list.into_iter() {
            let new_tag = NewTagItems {
                tag_id:  tag_id,
                item_id: _item.id,
                types:   types,
                created: chrono::Local::now().naive_utc(),
            };
            diesel::insert_into(schema::tags_items::table)
                .values(&new_tag)
                .execute(&_connection)
                .expect("Error.");
        }

        // создаем связь с тех категориями, которые будут
        // расширять списки опций, предлагая доп возможности и услуги
        for cat_id in form.close_tech_cats_list.into_iter() {
            let new_cat = NewTechCategoriesItem {
                category_id: cat_id,
                item_id:     _item.id,
                types:       types,
                is_active:   2,
            };
            diesel::insert_into(schema::tech_categories_items::table)
                .values(&new_cat)
                .execute(&_connection)
                .expect("Error.");
        }

        // создаем опции услуги и записываем id опций в вектор.
        let mut serve_ids = Vec::new();
        for serve_id in form.serve_list.into_iter() {
            let new_serve_form = NewServeItems {
                serve_id: serve_id,
                item_id:  _item.id,
                types:    types,
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

        let mut tech_cat_ids = Vec::new();
        let mut item_price = 0;
        for _serve in _serves.iter() {
            if !tech_cat_ids.iter().any(|&i| i==_serve.tech_cat_id) {
                tech_cat_ids.push(_serve.tech_cat_id);
            }
            item_price += _serve.price;
        }

        for id in tech_cat_ids.into_iter() {
            let new_cat = NewTechCategoriesItem {
                category_id: id,
                item_id:     _item.id,
                types:       types,
                is_active:   1,
            };
            diesel::insert_into(schema::tech_categories_items::table)
                .values(&new_cat)
                .execute(&_connection)
                .expect("Error.");
        }

        // фух. Связи созданы все, но надо еще посчитать цену
        // услуги для калькулятора. Как? А  это будет сумма всех
        // цен выбранных опций.
        let price_acc = crate::utils::get_price_acc_values(&item_price);
        diesel::update(&_item)
            .set((
                schema::items::price.eq(item_price),
                schema::items::price_acc.eq(price_acc),
            ))
            .execute(&_connection)
            .expect("Error.");
        
        return 1;
    }
    pub fn update_content_with_id(user: User, item_id: i32, form: crate::utils::ContentForm) -> i16 {
        let l = get_linguage_storage();
        let _connection = establish_connection();
        let _item = schema::items::table
            .filter(schema::items::id.eq(item_id))
            .first::<Item>(&_connection)
            .expect("E");
        if user.perm < 60 && user.id != _item.user_id {
            return 0;
        }
        if l == 1 { 
            diesel::update(&_item)
                .set(schema::items::content.eq(form.content.clone()))
                .execute(&_connection)
                .expect("E");
        }
        else if l == 2 { 
            diesel::update(&_item)
                .set(schema::items::content_en.eq(form.content.clone()))
                .execute(&_connection)
                .expect("E");
        }
        return 0;
    }
    pub fn update_item_with_id(id: i32, form: crate::utils::ItemForms) -> i16 {
        let l = get_linguage_storage();
        let _connection = establish_connection();

        let _item = items
            .filter(schema::items::id.eq(id))
            .first::<Item>(&_connection)
            .expect("E");

        if l == 1 {
            let _new_item = EditItem {
                title:          form.title.clone(),
                title_en:       form.title.clone(),
                description:    form.description.clone(),
                description_en: form.description.clone(),
                link:           form.link.clone(),
                image:          form.main_image.clone(),
                position:       form.position,
                slug:           form.slug.clone(),
            };

            diesel::update(&_item)
                .set(_new_item)
                .execute(&_connection)
                .expect("E");
        }
        else if l == 2 {
            let _new_item = EditItem {
                title:          form.title.clone(),
                title_en:       form.title.clone(),
                description:    form.description.clone(),
                description_en: form.description.clone(),
                link:           form.link.clone(),
                image:          form.main_image.clone(),
                position:       form.position,
                slug:           form.slug.clone(),
            };

            diesel::update(&_item)
                .set(_new_item)
                .execute(&_connection)
                .expect("E");
        }
        else {
            return 0;
        }

        use crate::schema::{
            tags::dsl::tags,
            items::dsl::items,
            serve_items::dsl::serve_items,
            tags_items::dsl::tags_items,
            categories::dsl::categories,
            category::dsl::category,
            tech_categories_items::dsl::tech_categories_items,
            serve::dsl::serve,
        };
        use crate::models::{
            NewTechCategoriesItem,
            Serve,
            NewServeItems,
            NewCategory,
            NewTagItems,
        };

        if _item.is_active {
            let _categories = _item.get_categories_obj();
            let _tags = _item.get_tags_obj();

            for _category in _categories.iter() {
                diesel::update(_category)
                    .set(schema::categories::count.eq(_category.count - 1))
                    .execute(&_connection)
                    .expect("Error.");
            };
            for _tag in _tags.iter() {
                diesel::update(_tag)
                    .set(schema::tags::count.eq(_tag.count - 1))
                    .execute(&_connection)
                    .expect("Error.");
            };
        }

        diesel::delete (
            tags_items
                .filter(schema::tags_items::item_id.eq(id))
                .filter(schema::tags_items::types.eq(_item.types))
            )
            .execute(&_connection)
            .expect("E");
        diesel::delete (
            serve_items
                .filter(schema::serve_items::item_id.eq(id))
                .filter(schema::serve_items::types.eq(_item.types))
            )
            .execute(&_connection)
            .expect("E");
        diesel::delete (
            tech_categories_items
                .filter(schema::tech_categories_items::item_id.eq(id))
                .filter(schema::tech_categories_items::types.eq(_item.types))
            )
            .execute(&_connection)
            .expect("E");
        diesel::delete (
            category
                .filter(schema::category::item_id.eq(id))
                .filter(schema::category::types.eq(_item.types))
            )
            .execute(&_connection)
            .expect("E");

        for category_id in form.category_list.into_iter() {
            let new_category = NewCategory {
                category_id: category_id,
                item_id:     _item.id,
                types:       _item.types,
            };
            diesel::insert_into(schema::category::table)
                .values(&new_category)
                .execute(&_connection)
                .expect("E.");

            if _item.is_active {
                let _category = categories
                    .filter(schema::categories::id.eq(category_id))
                    .filter(schema::categories::types.eq(_item.types))
                    .first::<Categories>(&_connection)
                    .expect("E");
                diesel::update(&_category)
                    .set(schema::categories::count.eq(_category.count + 1))
                    .execute(&_connection)
                    .expect("Error.");
            }
        };
        for tag_id in form.tags_list.into_iter() {
            let new_tag = NewTagItems {
                tag_id:  tag_id,
                item_id: _item.id,
                types:   _item.types,
                created: chrono::Local::now().naive_utc(),
            };
            diesel::insert_into(schema::tags_items::table)
                .values(&new_tag)
                .execute(&_connection)
                .expect("Error.");

            if _item.is_active {
                let _tag = tags
                    .filter(schema::tags::id.eq(tag_id))
                    .first::<Tag>(&_connection)
                    .expect("E");

                diesel::update(&_tag)
                    .set(schema::tags::count.eq(_tag.count + 1))
                    .execute(&_connection)
                    .expect("Error.");
            }
        }

        // создаем связь с тех категориями, которые будут
        // расширять списки опций, предлагая доп возможности и услуги
        for cat_id in form.close_tech_cats_list.into_iter() {
            let new_cat = NewTechCategoriesItem {
                category_id: cat_id,
                item_id:     _item.id,
                types:       _item.types,
                is_active:   2,
            };
            diesel::insert_into(schema::tech_categories_items::table)
                .values(&new_cat)
                .execute(&_connection)
                .expect("Error.");
        }

        // создаем опции услуги и записываем id опций в вектор.
        let mut serve_ids = Vec::new();
        for serve_id in form.serve_list.into_iter() {
            let new_serve_form = NewServeItems {
                serve_id: serve_id,
                item_id:  _item.id,
                types:    _item.types,
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

        let mut tech_cat_ids = Vec::new();
        let mut item_price = 0;
        for _serve in _serves.iter() {
            if !tech_cat_ids.iter().any(|&i| i==_serve.tech_cat_id) {
                tech_cat_ids.push(_serve.tech_cat_id);
            }
            item_price += _serve.price;
        }

        for id in tech_cat_ids.into_iter() {
            let new_cat = NewTechCategoriesItem {
                category_id: id,
                item_id:     _item.id,
                types:       _item.types,
                is_active:   1,
            };
            diesel::insert_into(schema::tech_categories_items::table)
                .values(&new_cat)
                .execute(&_connection)
                .expect("Error.");
        }

        // фух. Связи созданы все, но надо еще посчитать цену
        // услуги для калькулятора. Как? А  это будет сумма всех
        // цен выбранных опций.
        let price_acc = crate::utils::get_price_acc_values(&item_price);
        diesel::update(&_item)
            .set((
                schema::items::price.eq(item_price),
                schema::items::price_acc.eq(price_acc),
            ))
            .execute(&_connection)
            .expect("Error.");
        return 1;
    }
    pub fn get(slug: &String) -> Item {
        let _connection = establish_connection();
        return schema::items::table
            .filter(schema::items::slug.eq(slug))
            .first::<Item>(&_connection)
            .expect("E.");
    }
    pub fn get_with_id(id: i32) -> Item {
        let _connection = establish_connection();
        return schema::items::table
            .filter(schema::items::id.eq(id))
            .first::<Item>(&_connection)
            .expect("E.");
    }
    pub fn get_type(&self) -> String {
        return match self.types {
            1 => "блог".to_string(),
            2 => "услуга".to_string(),
            3 => "товар".to_string(),
            4 => "wiki".to_string(),
            5 => "работа".to_string(),
            6 => "помощь".to_string(),
            _ => "Непонятно".to_string(),
        };
    }
    pub fn get_image(&self) -> String {
        if self.image.is_some() {
            return self.image.as_deref().unwrap().to_string();
        }
        else {
            return "/static/images/img.jpg".to_string();
        }
    }
    pub fn get_files(&self) -> (
        Vec<SmallFile>, // photos id, src, description
        Vec<SmallFile>, // videos id, src, description
        Vec<SmallFile>, // audios id, src, description
        Vec<SmallFile>  // docs id, src, description
    ) {
        use schema::files::dsl::files;

        let _connection = establish_connection();
        let photos = files
            .filter(schema::files::item_id.eq(self.id))
            .filter(schema::files::item_types.eq(self.types))
            .filter(schema::files::types.eq(1))
            .select((
                schema::files::id, 
                schema::files::src, 
                schema::files::description.nullable(),
            ))
            .load::<SmallFile>(&_connection)
            .expect("E");
        let videos = files
            .filter(schema::files::item_id.eq(self.id))
            .filter(schema::files::item_types.eq(self.types))
            .filter(schema::files::types.eq(2))
            .select((
                schema::files::id, 
                schema::files::src, 
                schema::files::description.nullable(),
            ))
            .load::<SmallFile>(&_connection)
            .expect("E");
        let audios = files
            .filter(schema::files::item_id.eq(self.id))
            .filter(schema::files::item_types.eq(self.types))
            .filter(schema::files::types.eq(3))
            .select((
                schema::files::id, 
                schema::files::src, 
                schema::files::description.nullable(),
            ))
            .load::<SmallFile>(&_connection)
            .expect("E");
        let docs = files
            .filter(schema::files::item_id.eq(self.id))
            .filter(schema::files::item_types.eq(self.types))
            .filter(schema::files::types.eq(4))
            .select((
                schema::files::id, 
                schema::files::src, 
                schema::files::description.nullable(),
            ))
            .load::<SmallFile>(&_connection)
            .expect("E");

        return (photos, videos, audios, docs);
    }
    pub fn get_images_ids(&self) -> Vec<i32> {
        use schema::files::dsl::files;

        let _connection = establish_connection();
        return files
            .filter(schema::files::item_id.eq(self.id))
            .filter(schema::files::types.eq(1))
            .select(schema::files::id)
            .load::<i32>(&_connection)
            .expect("E");
    }
    pub fn get_100_description(&self, l: u8) -> String {
        if l == 1 {
            if self.description.is_some() {
                let _content = self.description.as_deref().unwrap();
                if _content.len() > 100 {
                    return _content[..100].to_string();
                }
                return _content.to_string();
            }
        }
        else if l == 2 {
            if self.description_en.is_some() {
                let _content = self.description_en.as_deref().unwrap();
                if _content.len() > 100 {
                    return _content[..100].to_string();
                }
                return _content.to_string();
            }
        }
        return "".to_string();
    }

    pub fn get_categories(&self) -> Vec<SmallCat> {
        use crate::schema::{
            category::dsl::category,
            categories::dsl::categories,
        };
        let _connection = establish_connection();
        let ids = category
            .filter(schema::category::item_id.eq(self.id))
            .filter(schema::category::types.eq(self.types))
            .select(schema::category::category_id)
            .load::<i32>(&_connection)
            .expect("E");

        let _categories = categories
            .filter(schema::categories::id.eq_any(ids))
            .select((
                schema::categories::name,
                schema::categories::slug,
                schema::categories::count
            ))
            .load::<SmallCat>(&_connection)
            .expect("E");
        return _categories;
    }
    pub fn get_categories_obj(&self) -> Vec<Categories> {
        use crate::schema::{
            category::dsl::category,
            categories::dsl::categories,
        };

        let _connection = establish_connection();
        let ids = category
            .filter(schema::category::item_id.eq(self.id))
            .filter(schema::category::types.eq(self.types))
            .select(schema::category::category_id)
            .load::<i32>(&_connection)
            .expect("E");

        let _categories = categories
            .filter(schema::categories::id.eq_any(ids))
            .load::<Categories>(&_connection)
            .expect("E");
        return _categories;
    }

    pub fn get_tags(&self, l: u8) -> Vec<SmallTag> {
        let _connection = establish_connection();
        let _tag_items = schema::tags_items::table
            .filter(schema::tags_items::item_id.eq(&self.id))
            .filter(schema::tags_items::types.eq(self.types))
            .select(schema::tags_items::tag_id)
            .load::<i32>(&_connection)
            .expect("E");
        return Tag::get_tags_with_ids(_tag_items, l);
    }
    pub fn get_tags_obj(&self) -> Vec<Tag> {
        use crate::schema::{
            tags_items::dsl::tags_items,
            tags::dsl::tags,
        };
        let _connection = establish_connection();

        let _tag_items = tags_items
            .filter(schema::tags_items::item_id.eq(&self.id))
            .filter(schema::tags_items::types.eq(self.types))
            .select(schema::tags_items::tag_id)
            .load::<i32>(&_connection)
            .expect("E");
        let _tags = tags
            .filter(schema::tags::id.eq_any(_tag_items))
            .load::<Tag>(&_connection)
            .expect("E");
        return _tags;
    }

    pub fn get_blogs (
        limit:    i64,
        offset:   i64,
        is_admin: bool
    ) -> Vec<Blog> {
        use crate::schema::items::dsl::items;

        let _connection = establish_connection();
        if is_admin {
             return items
                .filter(schema::items::types.eq(1))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::is_active,
                    schema::items::title,
                    schema::items::created,
                    schema::items::description.nullable(),
                ))
                .load::<Blog>(&_connection)
                .expect("E.");
        } else {
            return items
                .filter(schema::items::types.eq(1))
                .filter(schema::items::is_active.eq(true))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::is_active,
                    schema::items::title,
                    schema::items::created,
                    schema::items::description.nullable(),
                ))
                .load::<Blog>(&_connection)
                .expect("E.");
        }
    }
    pub fn search_blogs (
        q:        &String,
        limit:    i64,
        offset:   i64,
        is_admin: bool
    ) -> Vec<Blog> {
        use crate::schema::items::dsl::items;

        let _connection = establish_connection();
        if is_admin {
             return items
                .filter(schema::items::title.ilike(&q))
                .or_filter(schema::items::description.ilike(&q))
                .or_filter(schema::items::content.ilike(&q))
                .filter(schema::items::types.eq(1))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::is_active,
                    schema::items::title,
                    schema::items::created,
                    schema::items::description.nullable(),
                ))
                .load::<Blog>(&_connection)
                .expect("E.");
        } else {
            return items
                .filter(schema::items::title.ilike(&q))
                .or_filter(schema::items::description.ilike(&q))
                .or_filter(schema::items::content.ilike(&q))
                .filter(schema::items::types.eq(1))
                .filter(schema::items::is_active.eq(true))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::is_active,
                    schema::items::title,
                    schema::items::created,
                    schema::items::description.nullable(),
                ))
                .load::<Blog>(&_connection)
                .expect("E.");
        }
    }

    pub fn get_services (
        limit:    i64,
        offset:   i64,
        is_admin: bool
    ) -> Vec<Service> {
        use crate::schema::items::dsl::items;

        let _connection = establish_connection();
        if is_admin {
             return items
                .filter(schema::items::types.eq(2))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::is_active,
                    schema::items::title,
                    schema::items::description.nullable(),
                ))
                .load::<Service>(&_connection)
                .expect("E.");
        } else {
            return items
                .filter(schema::items::types.eq(2))
                .filter(schema::items::is_active.eq(true))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::is_active,
                    schema::items::title,
                    schema::items::description.nullable(),
                ))
                
                .load::<Service>(&_connection)
                .expect("E.");
        }
    }
    pub fn search_services (
        q:        &String,
        limit:    i64,
        offset:   i64,
        is_admin: bool
    ) -> Vec<Service> {
        use crate::schema::items::dsl::items;

        let _connection = establish_connection();
        if is_admin {
             return items
                .filter(schema::items::title.ilike(&q))
                .or_filter(schema::items::description.ilike(&q))
                .or_filter(schema::items::content.ilike(&q))
                .filter(schema::items::types.eq(2))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::is_active,
                    schema::items::title,
                    schema::items::description.nullable(),
                ))
                .load::<Service>(&_connection)
                .expect("E.");
        } else {
            return items
                .filter(schema::items::title.ilike(&q))
                .or_filter(schema::items::description.ilike(&q))
                .or_filter(schema::items::content.ilike(&q))
                .filter(schema::items::types.eq(2))
                .filter(schema::items::is_active.eq(true))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::slug,
                    schema::items::image.nullable(),
                    schema::items::is_active,
                    schema::items::title,
                    schema::items::description.nullable(),
                ))
                .load::<Service>(&_connection)
                .expect("E.");
        }
    }

    pub fn get_stores (
          limit:    i64,
          offset:   i64,
          is_admin: bool
      ) -> Vec<Store> {
          use crate::schema::items::dsl::items;

          let _connection = establish_connection();
          if is_admin {
               return items
                  .filter(schema::items::types.eq(3))
                  .order(schema::items::created.desc())
                  .limit(limit)
                  .offset(offset)
                  .select((
                        schema::items::id,
                        schema::items::slug,
                        schema::items::image.nullable(),
                        schema::items::is_active,
                        schema::items::title,
                        schema::items::description.nullable(),
                        schema::items::price,
                        schema::items::price_acc.nullable(),
                  ))
                  .load::<Store>(&_connection)
                  .expect("E.");
          } else {
              return items
                  .filter(schema::items::types.eq(3))
                  .filter(schema::items::is_active.eq(true))
                  .order(schema::items::created.desc())
                  .limit(limit)
                  .offset(offset)
                  .select((
                      schema::items::id,
                      schema::items::slug,
                      schema::items::image.nullable(),
                      schema::items::is_active,
                      schema::items::title,
                      schema::items::description.nullable(),
                      schema::items::price,
                      schema::items::price_acc.nullable(),
                  ))
                  .load::<Store>(&_connection)
                  .expect("E.");
          }
    }
    pub fn search_stores (
          q:        &String,
          limit:    i64,
          offset:   i64,
          is_admin: bool
      ) -> Vec<Store> {
          use crate::schema::items::dsl::items;

          let _connection = establish_connection();
          if is_admin {
               return items
                  .filter(schema::items::title.ilike(&q))
                  .or_filter(schema::items::description.ilike(&q))
                  .or_filter(schema::items::content.ilike(&q))
                  .filter(schema::items::types.eq(3))
                  .order(schema::items::created.desc())
                  .limit(limit)
                  .offset(offset)
                  .select((
                      schema::items::id,
                      schema::items::slug,
                      schema::items::image.nullable(),
                      schema::items::is_active,
                      schema::items::title,
                      schema::items::description.nullable(),
                      schema::items::price,
                      schema::items::price_acc.nullable(),
                  ))
                  .load::<Store>(&_connection)
                  .expect("E.");
          } else {
              return items
                  .filter(schema::items::title.ilike(&q))
                  .or_filter(schema::items::description.ilike(&q))
                  .or_filter(schema::items::content.ilike(&q))
                  .filter(schema::items::types.eq(3))
                  .filter(schema::items::is_active.eq(true))
                  .order(schema::items::created.desc())
                  .limit(limit)
                  .offset(offset)
                  .select((
                      schema::items::id,
                      schema::items::slug,
                      schema::items::image.nullable(),
                      schema::items::is_active,
                      schema::items::title,
                      schema::items::description.nullable(),
                      schema::items::price,
                      schema::items::price_acc.nullable(),
                  ))
                  .load::<Store>(&_connection)
                  .expect("E.");
          }
    }

    pub fn get_works (
          limit:    i64,
          offset:   i64,
          is_admin: bool
      ) -> Vec<Work> {
          use crate::schema::items::dsl::items;

          let _connection = establish_connection();
          if is_admin {
               return items
                  .filter(schema::items::types.eq(5))
                  .order(schema::items::created.desc())
                  .limit(limit)
                  .offset(offset)
                  .select((
                      schema::items::id,
                      schema::items::slug,
                      schema::items::image.nullable(),
                      schema::items::is_active,
                      schema::items::title,
                      schema::items::description.nullable(),
                  ))
                  .load::<Work>(&_connection)
                  .expect("E.");
          } else {
              return items
                  .filter(schema::items::types.eq(5))
                  .filter(schema::items::is_active.eq(true))
                  .order(schema::items::created.desc())
                  .limit(limit)
                  .offset(offset)
                  .select((
                      schema::items::id,
                      schema::items::slug,
                      schema::items::image.nullable(),
                      schema::items::is_active,
                      schema::items::title,
                      schema::items::description.nullable(),
                  ))
                  .load::<Work>(&_connection)
                  .expect("E.");
        }
    }
    pub fn search_works (
          q:        &String,
          limit:    i64,
          offset:   i64,
          is_admin: bool
      ) -> Vec<Work> {
          use crate::schema::items::dsl::items;

          let _connection = establish_connection();
          if is_admin {
               return items
                  .filter(schema::items::title.ilike(&q))
                  .or_filter(schema::items::description.ilike(&q))
                  .or_filter(schema::items::content.ilike(&q))
                  .filter(schema::items::types.eq(5))
                  .order(schema::items::created.desc())
                  .limit(limit)
                  .offset(offset)
                  .select((
                      schema::items::id,
                      schema::items::slug,
                      schema::items::image.nullable(),
                      schema::items::is_active,
                      schema::items::title,
                      schema::items::description.nullable(),
                  ))
                  .load::<Work>(&_connection)
                  .expect("E.");
          } else {
              return items
                  .filter(schema::items::title.ilike(&q))
                  .or_filter(schema::items::description.ilike(&q))
                  .or_filter(schema::items::content.ilike(&q))
                  .filter(schema::items::types.eq(5))
                  .filter(schema::items::is_active.eq(true))
                  .order(schema::items::created.desc())
                  .limit(limit)
                  .offset(offset)
                  .select((
                      schema::items::id,
                      schema::items::slug,
                      schema::items::image.nullable(),
                      schema::items::is_active,
                      schema::items::title,
                      schema::items::description.nullable(),
                  ))
                  .load::<Work>(&_connection)
                  .expect("E.");
        }
    }

    pub fn get_wikis (
          limit:    i64,
          offset:   i64,
          is_admin: bool
      ) -> Vec<Wiki> {
          use crate::schema::items::dsl::items;

          let _connection = establish_connection();
          if is_admin {
               return items
                  .filter(schema::items::types.eq(4))
                  .order(schema::items::created.desc())
                  .limit(limit)
                  .offset(offset)
                  .select((
                      schema::items::id,
                      schema::items::slug,
                      schema::items::image.nullable(),
                      schema::items::is_active,
                      schema::items::title,
                      schema::items::description.nullable(),
                      schema::items::created,
                  ))
                  .load::<Wiki>(&_connection)
                  .expect("E.");
          } else {
              return items
                  .filter(schema::items::types.eq(4))
                  .filter(schema::items::is_active.eq(true))
                  .order(schema::items::created.desc())
                  .limit(limit)
                  .offset(offset)
                  .select((
                      schema::items::id,
                      schema::items::slug,
                      schema::items::image.nullable(),
                      schema::items::is_active,
                      schema::items::title,
                      schema::items::description.nullable(),
                      schema::items::created
                  ))
                  .load::<Wiki>(&_connection)
                  .expect("E.");
        }
    }
    pub fn search_wikis (
          q:        &String,
          limit:    i64,
          offset:   i64,
          is_admin: bool
      ) -> Vec<Wiki> {
          use crate::schema::items::dsl::items;

          let _connection = establish_connection();
          if is_admin {
               return items
                  .filter(schema::items::title.ilike(&q))
                  .or_filter(schema::items::description.ilike(&q))
                  .or_filter(schema::items::content.ilike(&q))
                  .filter(schema::items::types.eq(4))
                  .order(schema::items::created.desc())
                  .limit(limit)
                  .offset(offset)
                  .select((
                      schema::items::id,
                      schema::items::slug,
                      schema::items::image.nullable(),
                      schema::items::is_active,
                      schema::items::title,
                      schema::items::description.nullable(),
                      schema::items::created
                  ))
                  .load::<Wiki>(&_connection)
                  .expect("E.");
          } else {
              return items
                  .filter(schema::items::title.ilike(&q))
                  .or_filter(schema::items::description.ilike(&q))
                  .or_filter(schema::items::content.ilike(&q))
                  .filter(schema::items::types.eq(4))
                  .filter(schema::items::is_active.eq(true))
                  .order(schema::items::created.desc())
                  .limit(limit)
                  .offset(offset)
                  .select((
                      schema::items::id,
                      schema::items::slug,
                      schema::items::image.nullable(),
                      schema::items::is_active,
                      schema::items::title,
                      schema::items::description.nullable(),
                      schema::items::created,
                  ))
                  .load::<Wiki>(&_connection)
                  .expect("E.");
        }
    }

    pub fn get_helps (
        limit:    i64,
        offset:   i64,
        is_admin: bool
    ) -> Vec<Help> {
        use crate::schema::items::dsl::items;

        let _connection = establish_connection();
        if is_admin {
             return items
                .filter(schema::items::types.eq(6))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::is_active,
                    schema::items::title,
                    schema::items::content,
                ))
                .load::<Help>(&_connection)
                .expect("E.");
        } else {
            return items
                .filter(schema::items::types.eq(6))
                .filter(schema::items::is_active.eq(true))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::is_active,
                    schema::items::title,
                    schema::items::content,
                ))
                .load::<Help>(&_connection)
                .expect("E.");
        }
    }

    pub fn search_helps (
        q:        &String,
        limit:    i64,
        offset:   i64,
        is_admin: bool
    ) -> Vec<Help> {
        use crate::schema::items::dsl::items;

        let _connection = establish_connection();
        if is_admin {
             return items
                .filter(schema::items::title.ilike(&q))
                .or_filter(schema::items::description.ilike(&q))
                .or_filter(schema::items::content.ilike(&q))
                .filter(schema::items::types.eq(6))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::is_active,
                    schema::items::title,
                    schema::items::content,
                ))
                .load::<Help>(&_connection)
                .expect("E.");
        } else {
            return items
                .filter(schema::items::title.ilike(&q))
                .or_filter(schema::items::description.ilike(&q))
                .or_filter(schema::items::content.ilike(&q))
                .filter(schema::items::is_active.eq(true))
                .order(schema::items::created.desc())
                .limit(limit)
                .offset(offset)
                .select((
                    schema::items::id,
                    schema::items::is_active,
                    schema::items::title,
                    schema::items::content,
                ))
                .load::<Help>(&_connection)
                .expect("E.");
        }
    }

    pub fn get_blogs_list_for_ids (
        page:     i32,
        limit:    i32,
        ids:      Vec<i32>,
        is_admin: bool,
        l:        u8
    ) -> ((Vec<Blog>, usize), i32) {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: (Vec<Blog>, usize);

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = Item::get_blogs_for_ids(limit.into(), step.into(), ids.clone(), is_admin, l);
        }
        else {
            have_next = limit + 1;
            object_list = Item::get_blogs_for_ids(limit.into(), 0, ids.clone(), is_admin, l);
        }
        if Item::get_blogs_for_ids(1, have_next.into(), ids, is_admin, l).0.len() > 0 {
            next_page_number = page + 1;
        }
        return (object_list, next_page_number);
    }

    pub fn get_blogs_for_ids (
        limit:    i64,
        offset:   i64,
        ids:      Vec<i32>,
        is_admin: bool,
        l:        u8
    ) -> (Vec<Blog>, usize) {
        use crate::schema::items::dsl::items;

        let _connection = establish_connection();
        let count = Item::get_count_for_ids(&ids, is_admin);
        if is_admin {
            if l == 1 {
                return (items
                    .filter(schema::items::id.eq_any(ids))
                    .order(schema::items::created.desc())
                    .limit(limit)
                    .offset(offset)
                    .select((
                        schema::items::id,
                        schema::items::slug,
                        schema::items::image.nullable(),
                        schema::items::is_active,
                        schema::items::title,
                        schema::items::created,
                        schema::items::description.nullable(),
                    ))
                    .load::<Blog>(&_connection)
                    .expect("E."), count);
            }
            else if l == 2 {
                return (items
                    .filter(schema::items::id.eq_any(ids))
                    .order(schema::items::created.desc())
                    .limit(limit)
                    .offset(offset)
                    .select((
                        schema::items::id,
                        schema::items::slug,
                        schema::items::image.nullable(),
                        schema::items::is_active,
                        schema::items::title_en,
                        schema::items::created,
                        schema::items::description_en.nullable(),
                    ))
                    .load::<Blog>(&_connection)
                    .expect("E."), count);
            }
        }
        else {
            if l == 1 {
                return (items
                    .filter(schema::items::id.eq_any(ids))
                    .filter(schema::items::is_active.eq(true))
                    .order(schema::items::created.desc())
                    .limit(limit)
                    .offset(offset)
                    .select((
                        schema::items::id,
                        schema::items::slug,
                        schema::items::image.nullable(),
                        schema::items::is_active,
                        schema::items::title,
                        schema::items::created,
                        schema::items::description.nullable(),
                    ))
                    .load::<Blog>(&_connection)
                    .expect("E."), count);
            }
            else if l == 2 {
                return (items
                    .filter(schema::items::id.eq_any(ids))
                    .filter(schema::items::is_active.eq(true))
                    .order(schema::items::created.desc())
                    .limit(limit)
                    .offset(offset)
                    .select((
                        schema::items::id,
                        schema::items::slug,
                        schema::items::image.nullable(),
                        schema::items::is_active,
                        schema::items::title_en,
                        schema::items::created,
                        schema::items::description_en.nullable(),
                    ))
                    .load::<Blog>(&_connection)
                    .expect("E."), count);
            }
        }
        return (Vec::new(), 0);
    }

    pub fn get_services_list_for_ids (
        page:     i32,
        limit:    i32,
        ids:      Vec<i32>,
        is_admin: bool,
        l:        u8
    ) -> ((Vec<Service>, usize), i32) {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: (Vec<Service>, usize);

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = Item::get_services_for_ids(limit.into(), step.into(), ids.clone(), is_admin, l);
        }
        else {
            have_next = limit + 1;
            object_list = Item::get_services_for_ids(limit.into(), 0, ids.clone(), is_admin, l);
        }
        if Item::get_services_for_ids(1, have_next.into(), ids, is_admin, l).0.len() > 0 {
            next_page_number = page + 1;
        }
        return (object_list, next_page_number);
    }

    pub fn get_services_for_ids (
        limit:    i64,
        offset:   i64,
        ids:      Vec<i32>,
        is_admin: bool,
        l:        u8
    ) -> (Vec<Service>, usize) {
        use crate::schema::items::dsl::items;

        let _connection = establish_connection();
        let count = Item::get_count_for_ids(&ids, is_admin);
        if is_admin {
            if l == 1 {
                return (items
                    .filter(schema::items::id.eq_any(ids))
                    .order(schema::items::created.desc())
                    .limit(limit)
                    .offset(offset)
                    .select((
                        schema::items::id,
                        schema::items::slug,
                        schema::items::image.nullable(),
                        schema::items::is_active,
                        schema::items::title,
                        schema::items::description.nullable(),
                    ))
                    .load::<Service>(&_connection)
                    .expect("E."), count);
            }
            else if l == 2 {
                return (items
                    .filter(schema::items::id.eq_any(ids))
                    .order(schema::items::created.desc())
                    .limit(limit)
                    .offset(offset)
                    .select((
                        schema::items::id,
                        schema::items::slug,
                        schema::items::image.nullable(),
                        schema::items::is_active,
                        schema::items::title_en,
                        schema::items::description_en.nullable(),
                    ))
                    .load::<Service>(&_connection)
                    .expect("E."), count);
            }
        }
        else {
            if l == 1 {
                return (items
                    .filter(schema::items::id.eq_any(ids))
                    .filter(schema::items::is_active.eq(true))
                    .order(schema::items::created.desc())
                    .limit(limit)
                    .offset(offset)
                    .select((
                        schema::items::id,
                        schema::items::slug,
                        schema::items::image.nullable(),
                        schema::items::is_active,
                        schema::items::title,
                        schema::items::description.nullable(),
                    ))
                    .load::<Service>(&_connection)
                    .expect("E."), count);
            }
            else if l == 1 {
                return (items
                    .filter(schema::items::id.eq_any(ids))
                    .filter(schema::items::is_active.eq(true))
                    .order(schema::items::created.desc())
                    .limit(limit)
                    .offset(offset)
                    .select((
                        schema::items::id,
                        schema::items::slug,
                        schema::items::image.nullable(),
                        schema::items::is_active,
                        schema::items::title_en,
                        schema::items::description_en.nullable(),
                    ))
                    .load::<Service>(&_connection)
                    .expect("E."), count);
            }
        }
        return (Vec::new(), 0);
    }

    pub fn get_stores_list_for_ids (
        page:     i32,
        limit:    i32,
        ids:      Vec<i32>,
        is_admin: bool,
        l:        u8
    ) -> ((Vec<Store>, usize), i32) {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: (Vec<Store>, usize);

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = Item::get_stores_for_ids(limit.into(), step.into(), ids.clone(), is_admin, l);
        }
        else {
            have_next = limit + 1;
            object_list = Item::get_stores_for_ids(limit.into(), 0, ids.clone(), is_admin, l);
        }
        if Item::get_stores_for_ids(1, have_next.into(), ids, is_admin, l).0.len() > 0 {
            next_page_number = page + 1;
        }
        return (object_list, next_page_number);
    }

    pub fn get_stores_for_ids (
        limit:    i64,
        offset:   i64,
        ids:      Vec<i32>,
        is_admin: bool,
        l:        u8
    ) -> (Vec<Store>, usize) {
        use crate::schema::items::dsl::items;

        let _connection = establish_connection();
        let count = Item::get_count_for_ids(&ids, is_admin);
        if is_admin {
            if l == 1 {
                return (items
                    .filter(schema::items::id.eq_any(ids))
                    .order(schema::items::created.desc())
                    .limit(limit)
                    .offset(offset)
                    .select((
                        schema::items::id,
                        schema::items::slug,
                        schema::items::image.nullable(),
                        schema::items::is_active,
                        schema::items::title,
                        schema::items::description.nullable(),
                        schema::items::price,
                        schema::items::price_acc.nullable()
                    ))
                    .load::<Store>(&_connection)
                    .expect("E."), count);
            }
            else if l == 2 {
                return (items
                    .filter(schema::items::id.eq_any(ids))
                    .order(schema::items::created.desc())
                    .limit(limit)
                    .offset(offset)
                    .select((
                        schema::items::id,
                        schema::items::slug,
                        schema::items::image.nullable(),
                        schema::items::is_active,
                        schema::items::title_en,
                        schema::items::description_en.nullable(),
                        schema::items::price,
                        schema::items::price_acc.nullable()
                    ))
                    .load::<Store>(&_connection)
                    .expect("E."), count);
            }
        }
        else {
            if l == 1 {
                return (items
                    .filter(schema::items::id.eq_any(ids))
                    .filter(schema::items::is_active.eq(true))
                    .order(schema::items::created.desc())
                    .limit(limit)
                    .offset(offset)
                    .select((
                        schema::items::id,
                        schema::items::slug,
                        schema::items::image.nullable(),
                        schema::items::is_active,
                        schema::items::title,
                        schema::items::description.nullable(),
                        schema::items::price,
                        schema::items::price_acc.nullable()
                    ))
                    .load::<Store>(&_connection)
                    .expect("E."), count);
            }
            else if l == 2 {
                return (items
                    .filter(schema::items::id.eq_any(ids))
                    .filter(schema::items::is_active.eq(true))
                    .order(schema::items::created.desc())
                    .limit(limit)
                    .offset(offset)
                    .select((
                        schema::items::id,
                        schema::items::slug,
                        schema::items::image.nullable(),
                        schema::items::is_active,
                        schema::items::title_en,
                        schema::items::description_en.nullable(),
                        schema::items::price,
                        schema::items::price_acc.nullable()
                    ))
                    .load::<Store>(&_connection)
                    .expect("E."), count);
            }
        }
        return (Vec::new(), 0);
    }

    pub fn get_wikis_list_for_ids (
        page:     i32,
        limit:    i32,
        ids:      Vec<i32>,
        is_admin: bool,
        l:        u8
    ) -> ((Vec<Wiki>, usize), i32) {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: (Vec<Wiki>, usize);

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = Item::get_wikis_for_ids(limit.into(), step.into(), ids.clone(), is_admin, l);
        }
        else {
            have_next = limit + 1;
            object_list = Item::get_wikis_for_ids(limit.into(), 0, ids.clone(), is_admin, l);
        }
        if Item::get_wikis_for_ids(1, have_next.into(), ids, is_admin, l).0.len() > 0 {
            next_page_number = page + 1;
        }
        return (object_list, next_page_number);
    }

    pub fn get_wikis_for_ids (
        limit:    i64,
        offset:   i64,
        ids:      Vec<i32>,
        is_admin: bool,
        l:        u8
    ) -> (Vec<Wiki>, usize) {
        use crate::schema::items::dsl::items;

        let _connection = establish_connection();
        let count = Item::get_count_for_ids(&ids, is_admin);
        if is_admin {
            if l == 1 {
                return (items
                    .filter(schema::items::id.eq_any(ids))
                    .order(schema::items::created.desc())
                    .limit(limit)
                    .offset(offset)
                    .select((
                        schema::items::id,
                        schema::items::slug,
                        schema::items::image.nullable(),
                        schema::items::is_active,
                        schema::items::title,
                        schema::items::description.nullable(),
                        schema::items::created,
                    ))
                    .load::<Wiki>(&_connection)
                    .expect("E."), count);
            }
            else if l == 2 {
                return (items
                    .filter(schema::items::id.eq_any(ids))
                    .order(schema::items::created.desc())
                    .limit(limit)
                    .offset(offset)
                    .select((
                        schema::items::id,
                        schema::items::slug,
                        schema::items::image.nullable(),
                        schema::items::is_active,
                        schema::items::title_en,
                        schema::items::description_en.nullable(),
                        schema::items::created,
                    ))
                    .load::<Wiki>(&_connection)
                    .expect("E."), count);
            }
        }
        else {
            if l == 1 {
                return (items
                    .filter(schema::items::id.eq_any(ids))
                    .filter(schema::items::is_active.eq(true))
                    .order(schema::items::created.desc())
                    .limit(limit)
                    .offset(offset)
                    .select((
                        schema::items::id,
                        schema::items::slug,
                        schema::items::image.nullable(),
                        schema::items::is_active,
                        schema::items::title,
                        schema::items::description.nullable(),
                        schema::items::created,
                    ))
                    .load::<Wiki>(&_connection)
                    .expect("E."), count);
            }
            else if l == 2 {
                return (items
                    .filter(schema::items::id.eq_any(ids))
                    .filter(schema::items::is_active.eq(true))
                    .order(schema::items::created.desc())
                    .limit(limit)
                    .offset(offset)
                    .select((
                        schema::items::id,
                        schema::items::slug,
                        schema::items::image.nullable(),
                        schema::items::is_active,
                        schema::items::title_en,
                        schema::items::description_en.nullable(),
                        schema::items::created,
                    ))
                    .load::<Wiki>(&_connection)
                    .expect("E."), count);
            }
        }
        return (Vec::new(), 0);
    }

    pub fn get_works_list_for_ids (
        page:     i32,
        limit:    i32,
        ids:      Vec<i32>,
        is_admin: bool,
        l:        u8
    ) -> ((Vec<Work>, usize), i32) {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: (Vec<Work>, usize);

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = Item::get_works_for_ids(limit.into(), step.into(), ids.clone(), is_admin, l);
        }
        else {
            have_next = limit + 1;
            object_list = Item::get_works_for_ids(limit.into(), 0, ids.clone(), is_admin, l);
        }
        if Item::get_works_for_ids(1, have_next.into(), ids, is_admin, l).0.len() > 0 {
            next_page_number = page + 1;
        }
        return (object_list, next_page_number);
    }

    pub fn get_works_for_ids (
        limit:    i64,
        offset:   i64,
        ids:      Vec<i32>,
        is_admin: bool,
        l:        u8
    ) -> (Vec<Work>, usize) {
        use crate::schema::items::dsl::items;

        let _connection = establish_connection();
        let count = Item::get_count_for_ids(&ids, is_admin);
        if is_admin {
            if l == 1 {
                return (items
                    .filter(schema::items::id.eq_any(ids))
                    .order(schema::items::created.desc())
                    .limit(limit)
                    .offset(offset)
                    .select((
                        schema::items::id,
                        schema::items::slug,
                        schema::items::image.nullable(),
                        schema::items::is_active,
                        schema::items::title,
                        schema::items::description.nullable(),
                    ))
                    .load::<Work>(&_connection)
                    .expect("E."), count);
            }
            else if l == 2 {
                return (items
                    .filter(schema::items::id.eq_any(ids))
                    .order(schema::items::created.desc())
                    .limit(limit)
                    .offset(offset)
                    .select((
                        schema::items::id,
                        schema::items::slug,
                        schema::items::image.nullable(),
                        schema::items::is_active,
                        schema::items::title_en,
                        schema::items::description_en.nullable(),
                    ))
                    .load::<Work>(&_connection)
                    .expect("E."), count);
            }
        }
        else {
            if l == 1 {
                return (items
                    .filter(schema::items::id.eq_any(ids))
                    .filter(schema::items::is_active.eq(true))
                    .order(schema::items::created.desc())
                    .limit(limit)
                    .offset(offset)
                    .select((
                        schema::items::id,
                        schema::items::slug,
                        schema::items::image.nullable(),
                        schema::items::is_active,
                        schema::items::title,
                        schema::items::description.nullable(),
                    ))
                    .load::<Work>(&_connection)
                    .expect("E."), count);
            }
            else if l == 2 {
                return (items
                    .filter(schema::items::id.eq_any(ids))
                    .filter(schema::items::is_active.eq(true))
                    .order(schema::items::created.desc())
                    .limit(limit)
                    .offset(offset)
                    .select((
                        schema::items::id,
                        schema::items::slug,
                        schema::items::image.nullable(),
                        schema::items::is_active,
                        schema::items::title_en,
                        schema::items::description_en.nullable(),
                    ))
                    .load::<Work>(&_connection)
                    .expect("E."), count);
            }
        }
        return (Vec::new(), 0);
    }

    pub fn get_helps_list_for_ids (
        page:     i32,
        limit:    i32,
        ids:      Vec<i32>,
        is_admin: bool,
        l:        u8
    ) -> ((Vec<Help>, usize), i32) {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: (Vec<Help>, usize);

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1; 
            object_list = Item::get_helps_for_ids(limit.into(), step.into(), ids.clone(), is_admin, l);
        }
        else {
            have_next = limit + 1;
            object_list = Item::get_helps_for_ids(limit.into(), 0, ids.clone(), is_admin, l);
        }
        if Item::get_helps_for_ids(1, have_next.into(), ids, is_admin, l).0.len() > 0 {
            next_page_number = page + 1;
        }
        return (object_list, next_page_number);
    } 

    pub fn get_count_for_ids(ids: &Vec<i32>, is_admin: bool) -> usize {
        let _connection = establish_connection();
        if is_admin {
            return schema::items::table
                .filter(schema::items::id.eq_any(ids))
                .select(schema::items::id) 
                .load::<i32>(&_connection)
                .expect("E.")
                .len();
        }
        return schema::items::table
            .filter(schema::items::id.eq_any(ids))
            .filter(schema::items::is_active.eq(true))
            .select(schema::items::id) 
            .load::<i32>(&_connection)
            .expect("E.")
            .len();
    }

    pub fn get_helps_for_ids (
        limit:    i64,
        offset:   i64,
        ids:      Vec<i32>,
        is_admin: bool,
        l:        u8
    ) -> (Vec<Help>, usize) {
        use crate::schema::items::dsl::items;

        let _connection = establish_connection();
        let count = Item::get_count_for_ids(&ids, is_admin);
        if is_admin {
            if l == 1 {
                return (items
                    .filter(schema::items::id.eq_any(ids))
                    .order(schema::items::created.desc())
                    .limit(limit)
                    .offset(offset)
                    .select((
                        schema::items::id,
                        schema::items::is_active,
                        schema::items::title,
                        schema::items::content.nullable(),
                    )) 
                    .load::<Help>(&_connection)
                    .expect("E."), count);
            }
            else if l == 2 {
                return (items
                    .filter(schema::items::id.eq_any(ids))
                    .order(schema::items::created.desc())
                    .limit(limit)
                    .offset(offset)
                    .select((
                        schema::items::id,
                        schema::items::is_active,
                        schema::items::title_en,
                        schema::items::content_en.nullable(),
                    )) 
                    .load::<Help>(&_connection)
                    .expect("E."), count);
            }
        }
        else {
            if l == 1 {
                return (items
                    .filter(schema::items::id.eq_any(ids))
                    .filter(schema::items::is_active.eq(true))
                    .order(schema::items::created.desc())
                    .limit(limit)
                    .offset(offset)
                    .select((
                        schema::items::id,
                        schema::items::is_active,
                        schema::items::title,
                        schema::items::content.nullable(),
                    ))
                    .load::<Help>(&_connection)
                    .expect("E."), count);
            }
            else if l == 2 {
                return (items
                    .filter(schema::items::id.eq_any(ids))
                    .filter(schema::items::is_active.eq(true))
                    .order(schema::items::created.desc())
                    .limit(limit)
                    .offset(offset)
                    .select((
                        schema::items::id,
                        schema::items::is_active,
                        schema::items::title_en,
                        schema::items::content_en.nullable(),
                    ))
                    .load::<Help>(&_connection)
                    .expect("E."), count);
            }
        }
        return (Vec::new(), 0);
    }

    pub fn get_serves_ids(&self) -> Vec<i32> {
        use schema::serve_items::dsl::serve_items;

        let _connection = establish_connection();
        return serve_items
            .filter(schema::serve_items::item_id.eq(&self.id))
            .filter(schema::serve_items::types.eq(self.types))
            .select(schema::serve_items::serve_id)
            .load::<i32>(&_connection)
            .expect("E");
    }
    pub fn get_serves(&self) -> Vec<Serve> {
        use schema::{
            serve_items::dsl::serve_items,
            serve::dsl::serve,
        };

        let _connection = establish_connection();
        let _items = serve_items
            .filter(schema::serve_items::item_id.eq(&self.id))
            .filter(schema::serve_items::types.eq(self.types))
            .select(schema::serve_items::serve_id)
            .load::<i32>(&_connection)
            .expect("E");

        return serve
            .filter(schema::serve::id.eq_any(_items))
            .load::<Serve>(&_connection)
            .expect("E");
    }
    pub fn get_open_tech_categories(&self, types: i16) -> Vec<TechCategories> {
        // получаем открытые тех.категории элемента
        use schema::{
            tech_categories_items::dsl::tech_categories_items,
            tech_categories::dsl::tech_categories,
        };

        let _connection = establish_connection();
        let ids = tech_categories_items
            .filter(schema::tech_categories_items::item_id.eq(&self.id))
            .filter(schema::tech_categories_items::types.eq(types))
            .filter(schema::tech_categories_items::is_active.eq(1))
            .select(schema::tech_categories_items::category_id)
            .load::<i32>(&_connection)
            .expect("E");

        return tech_categories
            .filter(schema::tech_categories::id.eq_any(ids))
            .order(schema::tech_categories::position.desc())
            .load::<TechCategories>(&_connection)
            .expect("E");
    }
    pub fn get_close_tech_categories(&self, types: i16) -> Vec<TechCategories> {
        // получаем закрытые тех.категории элемента
        use schema::{
            tech_categories_items::dsl::tech_categories_items,
            tech_categories::dsl::tech_categories,
        };

        let _connection = establish_connection();
        let ids = tech_categories_items
            .filter(schema::tech_categories_items::item_id.eq(&self.id))
            .filter(schema::tech_categories_items::types.eq(types))
            .filter(schema::tech_categories_items::is_active.eq(2))
            .select(schema::tech_categories_items::category_id)
            .load::<i32>(&_connection)
            .expect("E");

        return tech_categories
            .filter(schema::tech_categories::id.eq_any(ids))
            .order(schema::tech_categories::position.desc())
            .load::<TechCategories>(&_connection)
            .expect("E");
    }
    pub fn get_close_tech_cats_ids(&self, types: i16) -> Vec<i32> {
        use schema::tech_categories_items::dsl::tech_categories_items;

        let _connection = establish_connection();
        return tech_categories_items
            .filter(schema::tech_categories_items::item_id.eq(&self.id))
            .filter(schema::tech_categories_items::types.eq(types))
            .filter(schema::tech_categories_items::is_active.eq(2))
            .select(schema::tech_categories_items::category_id)
            .load::<i32>(&_connection)
            .expect("E");
    }
    pub fn delete(user: User, item_id: i32) -> i16 {
        use crate::schema::{
            items::dsl::items,
            tags_items::dsl::tags_items,
            category::dsl::category,
            files::dsl::files,
        };
        let _connection = establish_connection();
        let _item = Item::get_with_id(item_id);
        diesel::delete(&_item).execute(&_connection).expect("E");
        let _categories = _item.get_categories_obj();
        let _tags = _item.get_tags_obj();
        if user.perm < 60 && _item.user_id != user.id {
            return 0;
        }
        if _item.image.is_some() {
            std::fs::remove_file(_item.image.unwrap()).expect("E");
        }

        let _src_list = files
            .filter(schema::files::item_id.eq(item_id))
            .filter(schema::files::item_types.eq(_item.types))
            .select(schema::files::src)
            .load::<String>(&_connection)
            .expect("E");

        for f in _src_list.iter() {
            std::fs::remove_file(f);
        }

        diesel::delete (
            files
                .filter(schema::files::item_id.eq(item_id))
                .filter(schema::files::item_types.eq(_item.types))
            )
            .execute(&_connection)
            .expect("E");
        diesel::delete (
            tags_items
                .filter(schema::tags_items::item_id.eq(item_id))
                .filter(schema::tags_items::types.eq(_item.types))
            )
            .execute(&_connection)
            .expect("E");
        diesel::delete (
            category
                .filter(schema::category::item_id.eq(item_id))
                .filter(schema::category::types.eq(_item.types))
            ) 
            .execute(&_connection)
            .expect("E");

        for _category in _categories.iter() {
            diesel::update(_category)
                .set(schema::categories::count.eq(_category.count - 1))
                .execute(&_connection)
                .expect("Error.");
        };
        for _tag in _tags.iter() {
            diesel::update(_tag)
                .set(schema::tags::count.eq(_tag.count - 1))
                .execute(&_connection)
                .expect("Error.");
        };
        
        return 1;
    }
}

#[derive(Serialize, Insertable)]
#[table_name="items"] 
pub struct NewItem { 
    pub title:          String, 
    pub title_en:       String,
    pub description:    Option<String>,
    pub description_en: Option<String>,
    pub content:        Option<String>,
    pub content_en:     Option<String>,
    pub link:           Option<String>,
    pub image:          Option<String>,
    pub is_active:      bool,
    pub price:          i32,
    pub user_id:        i32,
    pub created:        chrono::NaiveDateTime,
    pub position:       i16,
    pub view:           i32,
    pub height:         f64,
    pub seconds:        i32,
    pub price_acc:      Option<i32>,
    pub types:          i16,
    pub slug:           String,
}

impl NewItem {
    pub fn create (
        title:          String,
        title_en:       String,
        description:    Option<String>,
        description_en: Option<String>,
        link:           Option<String>,
        image:          Option<String>,
        user_id:        i32,
        position:       i16,
        types:          i16,
        slug:           String
    ) -> Self {
        use chrono::Duration;

        NewItem {
            title:          title,
            title_en:       title_en,
            description:    description,
            description_en: description_en,
            content:        None,
            content_en:     None,
            link:           link,
            image:          image,
            is_active:      false,
            price:          0,
            user_id:        user_id,
            created:        chrono::Local::now().naive_utc() + Duration::hours(3),
            position:       position,
            view:           0,
            height:         0.0,
            seconds:        0,
            price_acc:      None,
            types:          types,
            slug:           slug,
        }
    }
}

#[derive(Queryable, Serialize, Deserialize, AsChangeset, Debug)]
#[table_name="items"]
pub struct EditItem {
    pub title:          String,
    pub title_en:       String,
    pub description:    Option<String>,
    pub description_en: Option<String>,
    pub link:           Option<String>,
    pub image:          Option<String>,
    pub position:       i16,
    pub slug:           String,
}


///////////
// types:
// 1. блог
// 2. услуга
// 3. товар
// 4. wiki
// 5. работа
// 6. помощь
// 7. заказ
// 8. веб-сервис
// 9. язык / технология
// 10. опция
#[derive(Identifiable, Queryable)]
#[table_name="category"]
pub struct Category {
    pub id:          i32,
    pub category_id: i32,
    pub item_id:     i32,
    pub types:       i16,
}

#[derive(Insertable)]
#[table_name="category"]
pub struct NewCategory {
    pub category_id: i32,
    pub item_id:     i32,
    pub types:       i16,
}

#[derive(Debug, Serialize, Queryable, Identifiable)]
pub struct ItemComment {
    pub id:        i32,
    pub comment:   String,
    pub blog_id:   i32,
    pub user_id:   i32,
    pub parent_id: Option<i32>,
    pub created:   chrono::NaiveDateTime,
}

#[derive(Serialize, Insertable)]
#[table_name="item_comments"]
pub struct NewItemComment {
    pub comment:   String,
    pub item_id:   i32,
    pub user_id:   i32,
    pub parent_id: Option<i32>,
    pub created:   chrono::NaiveDateTime,
}

impl NewItemComment {
    pub fn new (comment: String, item_id: i32,
        user_id: i32, parent_id: Option<i32>) -> Self {
        use chrono::Duration;

        NewItemComment {
            comment:   comment,
            item_id:   item_id,
            user_id:   user_id,
            parent_id: parent_id,
            created:   chrono::Local::now().naive_utc() + Duration::hours(3),
        }
    }
}
