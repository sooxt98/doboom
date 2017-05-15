use schema::products;

#[derive(Queryable, Identifiable, Debug)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub real_name: String,
    pub description: String,
    pub body: String,
    pub logo: String,
    pub owner: String,
    pub published: bool,
}