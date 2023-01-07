use std::cmp::Ordering;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut hands: Vec<PokerHand> = hands.iter().map(|&hand| PokerHand::from(hand)).collect();
    hands.sort_by(|left, right| left.partial_cmp(right).unwrap_or(Ordering::Equal));
    Vec::new()
}

enum PokerHand {
    FiveOfAKind(),
    StraightFlush(),
    FourOfAKind(),
    FullHouse(),
    Flush(),
    Straight(),
    ThreeOfAKind(),
    TwoPair(),
    OnePair(),
    HighCard(),
}

impl From<&str> for PokerHand {
    fn from(_: &str) -> Self {
        todo!()
    }
}

impl PartialEq for PokerHand {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl PartialOrd for PokerHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        todo!()
    }
}
