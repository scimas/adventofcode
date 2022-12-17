use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::Read,
};

fn main() -> Result<(), anyhow::Error> {
    let mut fl = File::open("resources/input12")?;
    let mut input = String::new();
    fl.read_to_string(&mut input)?;
    let (h_map, start_position, end_position) = parse_input(&input);
    let graph = create_graph(&h_map);
    println!("Day 12");
    println!("Part 1: {}", part1(&graph, start_position, end_position));
    println!("Part 2: {}", part2(&graph, &h_map, end_position));
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    row: i64,
    col: i64,
}

impl Position {
    fn new(row: i64, col: i64) -> Self {
        Position { row, col }
    }

    fn neighbors(&self) -> Vec<Self> {
        vec![
            Position::new(self.row - 1, self.col),
            Position::new(self.row + 1, self.col),
            Position::new(self.row, self.col - 1),
            Position::new(self.row, self.col + 1),
        ]
    }
}

type Graph = HashMap<Position, HashSet<Position>>;
type HeightMap = HashMap<Position, i8>;

fn parse_input(input: &str) -> (HeightMap, Position, Position) {
    let (mut row, mut col) = (0, 0);
    let mut hmap = HeightMap::new();
    let mut start = Position::new(-1, -1);
    let mut end = Position::new(-1, -1);
    for ch in input.chars() {
        if ch.is_whitespace() {
            row += 1;
            col = 0;
            continue;
        }
        let height = if ch == 'S' {
            start = Position::new(row, col);
            0
        } else if ch == 'E' {
            end = Position::new(row, col);
            25
        } else if let Some((idx, _)) = ('a'..='z').enumerate().find(|(_, h)| h == &ch) {
            idx as i8
        } else {
            panic!("unexpected character in input");
        };
        hmap.insert(Position::new(row, col), height);
        col += 1;
    }
    if start == Position::new(-1, -1) || end == Position::new(-1, -1) {
        panic!("start or end position not found");
    }
    (hmap, start, end)
}

fn create_graph(height_map: &HeightMap) -> Graph {
    let mut graph = Graph::new();
    for (pos, h) in height_map {
        let neighbors = pos.neighbors();
        let adjecant = neighbors
            .iter()
            .filter_map(|n_pos| height_map.get(n_pos).map(|n_h| (*n_pos, *n_h)))
            .filter(|(_, n_h)| *n_h <= *h + 1)
            .map(|(n_pos, _)| n_pos)
            .collect();
        graph.insert(*pos, adjecant);
    }
    graph
}

fn level_order_traverse(
    graph: &Graph,
    start_position: Position,
    end_position: Position,
) -> Option<usize> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::from([(start_position, 0)]);
    while !queue.is_empty() {
        let (pos, depth) = queue.pop_front().unwrap();
        visited.insert(pos);
        if pos == end_position {
            return Some(depth);
        }
        queue.extend(
            graph[&pos]
                .iter()
                .filter(|neighbor| {
                    if visited.contains(*neighbor) {
                        false
                    } else {
                        visited.insert(**neighbor);
                        true
                    }
                })
                .map(|neighbor| (*neighbor, depth + 1)),
        );
    }
    None
}

fn part1(graph: &Graph, start_position: Position, end_position: Position) -> usize {
    level_order_traverse(graph, start_position, end_position).unwrap()
}

