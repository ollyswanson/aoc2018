use std::collections::VecDeque;

pub struct Game {
    last: u32,
    marbles: VecDeque<u32>,
    scores: Vec<u32>,
}

impl Game {
    pub fn new(player_count: usize, last: u32) -> Self {
        let mut marbles: VecDeque<u32> = VecDeque::with_capacity(last as usize);
        marbles.push_back(0);
        let scores: Vec<u32> = vec![0; player_count];
        Game {
            last,
            marbles,
            scores,
        }
    }

    /// plays the game returning the highest score
    pub fn play(&mut self) -> u32 {
        // we model the circle as a double ended queue keeping the current marble at the back of
        // the queue with the marble that is 1 step clockwise at the front of the queue
        for (v, p) in (1..=self.last).zip((0..self.scores.len()).cycle()) {
            if v % 23 == 0 {
                for _ in 0..7 {
                    let t = self.marbles.pop_back().unwrap();
                    self.marbles.push_front(t);
                }

                let t = self.marbles.pop_back().unwrap();
                self.scores[p] += v + t;
                let t = self.marbles.pop_front().unwrap();
                self.marbles.push_back(t);
            } else {
                let t = self.marbles.pop_front().unwrap();
                self.marbles.push_back(t);
                self.marbles.push_back(v);
            }
        }

        *self.scores.iter().max().unwrap()
    }
}
