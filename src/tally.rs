use std::collections::HashMap;

const HEADER: &'static str = "Team                           | MP |  W |  D |  L |  P";

pub fn tally(match_results: &str) -> String {
    let mut team_scores: Vec<(&str, CompetitionResult)> = match_results
        .lines()
        .into_iter()
        .map(|played| played.split(';').collect::<Vec<&str>>())
        .fold(HashMap::<&str, CompetitionResult>::new(), |mut accu, team| {
            accu.entry(team[0])
                .or_insert(CompetitionResult::new_with_str(team[0]))
                .update(team[2], true);
            accu.entry(team[1])
                .or_insert(CompetitionResult::new_with_str(team[1]))
                .update(team[2], false);
            return accu;
        })
        .into_iter()
        .collect();
    team_scores.sort_by(|a, b| b.1.get_point().cmp(&a.1.get_point())
        .then_with(|| a.0.cmp(b.0)));
    vec![HEADER.to_string()]
        .into_iter()
        .chain(team_scores.into_iter().map(|(_, r)| r.into()))
        .collect::<Vec<String>>()
        .join("\n")
}


pub struct CompetitionResult {
    name: String,
    win: usize,
    draw: usize,
    loss: usize,
}

impl CompetitionResult {
    pub fn new_with_str(str: &str) -> Self {
        Self {
            name: str.to_string(),
            win: 0,
            draw: 0,
            loss: 0,
        }
    }
    pub fn update(&mut self, result: &str, is_team1: bool) {
        match (result, is_team1) {
            ("win", true) | ("loss", false) => self.win += 1,
            ("loss", true) | ("win", false) => self.loss += 1,
            _ => self.draw += 1,
        }
    }
    pub fn get_mp(&self) -> usize {
        self.win + self.draw + self.loss
    }
    pub fn get_point(&self) -> usize {
        3 * self.win + self.draw
    }
}

impl From<CompetitionResult> for String {
    fn from(origin: CompetitionResult) -> Self {
        format!(
            "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            origin.name,
            origin.get_mp(),
            origin.win,
            origin.draw,
            origin.loss,
            origin.get_point(),
        )
    }
}


mod tally_test {
    use crate::tally::tally;

    #[test]
    fn just_the_header_if_no_input() {
        let input: &[&str] = &[];
        let input = input.join("\n");
        let output = tally(&input);
        let expected = ["Team                           | MP |  W |  D |  L |  P"].join("\n");
        assert_eq!(output, expected);
    }

    #[test]
    fn a_win_is_three_points_a_loss_is_zero_points() {
        let input: &[&str] = &["Allegoric Alaskans;Blithering Badgers;win"];
        let input = input.join("\n");
        let output = tally(&input);
        let expected = [
            "Team                           | MP |  W |  D |  L |  P",
            "Allegoric Alaskans             |  1 |  1 |  0 |  0 |  3",
            "Blithering Badgers             |  1 |  0 |  0 |  1 |  0",
        ]
            .join("\n");
        assert_eq!(output, expected);
    }

    #[test]
    fn a_win_can_also_be_expressed_as_a_loss() {
        let input: &[&str] = &["Blithering Badgers;Allegoric Alaskans;loss"];
        let input = input.join("\n");
        let output = tally(&input);
        let expected = [
            "Team                           | MP |  W |  D |  L |  P",
            "Allegoric Alaskans             |  1 |  1 |  0 |  0 |  3",
            "Blithering Badgers             |  1 |  0 |  0 |  1 |  0",
        ]
            .join("\n");
        assert_eq!(output, expected);
    }

    #[test]
    fn a_different_team_can_win() {
        let input: &[&str] = &["Blithering Badgers;Allegoric Alaskans;win"];
        let input = input.join("\n");
        let output = tally(&input);
        let expected = [
            "Team                           | MP |  W |  D |  L |  P",
            "Blithering Badgers             |  1 |  1 |  0 |  0 |  3",
            "Allegoric Alaskans             |  1 |  0 |  0 |  1 |  0",
        ]
            .join("\n");
        assert_eq!(output, expected);
    }

    #[test]
    fn a_draw_is_one_point_each() {
        let input: &[&str] = &["Allegoric Alaskans;Blithering Badgers;draw"];
        let input = input.join("\n");
        let output = tally(&input);
        let expected = [
            "Team                           | MP |  W |  D |  L |  P",
            "Allegoric Alaskans             |  1 |  0 |  1 |  0 |  1",
            "Blithering Badgers             |  1 |  0 |  1 |  0 |  1",
        ]
            .join("\n");
        assert_eq!(output, expected);
    }

    #[test]
    fn there_can_be_more_than_one_match() {
        let input: &[&str] = &[
            "Allegoric Alaskans;Blithering Badgers;win",
            "Allegoric Alaskans;Blithering Badgers;win",
        ];
        let input = input.join("\n");
        let output = tally(&input);
        let expected = [
            "Team                           | MP |  W |  D |  L |  P",
            "Allegoric Alaskans             |  2 |  2 |  0 |  0 |  6",
            "Blithering Badgers             |  2 |  0 |  0 |  2 |  0",
        ]
            .join("\n");
        assert_eq!(output, expected);
    }

