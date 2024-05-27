use crate::models::user::User;

pub fn get_user_details(user: &User) -> String {
    format!(
        "Name: {}, Age: {}, Is Alive: {}",
        user.name,
        user.age,
        if user.is_alive { "Yes" } else { "No" }
    )
}

pub fn check_user(user: &User) -> &'static str {
    match (user.age, user.is_alive, user.name.as_str()) {
        (21, true, "Damla") => "This is Damla",
        _ => "Not Damla",
    }
}
