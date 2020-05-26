use crate::api::entities::user::User;
use crate::utils::errors::Error;

pub trait Repository {
    fn add_user(user: &User) -> Result<(), Error>;
    fn get_users() -> Result<Vec<User>, Error>;
    fn remove_user(username: String) -> Result<(), Error>;
    fn demote_user(username: String) -> Result<(), Error>;
    fn promote_user(username: String) -> Result<(), Error>;
}