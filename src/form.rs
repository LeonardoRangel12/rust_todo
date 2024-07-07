use rocket::form::Form;

#[derive(FromForm)]
pub struct Task {
    pub description: String,
}