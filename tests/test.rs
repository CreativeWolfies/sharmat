// The ~tests~ (the thing that tests functions & stuff)

use sharmat::player::*;
use sharmat::rule::*;

#[cfg(test)]
mod board;

#[cfg(test)]
mod movement;

// Engine basic tests

#[cfg(test)]
mod player {
    use super::*;

    #[test]
    fn player_create() {
        let _player = Player::new(PlayerColor::White /* UNKNOWN */);
    }
}

#[cfg(test)]
mod rule {
    use super::*;

    #[test]
    fn rule_create() {
        let _rule = Rule::new(/* UNKNOWN */);
    }
}
