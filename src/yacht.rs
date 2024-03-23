use std::collections::HashMap;

#[derive(Debug)]
pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

type Dice = [u8; 5];

pub fn score(dice: Dice, category: Category) -> u8 {
    match category {
        Category::Ones => dice.iter().filter(|&&num| num == 1).count() as u8,
        Category::Twos => dice.iter().filter(|&&num| num == 2).count() as u8 * 2,
        Category::Threes => dice.iter().filter(|&&num| num == 3).count() as u8 * 3,
        Category::Fours => dice.iter().filter(|&&num| num == 4).count() as u8 * 4,
        Category::Fives => dice.iter().filter(|&&num| num == 5).count() as u8 * 5,
        Category::Sixes => dice.iter().filter(|&&num| num == 6).count() as u8 * 6,
        Category::FullHouse => if is_full_house(dice) { dice.iter().sum() } else { 0 },
        Category::FourOfAKind => if let Some(four) = is_four_of_a_kind(dice) { 4 * four } else { 0 },
        Category::LittleStraight => if is_little_straight(dice) { 30 } else { 0 }
        Category::BigStraight => if is_big_straight(dice) { 30 } else { 0 },
        Category::Choice => dice.iter().sum(),
        Category::Yacht => if is_yacht(dice) { 50 } else { 0 }
    }
}

fn is_yacht(dice: Dice) -> bool {
    dice.iter().all(|&x| x.eq(&dice[0]))
}

fn is_big_straight(dice: Dice) -> bool {
    let mut dice_cloned = dice.clone();
    dice_cloned.sort();
    return dice_cloned == [2, 3, 4, 5, 6];
}

fn is_little_straight(dice: Dice) -> bool {
    let mut dice_cloned = dice.clone();
    dice_cloned.sort();
    return dice_cloned == [1, 2, 3, 4, 5];
}

fn is_four_of_a_kind(dice: Dice) -> Option<u8> {
    let freq = dice.iter().fold(HashMap::new(), |mut acc, &num| {
        *acc.entry(num).or_insert(0) += 1;
        acc
    });
    let values = freq.values().copied().collect::<Vec<u32>>();
    if values == vec![1, 4] || values == vec![4, 1] || values == vec![5] {
        // return the number that is repeated 4 times
        return Some(*freq.iter().find(|(_, &v)| v >= 4).unwrap().0);
    }
    None
}

fn is_full_house(dice: Dice) -> bool {
    // dice is full house if it has 2 different numbers and one of them is repeated 3 times
    let freq = dice.iter().fold(HashMap::new(), |mut acc, &num| {
        *acc.entry(num).or_insert(0) += 1;
        acc
    }).values().copied().collect::<Vec<u32>>();
    freq == vec![2, 3] || freq == vec![3, 2]
}

mod yacht_test {
    use super::*;

    #[test]
    fn yacht() {
        let expected = 50;
        assert_eq!(score([5, 5, 5, 5, 5], Category::Yacht), expected);
    }

    #[test]
    fn not_yacht() {
        let expected = 0;
        assert_eq!(score([1, 3, 3, 2, 5], Category::Yacht), expected);
    }

    #[test]
    fn ones() {
        let expected = 3;
        assert_eq!(score([1, 1, 1, 3, 5], Category::Ones), expected);
    }

    #[test]
    fn ones_out_of_order() {
        let expected = 3;
        assert_eq!(score([3, 1, 1, 5, 1], Category::Ones), expected);
    }

    #[test]
    fn no_ones() {
        let expected = 0;
        assert_eq!(score([4, 3, 6, 5, 5], Category::Ones), expected);
    }

    #[test]
    fn twos() {
        let expected = 2;
        assert_eq!(score([2, 3, 4, 5, 6], Category::Twos), expected);
    }

    #[test]
    fn fours() {
        let expected = 8;
        assert_eq!(score([1, 4, 1, 4, 1], Category::Fours), expected);
    }

