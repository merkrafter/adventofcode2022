#![feature(array_windows)]

use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
    let result: usize = read_lines()
        .map(|line| line.expect("Could not read line"))
        .map(|line| {
            line.parse::<Strategy>()
                .expect("line did not contain a strategy")
        })
        .map(score)
        .sum();
    println!("Total score is {result}");
}

#[derive(Debug)]
struct Strategy {
    opponent: HandShape,
    own_reply: HandShape,
}

impl FromStr for Strategy {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let opponent = s[0..1].parse::<HandShape>()?;
        let own_reply = s[2..3].parse::<HandShape>()?;
        Ok(Strategy {
            opponent,
            own_reply,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
enum HandShape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for HandShape {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next() {
            Some('A' | 'X') => Ok(Self::Rock),
            Some('B' | 'Y') => Ok(Self::Paper),
            Some('C' | 'Z') => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

fn score(strategy: Strategy) -> usize {
    let outcome = compute_outcome(&strategy) as usize;
    let shape_bonus = strategy.own_reply as usize;
    outcome + shape_bonus
}

#[derive(Debug, PartialEq)]
enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

fn compute_outcome(strategy: &Strategy) -> Outcome {
    use HandShape::*;

    let Strategy {
        opponent,
        own_reply,
    } = strategy;
    match (opponent, own_reply) {
        (x, y) if x == y => Outcome::Draw,
        (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => Outcome::Win,
        _ => Outcome::Loss,
    }
}

fn read_lines() -> io::Lines<io::StdinLock<'static>> {
    io::stdin().lock().lines()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn hand_shape_parsing_works() {
        for s in ["A", "X"] {
            let shape = s.parse::<HandShape>();
            assert_eq!(Ok(HandShape::Rock), shape);
        }
        for s in ["B", "Y"] {
            let shape = s.parse::<HandShape>();
            assert_eq!(Ok(HandShape::Paper), shape);
        }
        for s in ["C", "Z"] {
            let shape = s.parse::<HandShape>();
            assert_eq!(Ok(HandShape::Scissors), shape);
        }
    }

    #[test]
    fn compute_outcome_works_for_wins() {
        const EXPECTED_OUTCOME: Outcome = Outcome::Win;

        assert_eq!(
            EXPECTED_OUTCOME,
            compute_outcome(&Strategy {
                opponent: HandShape::Rock,
                own_reply: HandShape::Paper
            })
        );

        assert_eq!(
            EXPECTED_OUTCOME,
            compute_outcome(&Strategy {
                opponent: HandShape::Paper,
                own_reply: HandShape::Scissors
            })
        );

        assert_eq!(
            EXPECTED_OUTCOME,
            compute_outcome(&Strategy {
                opponent: HandShape::Scissors,
                own_reply: HandShape::Rock
            })
        );
    }

    #[test]
    fn compute_outcome_works_for_draws() {
        for shape in [HandShape::Rock, HandShape::Paper, HandShape::Scissors] {
            assert_eq!(
                Outcome::Draw,
                compute_outcome(&Strategy {
                    opponent: shape.clone(),
                    own_reply: shape
                })
            );
        }
    }

    #[test]
    fn compute_outcome_works_for_losses() {
        const EXPECTED_OUTCOME: Outcome = Outcome::Loss;

        assert_eq!(
            EXPECTED_OUTCOME,
            compute_outcome(&Strategy {
                opponent: HandShape::Rock,
                own_reply: HandShape::Scissors
            })
        );

        assert_eq!(
            EXPECTED_OUTCOME,
            compute_outcome(&Strategy {
                opponent: HandShape::Paper,
                own_reply: HandShape::Rock
            })
        );

        assert_eq!(
            EXPECTED_OUTCOME,
            compute_outcome(&Strategy {
                opponent: HandShape::Scissors,
                own_reply: HandShape::Paper
            })
        );
    }

    #[test]
    fn score_works_for_rock_vs_paper() {
        let strategy = Strategy {
            opponent: HandShape::Rock,
            own_reply: HandShape::Paper,
        };
        let score = score(strategy);
        assert_eq!(8, score);
    }

    #[test]
    fn score_works_for_paper_vs_rock() {
        let strategy = Strategy {
            opponent: HandShape::Paper,
            own_reply: HandShape::Rock,
        };
        let score = score(strategy);
        assert_eq!(1, score);
    }

    #[test]
    fn score_works_for_scissors_vs_scissors() {
        let strategy = Strategy {
            opponent: HandShape::Scissors,
            own_reply: HandShape::Scissors,
        };
        let score = score(strategy);
        assert_eq!(6, score);
    }
}
