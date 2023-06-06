/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
/// 
/// 
use std::cmp::{PartialOrd, Ordering};

#[derive(Debug)]
#[derive(PartialEq)]
#[allow(dead_code)]
enum Hand<'a> {
    StraightFlush(&'a str),
    FourOfAKind(&'a str),
    FullHouse(&'a str),
    Flush(&'a str),
    Straight(&'a str),
    ThreeOFAKind(&'a str),
    TwoPair(&'a str),
    OnePair(&'a str),
    HighCard(&'a str),
}

enum CardSuit {
    Club, Diamond, Heart, Spade,
}

enum CardValue {
    Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten,
    Jack, Queen, King, Ace,
}
struct ParseCardValueError;

impl CardValue {
    fn from_str(s: &str) -> Result<CardValue, ParseCardValueError> {
        Ok(CardValue::Ace)
    }
}

struct Card {
    suit: CardSuit,
    value: CardValue,
}

#[derive(Debug)]
struct ParseHandError;

impl Hand<'_> {
    fn from_str(s: &str) -> Result<Hand, ParseHandError> {
        let cards: Vec<&str> = s.split(" ").collect();
        println!("{:?}", cards);
        if cards.len() != 5 { return Err(ParseHandError) }
        Ok(Hand::FourOfAKind(s))
    }
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        Some(Ordering::Equal)
        //five of a kind  - 3C (club), 3D (diamond), 3H (heart), 3S (spade), J (jocker)
        //straight flush  - 3C 4C 5C 6C 7C
        //four of a kind
        //full house      - 6C 6D 6H KD KS
        //flush           - 2C 7C 8C JC QC
        //straight        - 3C 4D 5C 6S 7H
        //Three of a kind
        //two pair
        //one pair    
        //high card

    }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    println!("{:?}", Hand::from_str("hello"));
    unimplemented!("Out of {hands:?}, which hand wins?")
}
