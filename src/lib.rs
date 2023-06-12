use std::cmp::{PartialOrd, Ordering};
use std::collections::{BTreeSet, HashMap, BTreeMap};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum CardSuit {
    Club, Diamond, Heart, Spade,
}

impl CardSuit {
    fn from_str(s: &str) -> CardSuit {
        match s {
            "C" => CardSuit::Club,
            "D" => CardSuit::Diamond,
            "H" => CardSuit::Heart,
            "S" => CardSuit::Spade,
            _ => panic!("Bad suit: {}", s),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum CardValue { // Ace may have a value of One
    One, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace,
}


fn number_card(i: usize) -> Result<CardValue, usize> {
    const CARDVALUES: [CardValue; 14] = [
        CardValue::One, CardValue::Two, CardValue::Three, CardValue::Four, CardValue::Five,
        CardValue::Six, CardValue::Seven, CardValue::Eight, CardValue::Nine, CardValue::Ten,
        CardValue::Jack, CardValue::Queen, CardValue::King, CardValue::Ace,
    ];
    
    if (2..=10).contains(&i) {
        Ok(CARDVALUES[i - 1])
    } else { 
        Err(i)
    }
}

fn face_card(s: &str) -> Result<CardValue, ()> {
    match s {
        "J" => Ok(CardValue::Jack),
        "Q" => Ok(CardValue::Queen),
        "K" => Ok(CardValue::King),
        "A" => Ok(CardValue::Ace),
        _ => Err(()),
    }
}

impl CardValue {
    fn from_str(s: &str) -> CardValue {
        match s.parse::<usize>() {
            Ok(i) => number_card(i).unwrap(),
            Err(_) => face_card(s).unwrap(),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Card {
    value: CardValue,
    suit: CardSuit,
}

impl Card {
    fn from_str(s: &str) -> Card {
        let suit = &s[s.len()-1..];
        let value = &s[..s.len()-1];
        Card {suit: CardSuit::from_str(suit), value: CardValue::from_str(value)}
    }
    fn is_adjacent(&self, other: &Self) -> bool {
        (self.value as i8 - other.value as i8).abs() == 1
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(PartialEq, Eq)]
struct Hand<'a> {
    cards: BTreeSet<Card>,
    src: &'a str,
    rank: Rank,
    freq: BTreeMap<Tuple, Vec<CardValue>>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Tuple {
    Quad,
    Triad,
    Pair,
    Single,
}

fn frequencies(values: Vec<CardValue>) -> BTreeMap<Tuple, Vec<CardValue>> {
    let mut h1 = HashMap::<CardValue, u8>::new();
    let mut h2: HashMap<Tuple, BTreeSet<CardValue>> = HashMap::new();
    for v in values {
        *h1.entry(v).or_insert(0) += 1;
    }
    for (k, count) in h1 {
        h2.entry(match count {
            1 => Tuple::Single,
            2 => Tuple::Pair,
            3 => Tuple::Triad,
            4 => Tuple::Quad,
            _ => panic!("More that 4 cards with the same value!"),
        }).or_insert(BTreeSet::new()).insert(k);
    }
    h2.into_iter()
        .map(|(k, v)| (k, v.into_iter().rev().collect::<Vec<_>>()))
        .collect::<BTreeMap<Tuple, Vec<CardValue>>>()
}


fn ace_as_one(cards: &BTreeSet<Card>) -> BTreeSet<Card> {
    cards.iter()
        .map(|&c| if c.value == CardValue::Ace { Card { value: CardValue::One, ..c } } else { c }).collect::<BTreeSet<_>>()
}

fn is_straight(cards: &mut BTreeSet<Card>) -> bool {
    cards.iter().zip(cards.iter().skip(1)).all(|(c1, c2)| c1.is_adjacent(c2)) || {
        let mut alt_cards = ace_as_one(cards);
        if alt_cards.iter().zip(alt_cards.iter().skip(1)).all(|(c1, c2)| c1.is_adjacent(c2)) {
            cards.clear();
            cards.append(&mut alt_cards);
            true
        } else {
            false
        }
    }
}

fn is_flush(cards: &BTreeSet<Card>) -> bool {
    cards.iter().zip(cards.iter().skip(1)).all(|(c1, c2)| c1.suit == c2.suit)
}

fn have_two_pair(freq: &BTreeMap<Tuple, Vec<CardValue>>) -> bool {
    freq.contains_key(&Tuple::Pair) && freq.get(&Tuple::Pair).unwrap().len() == 2
}

impl Hand<'_> {
    fn from_str(src: &str) -> Hand {
        let cards = src.split(' ').collect::<Vec<_>>();
        if cards.len() != 5 { panic!("Cannot find 5 cards in the hand: {}", src) }
        let mut cards = cards.iter().map(|&s| Card::from_str(s)).collect::<BTreeSet<Card>>();
        let freq = frequencies(cards.iter().map(|c| c.value).collect::<Vec<_>>());
        let rank = {
            if is_straight(&mut cards) && is_flush(&cards) { Rank::StraightFlush }
            else if freq.contains_key(&Tuple::Quad) { Rank::FourOfAKind }
            else if freq.contains_key(&Tuple::Triad) && freq.contains_key(&Tuple::Pair) { Rank::FullHouse }
            else if is_flush(&cards) { Rank::Flush }
            else if is_straight(&mut cards) { Rank::Straight }
            else if freq.contains_key(&Tuple::Triad) { Rank::ThreeOFAKind }
            else if have_two_pair(&freq) { Rank::TwoPair }
            else if freq.contains_key(&Tuple::Pair) { Rank::OnePair }
            else { Rank::HighCard }
        };
        Hand {cards, src, rank, freq}
    }
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.rank != other.rank {
           self.rank.partial_cmp(&other.rank)
        } else {
            match self.rank {
                Rank::Straight | Rank::StraightFlush => self.cards.first().unwrap().partial_cmp(other.cards.first().unwrap()),
                Rank::HighCard | Rank::Flush => {
                    let v1 = self.cards.iter().rev().map(|c| c.value).collect::<Vec<_>>();
                    let v2 = other.cards.iter().rev().map(|c| c.value).collect::<Vec<_>>();
                    v1.partial_cmp(&v2)
                },
                _ => {
                    let v1 = &self.freq.values().collect::<Vec<_>>();
                    let v2 = &other.freq.values().collect::<Vec<_>>();
                    v1.partial_cmp(v2)
                }
            }   
        }
    }
}

impl<'a> Ord for Hand<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut hands = hands.iter().map(|&h| Hand::from_str(h)).collect::<Vec<_>>();
    if hands.len() > 1 {
        hands.sort();
        hands.reverse();
        let hand = &hands[0];
        hands.iter().filter(|&h| h.cmp(hand) == Ordering::Equal).map(|h| h.src).collect()
    } else {
        hands.iter().map(|h| h.src).collect()
    }
}