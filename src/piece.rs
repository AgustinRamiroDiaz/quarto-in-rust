use super::QUATRO;

use super::N_PROPERTIES;

pub(crate) type Piece = [bool; N_PROPERTIES];

pub(crate) fn check_match(pieces: Vec<&Piece>) -> bool {
    for property in 0..QUATRO {
        let properties = pieces
            .iter()
            .map(|piece| piece[property])
            .collect::<Vec<bool>>();

        let first_property = properties[0];
        if properties
            .iter()
            .all(|property| property == &first_property)
        {
            return true;
        }
    }

    false
}
