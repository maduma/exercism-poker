use std::cmp::{PartialOrd, Ordering};
use std::collections::BTreeSet;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum CardSuit {
    Club, Diamond, Heart, Spade,
}

#[derive(Debug)]
struct ParseError;

impl CardSuit {
    fn from_str(s: &str) -> Result<CardSuit, ParseError> {
        match s {
            "C" => Ok(CardSuit::Club),
            "D" => Ok(CardSuit::Diamond),
            "H" => Ok(CardSuit::Heart),
            "S" => Ok(CardSuit::Spade),
            _ => Err(ParseError),
        }
    }
}

#[derive(Debug)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum CardValue {
    Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace,
}

impl CardValue {
    fn from_str(s: &str) -> Result<CardValue, ParseError> {
        const CARDVALUES: [CardValue; 13] = [
            CardValue::Two, CardValue::Three, CardValue::Four, CardValue::Five,
            CardValue::Six, CardValue::Seven, CardValue::Eight, CardValue::Nine, CardValue::Ten,
            CardValue::Jack, CardValue::Queen, CardValue::King, CardValue::Ace,
        ];
        match s.parse::<usize>() {
            Ok(i) => {
                if i >=2 && i <=10 { Ok(CARDVALUES[i - 2]) } else { Err(ParseError) }
            },
            Err(_) => match s {
                "J" => Ok(CardValue::Jack),
                "Q" => Ok(CardValue::Queen),
                "K" => Ok(CardValue::King),
                "A" => Ok(CardValue::Ace),
                _ => Err(ParseError),
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Card {
    value: CardValue,
    suit: CardSuit,
}

impl Card {
    fn from_str(s: &str) -> Result<Card, ParseError> {
        let value: Result<CardValue, ParseError> = CardValue::from_str(&s[..s.len()-1]);
        let suit = CardSuit::from_str(&s[s.len()-1..]);
        match (value, suit) {
            (Ok(v), Ok(s)) => Ok(Card {suit: s, value: v}),
            _ => Err(ParseError),
        }
    }
    fn is_adjacent(self: &Self, other: &Self) -> bool {
        (self.value as i8 - other.value as i8).abs() == 1
    }
}


#[allow(dead_code)]
#[derive(Debug, PartialEq, PartialOrd)]
enum Rank {
    HighCard,
    OnePair(u8, u8, u8, u8),
    TwoPair(u8, u8, u8),
    ThreeOFAKind(u8, u8, u8),
    Straight,
    Flush,
    FullHouse(u8, u8),
    FourOfAKind(u8, u8),
    StraightFlush,
}

#[derive(Debug, PartialEq)]
struct Hand<'a> {
    cards: BTreeSet<Card>,
    src: &'a str,
    rank: Rank,
}

#[derive(Debug)]
struct ParseHandError<'a>(&'a str);

fn is_flush(cards: &BTreeSet<Card>) -> bool {
    let suit = cards.first().unwrap().suit;
    cards.iter().all(|c| c.suit == suit)
}

fn is_straight(cards: &BTreeSet<Card>) -> bool {
    cards.iter().zip(cards.iter().skip(1)).all(|(c1, c2)| c1.is_adjacent(c2))
}

impl Hand<'_> {
    fn from_str(s: &str) -> Result<Hand, ParseHandError> {
        let mut cards =  BTreeSet::<Card>::new();
        let cards_str = s.split(" ").collect::<Vec<_>>();
        if cards_str.len() != 5 {
            let err =  ParseHandError(s);
            println!("{:?}", err);
            return Err(err);
        }
        for card in cards_str {
            match Card::from_str(card) {
                Ok(c) => if !cards.insert(c) {
                    println!("Duplicate cards {:?}", card);
                    return Err(ParseHandError(s))
                },
                Err(e) => {
                    println!("{:?}", e);
                    return Err(ParseHandError(s));
                }
            }
        }
        if is_straight(&cards) && is_flush(&cards) {
            Ok(Hand {cards: cards, src: s, rank: Rank::StraightFlush})
        } else if is_flush(&cards) {
            Ok(Hand {cards: cards, src: s, rank: Rank::Flush})
        } else if is_straight(&cards) {
            Ok(Hand {cards: cards, src: s, rank: Rank::Straight})
        } else {
            Ok(Hand {cards: cards, src: s, rank: Rank::HighCard})
        }
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
    println!("{:?}", Hand::from_str("5C 6D 7H 8D 9S"));
    println!("{:?}", Hand::from_str("5C 6C 7C 8C 9C"));
    println!("{:?}", Hand::from_str("KC 6D 2H 3D QS"));
    println!("{:?}", Hand::from_str("KC 6C 2C 3C QC"));
    println!("{:?}", Rank::HighCard == Rank::HighCard);
    println!("{:?}", (2, 0, 5) > (2, 1, 3));
    unimplemented!("Out of {hands:?}, which hand wins?")
}
