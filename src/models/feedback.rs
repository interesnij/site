use crate::schema::feedbacks;
use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;


#[derive(Debug ,Queryable, Serialize, Identifiable)]
pub struct Feedback {
    pub id:       i32,
    pub username: String,
    pub email:    String,
    pub message:  String,
}
impl Feedback {
    pub fn get_all() -> Vec<Feedback> {
        let _connection = establish_connection();
        return schema::feedbacks::table
            .load::<Feedback>(&_connection)
            .expect("E");
    }
    pub fn create(form: crate::utils::feedback_form) -> i16 {
        let _connection = establish_connection();
        let new_feedback = NewFeedback {
            username: form.username.clone(),
            email:    form.email.clone(),
            message:  form.message
        }; 
        let _new_feedback = diesel::insert_into(schema::feedbacks::table)
            .values(&new_feedback)
            .execute(&_connection)
            .expect("E.");
        return 1;
    }
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="feedbacks"]
pub struct NewFeedback {
    pub username: String,
    pub email:    String,
    pub message:  String,
}