    #[test]
    fn there_can_be_more_than_one_winner() {
        let input: &[&str] = &[
            "Allegoric Alaskans;Blithering Badgers;loss",
            "Allegoric Alaskans;Blithering Badgers;win",
        ];
        let input = input.join("\n");
        let output = tally(&input);
        let expected = [
            "Team                           | MP |  W |  D |  L |  P",
            "Allegoric Alaskans             |  2 |  1 |  0 |  1 |  3",
            "Blithering Badgers             |  2 |  1 |  0 |  1 |  3",
        ]
            .join("\n");
        assert_eq!(output, expected);
    }

    #[test]
    fn there_can_be_more_than_two_teams() {
        let input: &[&str] = &[
            "Allegoric Alaskans;Blithering Badgers;win",
            "Blithering Badgers;Courageous Californians;win",
            "Courageous Californians;Allegoric Alaskans;loss",
        ];
        let input = input.join("\n");
        let output = tally(&input);
        let expected = [
            "Team                           | MP |  W |  D |  L |  P",
            "Allegoric Alaskans             |  2 |  2 |  0 |  0 |  6",
            "Blithering Badgers             |  2 |  1 |  0 |  1 |  3",
            "Courageous Californians        |  2 |  0 |  0 |  2 |  0",
        ]
            .join("\n");
        assert_eq!(output, expected);
    }

    #[test]
    fn typical_input() {
        let input: &[&str] = &[
            "Allegoric Alaskans;Blithering Badgers;win",
            "Devastating Donkeys;Courageous Californians;draw",
            "Devastating Donkeys;Allegoric Alaskans;win",
            "Courageous Californians;Blithering Badgers;loss",
            "Blithering Badgers;Devastating Donkeys;loss",
            "Allegoric Alaskans;Courageous Californians;win",
        ];
        let input = input.join("\n");
        let output = tally(&input);
        let expected = [
            "Team                           | MP |  W |  D |  L |  P",
            "Devastating Donkeys            |  3 |  2 |  1 |  0 |  7",
            "Allegoric Alaskans             |  3 |  2 |  0 |  1 |  6",
            "Blithering Badgers             |  3 |  1 |  0 |  2 |  3",
            "Courageous Californians        |  3 |  0 |  1 |  2 |  1",
        ]
            .join("\n");
        assert_eq!(output, expected);
    }

    #[test]
    fn incomplete_competition_not_all_pairs_have_played() {
        let input: &[&str] = &[
            "Allegoric Alaskans;Blithering Badgers;loss",
            "Devastating Donkeys;Allegoric Alaskans;loss",
            "Courageous Californians;Blithering Badgers;draw",
            "Allegoric Alaskans;Courageous Californians;win",
        ];
        let input = input.join("\n");
        let output = tally(&input);
        let expected = [
            "Team                           | MP |  W |  D |  L |  P",
            "Allegoric Alaskans             |  3 |  2 |  0 |  1 |  6",
            "Blithering Badgers             |  2 |  1 |  1 |  0 |  4",
            "Courageous Californians        |  2 |  0 |  1 |  1 |  1",
            "Devastating Donkeys            |  1 |  0 |  0 |  1 |  0",
        ]
            .join("\n");
        assert_eq!(output, expected);
    }

    #[test]
    fn ties_broken_alphabetically() {
        let input: &[&str] = &[
            "Courageous Californians;Devastating Donkeys;win",
            "Allegoric Alaskans;Blithering Badgers;win",
            "Devastating Donkeys;Allegoric Alaskans;loss",
            "Courageous Californians;Blithering Badgers;win",
            "Blithering Badgers;Devastating Donkeys;draw",
            "Allegoric Alaskans;Courageous Californians;draw",
        ];
        let input = input.join("\n");
        let output = tally(&input);
        let expected = [
            "Team                           | MP |  W |  D |  L |  P",
            "Allegoric Alaskans             |  3 |  2 |  1 |  0 |  7",
            "Courageous Californians        |  3 |  2 |  1 |  0 |  7",
            "Blithering Badgers             |  3 |  0 |  1 |  2 |  1",
            "Devastating Donkeys            |  3 |  0 |  1 |  2 |  1",
        ]
            .join("\n");
        assert_eq!(output, expected);
    }

    #[test]
    fn ensure_points_sorted_numerically() {
        let input: &[&str] = &[
            "Devastating Donkeys;Blithering Badgers;win",
            "Devastating Donkeys;Blithering Badgers;win",
            "Devastating Donkeys;Blithering Badgers;win",
            "Devastating Donkeys;Blithering Badgers;win",
            "Blithering Badgers;Devastating Donkeys;win",
        ];
        let input = input.join("\n");
        let output = tally(&input);
        let expected = [
            "Team                           | MP |  W |  D |  L |  P",
            "Devastating Donkeys            |  5 |  4 |  0 |  1 | 12",
            "Blithering Badgers             |  5 |  1 |  0 |  4 |  3",
        ]
            .join("\n");
        assert_eq!(output, expected);
    }
}