/// This is used to show the users profile
#[derive(Derialize, Serialize)]
struct user_profile {
    pub email: String,
    pub username: String,

    pub avatar: String,
    pub avatar_const: bool,

    // under_organization:
    // product:
}

