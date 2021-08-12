use mockall::*;

#[derive(Debug, Clone)]
pub struct Boardgames(pub Vec<Boardgame>);

#[automock]
impl Boardgames {
    pub fn sort(&self) -> Self {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Boardgame {
    pub name: String,
    pub price: i32,
}
