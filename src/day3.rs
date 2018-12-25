use regex::Regex;
use std::collections::HashMap;

/*
  Each Elf has made a claim about which area of fabric would be ideal for Santa's suit. All claims have an ID and consist of a single rectangle with edges parallel to the edges of the fabric. Each claim's rectangle is defined as follows:

  The number of inches between the left edge of the fabric and the left edge of the rectangle.
  The number of inches between the top edge of the fabric and the top edge of the rectangle.
  The width of the rectangle in inches.
  The height of the rectangle in inches.
*/
// Rectangle expresses the elf's square of fabric in terms of its cartesian coordinates.
// The origin point (0,0) is the top left point.
#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub struct Rectangle {
    id: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Rectangle {
    // Translates from claims to fabric square representations
    // #123 @ 3,2: 5x4
    pub fn new(claim: &str) -> Self {
        lazy_static! {
            static ref re: Regex = Regex::new(
                r"#(?P<id>\d{1,4}) @ (?P<x>\d+),(?P<y>\d+): (?P<width>\d+)x(?P<height>\d+)"
            )
            .unwrap();
        }
        let cap = re.captures(claim).unwrap();
        let id = cap.name("id").unwrap().as_str().parse::<i32>().unwrap();
        let x = cap.name("x").unwrap().as_str().parse::<i32>().unwrap();
        let y = cap.name("y").unwrap().as_str().parse::<i32>().unwrap();
        let width = cap.name("width").unwrap().as_str().parse::<i32>().unwrap();
        let height = cap.name("height").unwrap().as_str().parse::<i32>().unwrap();
        Rectangle {
            id: id,
            x: x,
            y: y,
            width: width,
            height: height,
        }
    }

    // Calculates the area of this rectangle
    pub fn area(&self) -> i32 {
        self.width * self.height
    }

    // Determines if this rectangle overlaps with the given rectangle
    pub fn overlaps_with(&self, s: &Rectangle) -> bool {
        self.x < s.x + s.width
            && self.x + self.width > s.x
            && self.y < s.y + s.width
            && self.y + self.width > s.y
    }

    // Gets the rectangle where this rectangle overlaps with another, if any
    pub fn intersection(&self, s: &Rectangle) -> Option<Rectangle> {
        if !self.overlaps_with(&s) {
            return None;
        }

        let l = i32::max(self.x, s.x);
        let r = i32::min(self.x + self.width, s.x + s.width);
        let b = i32::max(self.y, s.y);
        let t = i32::min(self.y + self.height, s.y + s.height);

        Some(Rectangle {
            id: -1,
            x: l,
            y: b,
            width: r - l,
            height: t - b,
        })
    }

    // Gets whether the given point is within this rectangle
    pub fn contains(&self, p: &Point) -> bool {
        self.x <= p.x && p.x <= self.x + self.width && self.y <= p.y && p.y <= self.y + self.height
    }

    // Gets points in this rectangle as a flat list
    pub fn points(&self) -> Vec<Point> {
        (self.x..self.x + self.width)
            .map(|i| (self.y..self.y + self.height).map(move |j| Point { x: i.clone(), y: j }))
            .flatten()
            .collect()
    }
}

impl std::fmt::Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Rectangle {}, {}, {}, {}",
            self.x, self.y, self.width, self.height
        )
    }
}

#[derive(Hash, Debug, PartialEq, Eq, Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

#[aoc_generator(day3)]
pub fn fabric_square_generator(input: &str) -> Vec<Rectangle> {
    input.lines().map(|l| Rectangle::new(l)).collect()
}

