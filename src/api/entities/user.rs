pub enum AccessLevel {
    Level1,
    Level2,
    Level3,
    Admin
}

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize)]
pub struct User {
    username: String,
    name: String,
    password: String,
    level: AccessLevel
}