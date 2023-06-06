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

#[derive(Debug)]
enum CardSuit {
    Club, Diamond, Heart, Spade,
}

#[derive(Debug)]
struct ParseCardSuitError;

impl CardSuit {
    fn from_str(s: &str) -> Result<CardSuit, ParseCardSuitError> {
        match s {
            "C" => Ok(CardSuit::Club),
            "D" => Ok(CardSuit::Diamond),
            "H" => Ok(CardSuit::Heart),
            "S" => Ok(CardSuit::Spade),
            _ => Err(ParseCardSuitError),
        }
    }
}

#[derive(Debug)]
#[derive(Copy, Clone)]
enum CardValue {
    Two = 0, Three = 1, Four = 2, Five = 3, Six = 4, Seven = 5, Eight = 6, Nine = 7, Ten = 8,
    Jack, Queen, King, Ace,
}

#[derive(Debug)]
struct ParseCardValueError;

impl CardValue {
    fn from_str(s: &str) -> Result<CardValue, ParseCardValueError> {
        const CARDVALUES: [CardValue; 13] = [
            CardValue::Two, CardValue::Three, CardValue::Four, CardValue::Five,
            CardValue::Six, CardValue::Seven, CardValue::Eight, CardValue::Nine, CardValue::Ten,
            CardValue::Jack, CardValue::Queen, CardValue::King, CardValue::Ace,
        ];
        match s.parse::<usize>() {
            Ok(i) => {
                if i >=2 && i <=10 { Ok(CARDVALUES[i - 2]) } else { Err(ParseCardValueError) }
            },
            Err(_) => match s {
                "J" => Ok(CardValue::Jack),
                "Q" => Ok(CardValue::Queen),
                "K" => Ok(CardValue::King),
                "A" => Ok(CardValue::Ace),
                _ => Err(ParseCardValueError),
            }
        }
    }
}

#[derive(Debug)]
struct Card {
    suit: CardSuit,
    value: CardValue,
}

#[derive(Debug)]
struct ParseHandError<'a>(&'a str);

impl Hand<'_> {
    fn from_str(s: &str) -> Result<Hand, ParseHandError> {
        let val = CardValue::from_str(&s[..s.len()-1]);
        let suit = CardSuit::from_str(&s[s.len()-1..]);
        println!("{:?} {:?}", val, suit);
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
    println!("{:?}", Hand::from_str("4H"));
    unimplemented!("Out of {hands:?}, which hand wins?")
}
