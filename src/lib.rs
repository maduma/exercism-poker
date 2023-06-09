use std::cmp::{PartialOrd, Ordering};
use std::collections::{BTreeSet, HashMap};
use std::fmt;

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
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum CardValue {
    One, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace,
}

impl CardValue {
    fn from_str(s: &str) -> Result<CardValue, ParseError> {
        const CARDVALUES: [CardValue; 14] = [
            CardValue::One, CardValue::Two, CardValue::Three, CardValue::Four, CardValue::Five,
            CardValue::Six, CardValue::Seven, CardValue::Eight, CardValue::Nine, CardValue::Ten,
            CardValue::Jack, CardValue::Queen, CardValue::King, CardValue::Ace,
        ];
        match s.parse::<usize>() {
            Ok(i) => {
                if i >=2 && i <=10 { Ok(CARDVALUES[i - 1]) } else { Err(ParseError) }
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
    OnePair,
    TwoPair,
    ThreeOFAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

#[derive(PartialEq)]
struct Hand<'a> {
    cards: BTreeSet<Card>,
    src: &'a str,
    rank: Rank,
}

impl fmt::Debug for Hand<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} ({})", self.rank, self.src)
    }
}

#[derive(Debug)]
struct ParseHandError<'a>(&'a str);


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Tuple {
    Single,
    Pair,
    Triad,
    Quad,
}

fn frequencies(values: Vec<CardValue>) -> HashMap<Tuple, Vec<CardValue>> {
    let mut h1 = HashMap::<CardValue, u8>::new();
    let mut h2: HashMap<Tuple, Vec<CardValue>> = HashMap::new();
    for v in values {
        h1.entry(v).and_modify(|count| *count += 1).or_insert(1);
    }
    for (k, count) in h1 {
        h2.entry(match count {
            1 => Tuple::Single,
            2 => Tuple::Pair,
            3 => Tuple::Triad,
            4 => Tuple::Quad,
            _ => panic!("More that 4 fo the same cards!"),
        }).or_insert(Vec::new()).push(k);
    }
    h2
}

fn is_flush(cards: &BTreeSet<Card>) -> bool {
    cards.iter().zip(cards.iter().skip(1)).all(|(c1, c2)| c1.suit == c2.suit)
}

fn is_straight(cards: &BTreeSet<Card>) -> bool {
    cards.iter().zip(cards.iter().skip(1)).all(|(c1, c2)| c1.is_adjacent(c2))
}

fn is_four_of_a_kind(cards: &BTreeSet<Card>) -> bool {
    let values = cards.iter().map(|c| c.value).collect::<Vec<CardValue>>();
    let freq = frequencies(values);
    freq.contains_key(&Tuple::Quad)
}

fn have_one_pair(cards: &BTreeSet<Card>) -> bool {
    let values = cards.iter().map(|c| c.value).collect::<Vec<CardValue>>();
    let freq = frequencies(values);
    freq.contains_key(&Tuple::Pair)
}

fn have_two_pair(cards: &BTreeSet<Card>) -> bool {
    let values = cards.iter().map(|c| c.value).collect::<Vec<CardValue>>();
    let freq = frequencies(values);
    freq.contains_key(&Tuple::Pair) && freq.get(&Tuple::Pair).unwrap().len() == 2
}

fn have_three_of_a_kind(cards: &BTreeSet<Card>) -> bool {
    let values = cards.iter().map(|c| c.value).collect::<Vec<CardValue>>();
    let freq = frequencies(values);
    freq.contains_key(&Tuple::Triad)
}

fn is_full_house(cards: &BTreeSet<Card>) -> bool {
    have_three_of_a_kind(cards) && have_one_pair(cards)
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
        } else if is_four_of_a_kind(&cards) {
            Ok(Hand {cards: cards, src: s, rank: Rank::FourOfAKind})
        } else if is_full_house(&cards) {
            Ok(Hand {cards: cards, src: s, rank: Rank::FullHouse})
        } else if is_flush(&cards) {
            Ok(Hand {cards: cards, src: s, rank: Rank::Flush})
        } else if is_straight(&cards) {
            Ok(Hand {cards: cards, src: s, rank: Rank::Straight})
        } else if have_three_of_a_kind(&cards) {
            Ok(Hand {cards: cards, src: s, rank: Rank::ThreeOFAKind})
        } else if have_two_pair(&cards) {
            Ok(Hand {cards: cards, src: s, rank: Rank::TwoPair})
        } else if have_one_pair(&cards) {
            Ok(Hand {cards: cards, src: s, rank: Rank::OnePair})
        } else {
            Ok(Hand {cards: cards, src: s, rank: Rank::HighCard})
        }
    }
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.rank != other.rank {
            // println!("self, {:?}", self);
            // println!("other, {:?}", other);
            let toto = self.rank.partial_cmp(&other.rank);
            // println!("partial_cmp, {:?}", toto);
            toto
        } else {
            match self.rank {
                Rank::Straight | Rank::StraightFlush => self.cards.first().unwrap().partial_cmp(other.cards.first().unwrap()),
                _ => Some(Ordering::Equal),
            }   
        }
    }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    // println!("{:?}", Hand::from_str("5C 6C 7C 8C 9C"));
    // println!("{:?}", Hand::from_str("JC JD JH JS 9S"));
    // println!("{:?}", Hand::from_str("5C 5D 7H 7D 5S"));
    // println!("{:?}", Hand::from_str("5C 6C 8C 10C JC"));
    // println!("{:?}", Hand::from_str("5C 6D 7H 8D 9S"));
    // println!("{:?}", Hand::from_str("5C 5D 7H 6D 5S"));
    // println!("{:?}", Hand::from_str("5C 5D 7H 7D AS"));
    // println!("{:?}", Hand::from_str("5C 4D 7H AD AS"));
    // println!("{:?}", Hand::from_str("5C 4D 7H AD JS"));
    
    let mut hands = hands.iter().map(|h| Hand::from_str(h).unwrap()).collect::<Vec<Hand>>();
    hands.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Less));
    hands.reverse();
    // println!("hands {:?}", hands);
    if hands.len() > 1 {
        let hand = &hands[0];
        hands.iter().filter(|&h| h.partial_cmp(hand).unwrap() == Ordering::Equal).map(|h| h.src).collect()
    } else {
        hands.iter().map(|h| h.src).collect()
    }
}
