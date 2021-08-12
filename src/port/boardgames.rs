cfg_if::cfg_if! {
  if #[cfg(test)] {
    use mockall_double::double;
    #[double]
    use crate::domain::boardgames::Boardgames;
  } else {
    use crate::domain::boardgames::Boardgames;
  }
}

use mockall::*;

#[automock]
pub trait BoardgamesPort {
    fn find_all(&self) -> Result<Boardgames, String>;
}
