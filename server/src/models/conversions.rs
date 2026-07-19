use crate::models::user_model::User;
use shared::models::auth_models::UserDto;

// Use &User if user is to be used again after user.into()
impl From<User> for UserDto {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            display_name: user.display_name,
        }
    }
}