fn part2(graph: &Graph, height_map: &HeightMap, end_position: Position) -> usize {
    height_map
        .iter()
        .filter(|(_, h)| **h == 0)
        .filter_map(|(pos, _)| level_order_traverse(graph, *pos, end_position))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use crate::{create_graph, parse_input, part1, part2, Position};

    fn test_input_1() -> String {
        "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
"
        .to_string()
    }

    #[test]
    fn parse_test() {
        let input = test_input_1();
        #[rustfmt::skip]
        let expected_hmap = HashMap::from([
            (Position::new(0,0), 0), (Position::new(0,1), 0), (Position::new(0,2), 1), (Position::new(0,3), 16), (Position::new(0,4), 15), (Position::new(0,5), 14), (Position::new(0,6), 13), (Position::new(0,7), 12), 
            (Position::new(1,0), 0), (Position::new(1,1), 1), (Position::new(1,2), 2), (Position::new(1,3), 17), (Position::new(1,4), 24), (Position::new(1,5), 23), (Position::new(1,6), 23), (Position::new(1,7), 11), 
            (Position::new(2,0), 0), (Position::new(2,1), 2), (Position::new(2,2), 2), (Position::new(2,3), 18), (Position::new(2,4), 25), (Position::new(2,5), 25), (Position::new(2,6), 23), (Position::new(2,7), 10), 
            (Position::new(3,0), 0), (Position::new(3,1), 2), (Position::new(3,2), 2), (Position::new(3,3), 19), (Position::new(3,4), 20), (Position::new(3,5), 21), (Position::new(3,6), 22), (Position::new(3,7),  9), 
            (Position::new(4,0), 0), (Position::new(4,1), 1), (Position::new(4,2), 3), (Position::new(4,3),  4), (Position::new(4,4),  5), (Position::new(4,5),  6), (Position::new(4,6),  7), (Position::new(4,7),  8),
        ]);
        let expected_start = Position::new(0, 0);
        let expected_end = Position::new(2, 5);
        assert_eq!(
            parse_input(&input),
            (expected_hmap, expected_start, expected_end)
        );
    }

    #[test]
    fn create_graph_test() {
        let input = test_input_1();
        let (h_map, _, _) = parse_input(&input);
        let expected = HashMap::from([
            (
                Position::new(0, 0),
                HashSet::from([Position::new(0, 1), Position::new(1, 0)]),
            ),
            (
                Position::new(0, 1),
                HashSet::from([
                    Position::new(0, 0),
                    Position::new(1, 1),
                    Position::new(0, 2),
                ]),
            ),
            (
                Position::new(0, 2),
                HashSet::from([Position::new(0, 1), Position::new(1, 2)]),
            ),
            (
                Position::new(0, 3),
                HashSet::from([
                    Position::new(0, 2),
                    Position::new(0, 4),
                    Position::new(1, 3),
                ]),
            ),
            (
                Position::new(0, 4),
                HashSet::from([Position::new(0, 3), Position::new(0, 5)]),
            ),
            (
                Position::new(0, 5),
                HashSet::from([Position::new(0, 4), Position::new(0, 6)]),
            ),
            (
                Position::new(0, 6),
                HashSet::from([Position::new(0, 5), Position::new(0, 7)]),
            ),
            (
                Position::new(0, 7),
                HashSet::from([Position::new(0, 6), Position::new(1, 7)]),
            ),
            (
                Position::new(1, 0),
                HashSet::from([
                    Position::new(0, 0),
                    Position::new(2, 0),
                    Position::new(1, 1),
                ]),
            ),
            (
                Position::new(1, 1),
                HashSet::from([
                    Position::new(0, 1),
                    Position::new(2, 1),
                    Position::new(1, 0),
                    Position::new(1, 2),
                ]),
            ),
            (
                Position::new(1, 2),
                HashSet::from([
                    Position::new(0, 2),
                    Position::new(2, 2),
                    Position::new(1, 1),
                ]),
            ),
            (
                Position::new(1, 3),
                HashSet::from([
                    Position::new(0, 3),
                    Position::new(2, 3),
                    Position::new(1, 2),
                ]),
            ),
            (
                Position::new(1, 4),
                HashSet::from([
                    Position::new(0, 4),
                    Position::new(2, 4),
                    Position::new(1, 3),
                    Position::new(1, 5),
                ]),
            ),
            (
                Position::new(1, 5),
                HashSet::from([
                    Position::new(0, 5),
                    Position::new(1, 4),
                    Position::new(1, 6),
                ]),
            ),
            (
                Position::new(1, 6),
                HashSet::from([
                    Position::new(0, 6),
                    Position::new(2, 6),
                    Position::new(1, 5),
                    Position::new(1, 7),
                ]),
            ),
            (
                Position::new(1, 7),
                HashSet::from([Position::new(0, 7), Position::new(2, 7)]),
            ),
            (
                Position::new(2, 0),
                HashSet::from([Position::new(1, 0), Position::new(3, 0)]),
            ),
            (
                Position::new(2, 1),
                HashSet::from([
                    Position::new(2, 0),
                    Position::new(2, 2),
                    Position::new(1, 1),
                    Position::new(3, 1),
                ]),
            ),
            (
                Position::new(2, 2),
                HashSet::from([
                    Position::new(1, 2),
                    Position::new(3, 2),
                    Position::new(2, 1),
                ]),
            ),
            (
                Position::new(2, 3),
                HashSet::from([
                    Position::new(1, 3),
                    Position::new(3, 3),
                    Position::new(2, 2),
                ]),
            ),
            (
                Position::new(2, 4),
                HashSet::from([
                    Position::new(1, 4),
                    Position::new(3, 4),
                    Position::new(2, 3),
                    Position::new(2, 5),
                ]),
            ),
            (
                Position::new(2, 5),
                HashSet::from([
                    Position::new(1, 5),
                    Position::new(3, 5),
                    Position::new(2, 4),
                    Position::new(2, 6),
                ]),
            ),
            (
                Position::new(2, 6),
                HashSet::from([
                    Position::new(1, 6),
                    Position::new(3, 6),
                    Position::new(2, 7),
                ]),
            ),
            (
                Position::new(2, 7),
                HashSet::from([Position::new(1, 7), Position::new(3, 7)]),
            ),
            (
                Position::new(3, 0),
                HashSet::from([Position::new(2, 0), Position::new(4, 0)]),
            ),
            (
                Position::new(3, 1),
                HashSet::from([
                    Position::new(2, 1),
                    Position::new(4, 1),
                    Position::new(3, 0),
                    Position::new(3, 2),
                ]),
            ),
            (
                Position::new(3, 2),
                HashSet::from([
                    Position::new(2, 2),
                    Position::new(4, 2),
                    Position::new(3, 1),
                ]),
            ),
            (
                Position::new(3, 3),
                HashSet::from([
                    Position::new(2, 3),
                    Position::new(4, 3),
                    Position::new(3, 2),
                    Position::new(3, 4),
                ]),
            ),
            (
                Position::new(3, 4),
                HashSet::from([
                    Position::new(4, 4),
                    Position::new(3, 3),
                    Position::new(3, 5),
                ]),
            ),
            (
                Position::new(3, 5),
                HashSet::from([
                    Position::new(4, 5),
                    Position::new(3, 4),
                    Position::new(3, 6),
                ]),
            ),
            (
                Position::new(3, 6),
                HashSet::from([
                    Position::new(2, 6),
                    Position::new(4, 6),
                    Position::new(3, 5),
                    Position::new(3, 7),
                ]),
            ),
            (
                Position::new(3, 7),
                HashSet::from([Position::new(2, 7), Position::new(4, 7)]),
            ),
            (
                Position::new(4, 0),
                HashSet::from([Position::new(3, 0), Position::new(4, 1)]),
            ),
            (
                Position::new(4, 1),
                HashSet::from([Position::new(3, 1), Position::new(4, 0)]),
            ),
            (
                Position::new(4, 2),
                HashSet::from([
                    Position::new(3, 2),
                    Position::new(4, 1),
                    Position::new(4, 3),
                ]),
            ),
            (
                Position::new(4, 3),
                HashSet::from([Position::new(4, 2), Position::new(4, 4)]),
            ),
            (
                Position::new(4, 4),
                HashSet::from([Position::new(4, 3), Position::new(4, 5)]),
            ),
            (
                Position::new(4, 5),
                HashSet::from([Position::new(4, 4), Position::new(4, 6)]),
            ),
            (
                Position::new(4, 6),
                HashSet::from([Position::new(4, 5), Position::new(4, 7)]),
            ),
            (
                Position::new(4, 7),
                HashSet::from([Position::new(3, 7), Position::new(4, 6)]),
            ),
        ]);
        let graph = create_graph(&h_map);
        for k in expected.keys() {
            assert_eq!(graph[k], expected[k], "failure for {k:?}");
        }
    }

    #[test]
    fn part1_test_1() {
        let input = test_input_1();
        let (h_map, start_position, end_position) = parse_input(&input);
        let graph = create_graph(&h_map);
        assert_eq!(part1(&graph, start_position, end_position), 31);
    }

    #[test]
    fn part2_test_2() {
        let input = test_input_1();
        let (h_map, _, end_position) = parse_input(&input);
        let graph = create_graph(&h_map);
        assert_eq!(part2(&graph, &h_map, end_position), 29);
    }
}