#[aoc(day3, part1)]
// If the Elves all proceed with their own plans, none of them will have enough
// fabric. How many square inches of fabric are within two or more claims?
pub fn day3_part1_find_overlapped_area(input: &[Rectangle]) -> i32 {
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;

    let mut points_overlap: HashMap<Point, i32> = HashMap::new();

    // Find outer bounds of all the fabric
    input.iter().for_each(|r| {
        min_x = i32::min(min_x, r.x);
        min_y = i32::min(min_y, r.y);
        max_x = i32::max(max_x, r.x);
        max_y = i32::max(max_y, r.y);

        if min_x == 0 {
            min_x = r.x;
        }
        if min_y == 0 {
            min_y = r.y;
        }

        let points = r.points();
        points.iter().for_each(|p| {
            let count = match points_overlap.get(&p) {
                Some(c) => c + 1,
                None => 1,
            };
            points_overlap.insert(*p, count);
        })
    });

    points_overlap.values().filter(|c| **c >= 2).count() as i32
}

#[aoc(day3, part2, find_outlier)]
pub fn day3_part2_find_outlier(input: &[Rectangle]) -> i32 {
    if input.len() == 0 {
        return 0;
    }
    if input.len() == 1 {
        return input[0].id;
    }

    let mut remaining: Vec<Rectangle> = input.to_vec();
    loop {
        let first = remaining[0].clone();
        remaining.remove(0);
        let len = remaining.len();
        remaining = remaining
            .iter()
            .cloned()
            .filter(|r| !first.overlaps_with(&r))
            .collect();

        if remaining.len() == len {
            println!("nothing filtered out, returning: {}", first);
            return first.id;
        }
        println!("filtered out: {} claim", len - remaining.len());

        if remaining.len() == 0 {
            println!("returned: {}", first);
            return first.id;
        }
    }
}

