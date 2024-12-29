use crate::common::{Grid, Vec2i};
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash)]
pub enum Tile {
    #[default]
    Off,
    On,
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Tile::Off,
            '#' => Tile::On,
            _ => {
                return Err(());
            }
        })
    }
}

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Grid<Tile> {
    input.parse().unwrap()
}

fn solve<const PART2: bool>(grid: &Grid<Tile>, steps: usize) -> usize {
    const NEIGHBORS: [Vec2i; 8] = [
        Vec2i::new(0, 1),
        Vec2i::new(0, -1),
        Vec2i::new(1, 0),
        Vec2i::new(-1, 0),
        Vec2i::new(1, 1),
        Vec2i::new(1, -1),
        Vec2i::new(-1, 1),
        Vec2i::new(-1, -1),
    ];

    let mut grid = grid.clone();
    let (size_x, size_y) = (grid.size_x, grid.size_y);
    if PART2 {
        grid[Vec2i::new(0, 0)] = Tile::On;
        grid[Vec2i::new(0, size_y as i64 - 1)] = Tile::On;
        grid[Vec2i::new(size_x as i64 - 1, 0)] = Tile::On;
        grid[Vec2i::new(size_x as i64 - 1, size_y as i64 - 1)] = Tile::On;
    }

    for _ in 0..steps {
        let mut new_grid = Grid::new_from_default(size_x, size_y);
        for (pos, tile) in grid.pos_iter() {
            let active_neighbors = NEIGHBORS
                .into_iter()
                .map(|d| pos + d)
                .filter(|p| grid.in_bounds(p))
                .map(|p| grid[p])
                .filter(|&t| t == Tile::On)
                .count();
            let new_tile = match (tile, active_neighbors) {
                (Tile::Off, 3) => Tile::On,
                (Tile::On, 2 | 3) => Tile::On,
                _ => Tile::Off,
            };
            new_grid[pos] = new_tile;
        }
        if PART2 {
            new_grid[Vec2i::new(0, 0)] = Tile::On;
            new_grid[Vec2i::new(0, size_y as i64 - 1)] = Tile::On;
            new_grid[Vec2i::new(size_x as i64 - 1, 0)] = Tile::On;
            new_grid[Vec2i::new(size_x as i64 - 1, size_y as i64 - 1)] = Tile::On;
        }
        grid = new_grid;
    }

    grid.iter().filter(|&&tile| tile == Tile::On).count()
}

#[aoc(day18, part1)]
pub fn part1(input: &Grid<Tile>) -> usize {
    solve::<false>(input, 100)
}

#[aoc(day18, part2)]
pub fn part2(input: &Grid<Tile>) -> usize {
    solve::<true>(input, 100)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    const INPUT: &str = r#".#.#.#
...##.
#....#
..#...
#.#..#
####.."#;

    #[test]
    fn test_part1() {
        assert_eq!(solve::<false>(&input_generator(INPUT), 4), 4);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve::<true>(&input_generator(INPUT), 5), 17);
    }
}
