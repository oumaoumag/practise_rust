pub enum AccessLevel {
    Guest,
    Normal,
    Admin,
}

pub struct User {
    name: String,
    access_level: AccessLevel,
}


impl User {
    pub fn new(name: String, level: AccessLevel) -> User {
        User {
            name,
            access_level: level,
        }
    }

pub fn send_name(&self) -> Option<&str> {
    match self.access_level {
        AccessLevel::Guest => None,
        _ => Some(&self.name)
    }
}

pub fn check_user_name(user: &User) -> (bool, &str) {
    match user.send_name() {
        Some(name) => (true, name),
        None => (false, "ERROR: user is guest")
    }
}