#[aoc(day3, part2, find_outlier2)]
// If the Elves all proceed with their own plans, none of them will have enough
// fabric. How many square inches of fabric are within two or more claims?
pub fn day3_part2_find_outlier2(input: &[Rectangle]) -> i32 {
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;

    let mut point_map: HashMap<Point, i32> = HashMap::new();

    // Find outer bounds of all the fabric
    input.iter().for_each(|r| {
        min_x = i32::min(min_x, r.x);
        min_y = i32::min(min_y, r.y);
        max_x = i32::max(max_x, r.x);
        max_y = i32::max(max_y, r.y);

        if min_x == 0 {
            min_x = r.x;
        }
        if min_y == 0 {
            min_y = r.y;
        }

        let points = r.points();
        points.iter().for_each(|p| {
            let count = match point_map.get(&p) {
                Some(c) => c + 1,
                None => 1,
            };
            point_map.insert(*p, count);
        })
    });

    for r in input {
        let points = r.points();
        if !points.iter().any(|p| {
            let val = point_map.get(p).unwrap();
            val > &1
        }) {
            return r.id;
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn claim_to_fabric_square_test1() {
        let input = "#1349 @ 724,871: 21x26";
        let output = Rectangle {
            id: 1349,
            x: 724,
            y: 871,
            width: 21,
            height: 26,
        };
        assert_eq!(Rectangle::new(&input), output);
    }

    #[test]
    fn claim_to_fabric_square_test2() {
        let input = "#23 @ 771,152: 16x19";
        let output = Rectangle {
            id: 23,
            x: 771,
            y: 152,
            width: 16,
            height: 19,
        };
        assert_eq!(Rectangle::new(&input), output);
    }

    #[test]
    fn claim_to_fabric_square_test3() {
        let input = "#2 @ 102,14: 23x14";
        let output = Rectangle {
            id: 2,
            x: 102,
            y: 14,
            width: 23,
            height: 14,
        };
        assert_eq!(Rectangle::new(&input), output);
    }

    #[test]
    fn intersection_not_exists_test1() {
        let s1 = Rectangle {
            id: 0,
            x: 1,
            y: 3,
            width: 4,
            height: 4,
        };
        let s3 = Rectangle {
            id: 2,
            x: 5,
            y: 6,
            width: 2,
            height: 2,
        };
        assert_eq!(s1.intersection(&s3).is_none(), true);
    }

    #[test]
    fn contains_test1() {
        let s1 = Rectangle {
            id: 0,
            x: 1,
            y: 3,
            width: 4,
            height: 4,
        };
        assert_eq!(s1.contains(&Point { x: 0, y: 0 }), false);
        assert_eq!(s1.contains(&Point { x: 1, y: 3 }), true);
        assert_eq!(s1.contains(&Point { x: 7, y: 7 }), false);
    }

    #[test]
    fn intersection_exists_test1() {
        let s1 = Rectangle {
            id: 0,
            x: 1,
            y: 3,
            width: 4,
            height: 4,
        };
        let s2 = Rectangle {
            id: 1,
            x: 3,
            y: 1,
            width: 4,
            height: 4,
        };
        let s0 = s1.intersection(&s2).unwrap();
        assert_eq!(s0.x, 3);
        assert_eq!(s0.y, 3);
        assert_eq!(s0.height, 2);
        assert_eq!(s0.width, 2);
    }

    /*
      The problem is that many of the claims overlap, causing two or more claims to cover part of the same areas. For example, consider the following claims:

      #1 @ 1,3: 4x4
      #2 @ 3,1: 4x4
      #3 @ 5,5: 2x2

      Visually, these claim the following areas:

      ........
      ...2222.
      ...2222.
      .11XX22.
      .11XX22.
      .111133.
      .111133.
      ........

      The four square inches marked with X are claimed by both 1 and 2. (Claim 3, while adjacent to the others, does not overlap either of them.)
    */

    #[test]
    fn rectangle_points_test() {
        let s1 = Rectangle {
            id: 0,
            x: 1,
            y: 3,
            width: 4,
            height: 4,
        };
        assert_eq!(
            s1.points(),
            vec![
                Point { x: 1, y: 3 },
                Point { x: 1, y: 4 },
                Point { x: 1, y: 5 },
                Point { x: 1, y: 6 },
                Point { x: 2, y: 3 },
                Point { x: 2, y: 4 },
                Point { x: 2, y: 5 },
                Point { x: 2, y: 6 },
                Point { x: 3, y: 3 },
                Point { x: 3, y: 4 },
                Point { x: 3, y: 5 },
                Point { x: 3, y: 6 },
                Point { x: 4, y: 3 },
                Point { x: 4, y: 4 },
                Point { x: 4, y: 5 },
                Point { x: 4, y: 6 },
            ]
        );
    }

    #[test]
    fn overlap_false_test1() {
        let s1 = Rectangle {
            id: 0,
            x: 1,
            y: 3,
            width: 4,
            height: 4,
        };
        let s2 = Rectangle {
            id: 1,
            x: 3,
            y: 1,
            width: 4,
            height: 4,
        };
        let s3 = Rectangle {
            id: 2,
            x: 5,
            y: 5,
            width: 2,
            height: 2,
        };
        assert_eq!(s1.overlaps_with(&s3), false);
        assert_eq!(s2.overlaps_with(&s3), false);
    }

    #[test]
    fn overlap_true_test1() {
        let s1 = Rectangle {
            id: 0,
            x: 1,
            y: 3,
            width: 4,
            height: 4,
        };
        let s2 = Rectangle {
            id: 1,
            x: 3,
            y: 1,
            width: 4,
            height: 4,
        };
        assert_eq!(s1.overlaps_with(&s2), true);
    }

    #[test]
    fn area_test1() {
        assert_eq!(
            Rectangle {
                id: 0,
                x: 0,
                y: 0,
                width: 2,
                height: 2,
            }
            .area(),
            4
        );
    }

    #[test]
    fn part1_overlap_integration_test() {
        let input = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2";
        let output = 4;
        assert_eq!(
            day3_part1_find_overlapped_area(&fabric_square_generator(&input)),
            output
        );
    }

    #[test]
    fn part2_integration_test() {
        let input = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2";
        let output = 3;
        assert_eq!(
            day3_part2_find_outlier(&fabric_square_generator(&input)),
            output
        );
    }

    #[test]
    fn part2_integration_test2() {
        let input = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2";
        let output = 3;
        assert_eq!(
            day3_part2_find_outlier2(&fabric_square_generator(&input)),
            output
        );
    }
}
