use std::cmp::{PartialOrd, Ordering};
use std::collections::{BTreeSet, HashMap};
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum CardValue { // Ace may have a value of One
    One, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace,
}

impl CardValue {
    fn from_str(s: &str) -> CardValue {
        const CARDVALUES: [CardValue; 14] = [
            CardValue::One, CardValue::Two, CardValue::Three, CardValue::Four, CardValue::Five,
            CardValue::Six, CardValue::Seven, CardValue::Eight, CardValue::Nine, CardValue::Ten,
            CardValue::Jack, CardValue::Queen, CardValue::King, CardValue::Ace,
        ];
        match s.parse::<usize>() {
            Ok(i) => {
                if i >=2 && i <=10 { CARDVALUES[i-1] } else { panic!("Bad value: {}", s) }
            },
            Err(_) => match s {
                "J" => CardValue::Jack,
                "Q" => CardValue::Queen,
                "K" => CardValue::King,
                "A" => CardValue::Ace,
                _ => panic!("Bad value: {}", s),
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
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
    fn is_adjacent(self: &Self, other: &Self) -> bool {
        (self.value as i8 - other.value as i8).abs() == 1
    }
}

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
    freq: HashMap<Tuple, Vec<CardValue>>,
}

impl fmt::Debug for Hand<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} ({})", self.rank, self.src)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Tuple {
    Single,
    Pair,
    Triad,
    Quad,
}

fn frequencies(values: Vec<CardValue>) -> HashMap<Tuple, Vec<CardValue>> {
    let mut h1 = HashMap::<CardValue, u8>::new();
    for v in values {
        h1.entry(v).and_modify(|count| *count += 1).or_insert(1);
    }
    let mut h2: HashMap<Tuple, BTreeSet<CardValue>> = HashMap::new();
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
        .map(|(k, v)| (k, v.into_iter().rev().collect::<Vec<CardValue>>()))
        .collect::<HashMap<Tuple, Vec<CardValue>>>()
}


fn is_straight(hand: &mut Hand) -> bool {
    if hand.cards.iter().zip(hand.cards.iter().skip(1)).all(|(c1, c2)| c1.is_adjacent(c2)) {
        return true
    }
    // check with Ace as value One
    let mut new_cards = hand.cards.iter().map(|&c| if c.value == CardValue::Ace { Card { value: CardValue::One, ..c } } else { c }).collect::<BTreeSet<Card>>();
    if new_cards.iter().zip(new_cards.iter().skip(1)).all(|(c1, c2)| c1.is_adjacent(c2)) {
        // replace Ace value with One
        hand.cards.clear();
        hand.cards.append(&mut new_cards);
        return true
    }
    false
}

fn is_flush(hand: &Hand) -> bool {
    hand.cards.iter().zip(hand.cards.iter().skip(1)).all(|(c1, c2)| c1.suit == c2.suit)
}

fn is_four_of_a_kind(hand: &Hand) -> bool {
    hand.freq.contains_key(&Tuple::Quad)
}

fn have_one_pair(hand: &Hand) -> bool {
    hand.freq.contains_key(&Tuple::Pair)
}

fn have_two_pair(hand: &Hand) -> bool {
    hand.freq.contains_key(&Tuple::Pair) && hand.freq.get(&Tuple::Pair).unwrap().len() == 2
}

fn have_three_of_a_kind(hand: &Hand) -> bool {
    hand.freq.contains_key(&Tuple::Triad)
}

fn is_full_house(hand: &Hand) -> bool {
    have_three_of_a_kind(hand) && have_one_pair(hand)
}

impl Hand<'_> {
    fn from_str(src: &str) -> Hand {
        let cards_str = src.split(" ").collect::<Vec<_>>();
        if cards_str.len() != 5 { panic!("Cannot find 5 cards in the hand: {}", src) }
        let cards: BTreeSet<Card> = cards_str.iter().map(|&s| Card::from_str(s)).collect();
        let values = cards.iter().map(|c| c.value).collect::<Vec<CardValue>>();
        let mut hand = Hand {cards, src, rank: Rank::HighCard, freq: frequencies(values)};
        
        if is_straight(&mut hand) && is_flush(&hand) { hand.rank = Rank::StraightFlush }
        else if is_four_of_a_kind(&hand) { hand.rank = Rank::FourOfAKind }
        else if is_full_house(&hand) { hand.rank = Rank::FullHouse }
        else if is_flush(&hand) { hand.rank = Rank::Flush }
        else if is_straight(&mut hand) { hand.rank = Rank::Straight }
        else if have_three_of_a_kind(&hand) { hand.rank = Rank::ThreeOFAKind }
        else if have_two_pair(&hand) { hand.rank = Rank::TwoPair }
        else if have_one_pair(&hand) { hand.rank = Rank::OnePair }
        hand
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
                    let v1 = self.cards.iter().map(|c| c.value).collect::<Vec<CardValue>>();
                    let v2 = other.cards.iter().map(|c| c.value).collect::<Vec<CardValue>>();
                    v1.partial_cmp(&v2)
                },
                Rank::FourOfAKind => {
                    let v11 = &self.freq.get(&Tuple::Quad).unwrap();
                    let v12 = &self.freq.get(&Tuple::Single).unwrap();
                    let v21 = &other.freq.get(&Tuple::Quad).unwrap();
                    let v22 = &other.freq.get(&Tuple::Single).unwrap();
                    (v11, v12).partial_cmp(&(v21, v22))
                },
                Rank::FullHouse => {
                    let v11 = &self.freq.get(&Tuple::Triad).unwrap();
                    let v12 = &self.freq.get(&Tuple::Pair).unwrap();
                    let v21 = &other.freq.get(&Tuple::Triad).unwrap();
                    let v22 = &other.freq.get(&Tuple::Pair).unwrap();
                    (v11, v12).partial_cmp(&(v21, v22))
                },
                Rank::ThreeOFAKind => {
                    let v11 = &self.freq.get(&Tuple::Triad).unwrap();
                    let v12 = &self.freq.get(&Tuple::Single).unwrap();
                    let v21 = &other.freq.get(&Tuple::Triad).unwrap();
                    let v22 = &other.freq.get(&Tuple::Single).unwrap();
                    (v11, v12).partial_cmp(&(v21, v22))
                },
                Rank::OnePair | Rank::TwoPair => {
                    let v11 = &self.freq.get(&Tuple::Pair).unwrap();
                    let v12 = &self.freq.get(&Tuple::Single).unwrap();
                    let v21 = &other.freq.get(&Tuple::Pair).unwrap();
                    let v22 = &other.freq.get(&Tuple::Single).unwrap();
                    (v11, v12).partial_cmp(&(v21, v22))
                },
            }   
        }
    }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut hands = hands.iter().map(|&h| Hand::from_str(h)).collect::<Vec<Hand>>();
    hands.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Less));
    hands.reverse();
    if hands.len() > 1 {
        let hand = &hands[0];
        hands.iter().filter(|&h| h.partial_cmp(hand).unwrap() == Ordering::Equal).map(|h| h.src).collect()
    } else {
        hands.iter().map(|h| h.src).collect()
    }
}