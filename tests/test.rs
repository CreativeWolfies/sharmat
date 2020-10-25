// The ~tests~ (the thing that tests functions & stuff)

use sharmat::rule::*;
use sharmat::player::*;

#[cfg(test)]
mod board;

// Engine basic tests

#[cfg(test)]
mod player {
    use super::*;

    #[test]
    fn player_create() {
        let _player = Player::new(/* UNKNOWN */);
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
