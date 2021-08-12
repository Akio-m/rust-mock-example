cfg_if::cfg_if! {
  if #[cfg(test)] {
    use mockall_double::double;
    #[double]
    use crate::domain::boardgames::Boardgames;
  } else {
    use crate::domain::boardgames::Boardgames;
  }
}
use crate::port::boardgames::BoardgamesPort;

pub struct GetBoardgameUsecase<T: BoardgamesPort> {
    pub port: T,
}

impl<T: BoardgamesPort> GetBoardgameUsecase<T> {
    pub fn execute(&self) -> Result<Boardgames, String> {
        let boardgames = self.port.find_all().unwrap();
        Ok(boardgames.sort())
    }
}

#[cfg(test)]
mod tests {
    // use std::rc::Rc;
    use super::*;
    use crate::port::boardgames::*;

    #[test]
    fn test_execute() {
        // let rc_expected = Rc::new(Boardgames::new());
        // let expected = rc_expected.clone();
        // let expected = Boardgames::new();

        let mut mock_boardgames_port = MockBoardgamesPort::new();
        mock_boardgames_port
            .expect_find_all()
            .returning(|| {
              let mut mock_boardgames = Boardgames::new();
              mock_boardgames.expect_sort().returning(|| Boardgames::new());
              Ok(mock_boardgames)
            });

        let target = GetBoardgameUsecase {
            port: mock_boardgames_port,
        };

        let actual = target.execute().unwrap();
        print_typename(actual);
        // assert_eq!(actual, expected);
    }

    fn print_typename<T>(_: T) {
      println!("{}", std::any::type_name::<T>());
    }
}
