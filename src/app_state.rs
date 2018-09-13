

pub struct User {
    user_name: String,
}

pub enum AuthState {
    Off,
    In(User),
}

pub struct AppState {
    pub auth_state: AuthState,
}