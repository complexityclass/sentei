use crate::player::Player;
use crate::point::Point;
use std::collections::HashSet;
use std::fmt;

#[derive(Clone)]
pub struct StonesString {
    pub color: Player,
    pub stones: HashSet<Point>,
    pub liberties: HashSet<Point>,
}

impl StonesString {
    pub(crate) fn new(color: Player, stones: &[Point], liberties: &[Point]) -> Self {
        StonesString {
            color,
            stones: stones.iter().cloned().collect(),
            liberties: liberties.iter().cloned().collect(),
        }
    }

    pub(crate) fn remove_liberty(&mut self, point: &Point) {
        self.liberties.remove(point);
    }

    pub(crate) fn add_liberty(&mut self, point: &Point) {
        self.liberties.insert(point.clone());
    }

    pub(crate) fn merge(&self, other: &StonesString) -> Self {
        assert_eq!(self.color, other.color);

        let combine = |a: &HashSet<_>, b: &HashSet<_>| {
            let mut combined = a.clone();
            combined.extend(b.clone());
            combined
        };

        let combined_stones = combine(&self.stones, &other.stones);

        let mut combined_liberties = combine(&self.liberties, &other.liberties);
        combined_liberties = combined_liberties
            .difference(&combined_stones)
            .cloned()
            .collect();

        StonesString {
            color: self.color.clone(),
            stones: combined_stones,
            liberties: combined_liberties,
        }
    }

    pub(crate) fn liberties_count(&self) -> usize {
        self.liberties.len()
    }
}

impl PartialEq for StonesString {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
            && self.stones == other.stones
            && self.liberties == other.liberties
    }
}

impl Eq for StonesString {}

impl fmt::Debug for StonesString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StonesString")
            .field("color", &self.color)
            .field("stones", &self.stones)
            .field("liberties", &self.liberties)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stones_string_new() {
        let stones = [Point::new(0, 0), Point::new(0, 1)];
        let liberties = [Point::new(1, 0), Point::new(1, 1)];
        let stones_string = StonesString::new(Player::White, &stones, &liberties);
        assert_eq!(
            stones_string,
            StonesString {
                color: Player::White,
                stones: stones.iter().cloned().collect(),
                liberties: liberties.iter().cloned().collect(),
            }
        );
    }

    #[test]
    fn test_stones_string_remove_liberty() {
        let stones = [Point::new(0, 0), Point::new(0, 1)];
        let liberties = [Point::new(1, 0), Point::new(1, 1)];
        let mut stones_string = StonesString::new(Player::White, &stones, &liberties);
        stones_string.remove_liberty(&Point::new(1, 0));
        assert_eq!(stones_string.liberties.len(), 1);
    }

    #[test]
    fn test_stones_string_add_liberty() {
        let stones = [Point::new(0, 0), Point::new(0, 1)];
        let liberties = [Point::new(1, 0), Point::new(1, 1)];
        let mut stones_string = StonesString::new(Player::White, &stones, &liberties);
        stones_string.add_liberty(&Point::new(2, 0));
        assert_eq!(stones_string.liberties.len(), 3);
    }

    #[test]
    fn test_stones_string_merge() {
        let stones = [Point::new(0, 0), Point::new(0, 1)];
        let liberties = [Point::new(1, 0), Point::new(1, 1)];
        let stones_string1 = StonesString::new(Player::White, &stones, &liberties);

        let stones2 = [Point::new(1, 0), Point::new(1, 1)];
        let liberties2 = [Point::new(2, 0), Point::new(2, 1)];
        let stones_string2 = StonesString::new(Player::White, &stones2, &liberties2);

        let merged = stones_string1.merge(&stones_string2);
        assert_eq!(
            merged,
            StonesString::new(
                Player::White,
                &[
                    Point::new(0, 0),
                    Point::new(0, 1),
                    Point::new(1, 0),
                    Point::new(1, 1)
                ],
                &[Point::new(2, 0), Point::new(2, 1)]
            )
        );
    }

    #[test]
    fn test_stones_string_liberties_count() {
        let stones = [Point::new(0, 0), Point::new(0, 1)];
        let liberties = [Point::new(1, 0), Point::new(1, 1)];
        let stones_string = StonesString::new(Player::White, &stones, &liberties);
        assert_eq!(stones_string.liberties_count(), 2);
    }
}