    #[test]
    fn yacht_counted_as_threes() {
        let expected = 15;
        assert_eq!(score([3, 3, 3, 3, 3], Category::Threes), expected);
    }

    #[test]
    fn yacht_of_3s_counted_as_fives() {
        let expected = 0;
        assert_eq!(score([3, 3, 3, 3, 3], Category::Fives), expected);
    }

    #[test]
    fn fives() {
        let expected = 10;
        assert_eq!(score([1, 5, 3, 5, 3], Category::Fives), expected);
    }

    #[test]
    fn sixes() {
        let expected = 6;
        assert_eq!(score([2, 3, 4, 5, 6], Category::Sixes), expected);
    }

    #[test]
    fn full_house_two_small_three_big() {
        let expected = 16;
        assert_eq!(score([2, 2, 4, 4, 4], Category::FullHouse), expected);
    }

    #[test]
    fn full_house_three_small_two_big() {
        let expected = 19;
        assert_eq!(score([5, 3, 3, 5, 3], Category::FullHouse), expected);
    }

    #[test]
    fn two_pair_is_not_a_full_house() {
        let expected = 0;
        assert_eq!(score([2, 2, 4, 4, 5], Category::FullHouse), expected);
    }

    #[test]
    fn four_of_a_kind_is_not_a_full_house() {
        let expected = 0;
        assert_eq!(score([1, 4, 4, 4, 4], Category::FullHouse), expected);
    }

    #[test]
    fn yacht_is_not_a_full_house() {
        let expected = 0;
        assert_eq!(score([2, 2, 2, 2, 2], Category::FullHouse), expected);
    }

    #[test]
    fn four_of_a_kind() {
        let expected = 24;
        assert_eq!(score([6, 6, 4, 6, 6], Category::FourOfAKind), expected);
    }

    #[test]
    fn yacht_can_be_scored_as_four_of_a_kind() {
        let expected = 12;
        assert_eq!(score([3, 3, 3, 3, 3], Category::FourOfAKind), expected);
    }

    #[test]
    fn full_house_is_not_four_of_a_kind() {
        let expected = 0;
        assert_eq!(score([3, 3, 3, 5, 5], Category::FourOfAKind), expected);
    }

    #[test]
    fn little_straight() {
        let expected = 30;
        assert_eq!(score([3, 5, 4, 1, 2], Category::LittleStraight), expected);
    }

    #[test]
    fn little_straight_as_big_straight() {
        let expected = 0;
        assert_eq!(score([1, 2, 3, 4, 5], Category::BigStraight), expected);
    }

    #[test]
    fn four_in_order_but_not_a_little_straight() {
        let expected = 0;
        assert_eq!(score([1, 1, 2, 3, 4], Category::LittleStraight), expected);
    }

    #[test]
    fn no_pairs_but_not_a_little_straight() {
        let expected = 0;
        assert_eq!(score([1, 2, 3, 4, 6], Category::LittleStraight), expected);
    }

    #[test]
    fn minimum_is_1_maximum_is_5_but_not_a_little_straight() {
        let expected = 0;
        assert_eq!(score([1, 1, 3, 4, 5], Category::LittleStraight), expected);
    }

    #[test]
    fn big_straight() {
        let expected = 30;
        assert_eq!(score([4, 6, 2, 5, 3], Category::BigStraight), expected);
    }

    #[test]
    fn big_straight_as_little_straight() {
        let expected = 0;
        assert_eq!(score([6, 5, 4, 3, 2], Category::LittleStraight), expected);
    }

    #[test]
    fn no_pairs_but_not_a_big_straight() {
        let expected = 0;
        assert_eq!(score([6, 5, 4, 3, 1], Category::BigStraight), expected);
    }

    #[test]
    fn choice() {
        let expected = 23;
        assert_eq!(score([3, 3, 5, 6, 6], Category::Choice), expected);
    }

    #[test]
    fn yacht_as_choice() {
        let expected = 10;
        assert_eq!(score([2, 2, 2, 2, 2], Category::Choice), expected);
    }
}