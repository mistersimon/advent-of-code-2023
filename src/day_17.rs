use std::cmp;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    fn get_diff(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
    direction: Direction,
    steps_direction: usize,
}

// Priority queue depends on `Ord`
// Implement so trait so queue becomes a min-heap
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Key {
    position: (usize, usize),
    direction: Direction,
    steps_direction: usize,
}

fn shortest_path(field: &Vec<Vec<usize>>, min_step: usize, max_step: usize) -> Option<usize> {
    let start = (0, 0);
    let goal = (field.len() - 1, field[0].len() - 1);

    // dist node = current shortest distance form start to `node`
    let mut dist: HashMap<Key, usize> = HashMap::new();
    let mut heap = BinaryHeap::new();

    let directions = vec![
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];

    // we are at `start` with zero cost
    dist.insert(
        Key {
            position: start,
            direction: Direction::Right,
            steps_direction: 0,
        },
        0,
    );
    dist.insert(
        Key {
            position: start,
            direction: Direction::Down,
            steps_direction: 0,
        },
        0,
    );
    heap.push(State {
        cost: 0,
        position: start,
        direction: Direction::Right,
        steps_direction: 0,
    });

    let mut shortest: Option<usize> = None;
    while let Some(State {
        cost,
        position,
        direction,
        steps_direction,
    }) = heap.pop()
    {
        if position == goal && steps_direction >= min_step {
            if shortest.is_none() {
                shortest = Some(cost)
            } else {
                shortest = Some(cmp::min(cost, shortest.unwrap()))
            }
        }

        let key = Key {
            position,
            direction,
            steps_direction,
        };

        if dist.contains_key(&key) && cost > dist[&key] {
            // Already found a better way here
            continue;
        }

        for dir in &directions {
            // Don't go back
            if *dir == direction.opposite() {
                continue;
            }
            let (dx, dy) = dir.get_diff();
            if (position.0 as isize + dx < 0)
                || (position.1 as isize + dy < 0)
                || (position.0 as isize + dx >= field.len() as isize)
                || (position.1 as isize + dy >= field[0].len() as isize)
            {
                continue;
            }

            let new_position = (
                (position.0 as isize + dx) as usize,
                (position.1 as isize + dy) as usize,
            );

            let next = State {
                position: new_position,
                direction: *dir,
                steps_direction: if *dir == direction {
                    steps_direction + 1
                } else {
                    1
                },
                cost: cost + field[new_position.0][new_position.1],
            };

            let key = Key {
                position: new_position,
                direction: *dir,
                steps_direction: next.steps_direction,
            };

            if (direction == *dir || steps_direction >= min_step)
                && next.steps_direction <= max_step
                && (!dist.contains_key(&key) || next.cost < dist[&key])
            {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist.insert(key, next.cost);
            }
        }
    }

    return shortest;
}

pub fn part_one(lines: impl Iterator<Item = String>) -> u64 {
    let map = lines
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    shortest_path(&map, 1, 3).unwrap().try_into().unwrap()
}

pub fn part_two(lines: impl Iterator<Item = String>) -> u64 {
    let map = lines
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    shortest_path(&map, 4, 10).unwrap().try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    // use rstest::rstest;

    const CASE_1: &str = indoc! {"
        2413432311323
        3215453535623
        3255245654254
        3446585845452
        4546657867536
        1438598798454
        4457876987766
        3637877979653
        4654967986887
        4564679986453
        1224686865563
        2546548887735
        4322674655533
    "};

    #[test]
    fn test_part_one_case_1() {
        let lines = CASE_1.lines().map(|line| line.to_string());
        let result = super::part_one(lines);
        assert_eq!(result, 102);
    }

    #[test]
    fn test_part_two_case_1() {
        let lines = CASE_1.lines().map(|line| line.to_string());
        let result = super::part_two(lines);
        assert_eq!(result, 94);
    }
}
