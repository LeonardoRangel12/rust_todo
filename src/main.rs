use rocket::form::Form;
#[macro_use] extern crate rocket;

mod file_io;
mod form;
use form::Task;

#[get("/<usr>")]
fn get_todo(usr:&str) -> String {
    
    match file_io::read_file(&usr){
        Ok(resp) => format!("{}", resp),
        Err(_) => format!("Error reading file")
    }
    
}

#[put("/<usr>/<index>", data = "<task>")]
fn update_todo(usr:&str, index:usize, task:Form<Task>) -> String{
    println!("{}", index);
    match file_io::update_line(&usr, &index, &task.description){
        Ok(_) => format!("Updated!"),
        Err(_) => format!("Error updating line..."),
    }

    
}


#[delete("/<usr>/<index>")]
fn delete_todo(usr:&str, index:usize) -> String{
    println!("{}", index);
    match file_io::delete_line(&usr, &index){
        Ok(_) => format!("Updated!"),
        Err(_) => format!("Error updating line..."),
    }
}

#[post("/<usr>", data = "<task>")]
fn add_todo(usr:&str, task:Form<Task>) -> String{
    if task.description.is_empty() {
        return format!("Not allowed empty")
    }
    match file_io::append_to_end_of_file(usr, &task.description){
        Ok(_) => format!("{}", task.description),
        Err(e) => format!("{}", e),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_todo, add_todo, update_todo, delete_todo])
}