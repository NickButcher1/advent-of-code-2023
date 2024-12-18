use regex::Regex;
use std::cmp::max;

#[derive(Clone, Debug)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

// Read input where each line is one point, formatted "1,2".
pub fn read_points(input: &[String]) -> Vec<Point> {
    let re = Regex::new(r"^(-?\d+),(-?\d+)$").unwrap();

    input
        .iter()
        .map(|line| {
            let captures = re.captures(line).unwrap();
            Point {
                x: captures.get(1).unwrap().as_str().parse::<isize>().unwrap(),
                y: captures.get(2).unwrap().as_str().parse::<isize>().unwrap(),
            }
        })
        .collect()
}

// Read input where each line is a pair of points, formatted "1,2 -> 3,4".
pub fn read_point_pairs(input: &[String]) -> Vec<(Point, Point)> {
    let re = Regex::new(r"^(-?\d+),(-?\d+) -> (-?\d+),(-?\d+)$").unwrap();

    input
        .iter()
        .map(|line| {
            let captures = re.captures(line).unwrap();
            (
                Point {
                    x: captures.get(1).unwrap().as_str().parse::<isize>().unwrap(),
                    y: captures.get(2).unwrap().as_str().parse::<isize>().unwrap(),
                },
                Point {
                    x: captures.get(3).unwrap().as_str().parse::<isize>().unwrap(),
                    y: captures.get(4).unwrap().as_str().parse::<isize>().unwrap(),
                },
            )
        })
        .collect()
}

pub fn max_x_y(points: &[Point]) -> (isize, isize) {
    points.iter().fold((0, 0), |(max_x, max_y), point| {
        (max(max_x, point.x), max(max_y, point.y))
    })
}

pub fn max_x_y_for_pairs(points: &[(Point, Point)]) -> (isize, isize) {
    points
        .iter()
        .fold((0, 0), |(max_x, max_y), (point_from, point_to)| {
            (
                max(max(max_x, point_from.x), point_to.x),
                max(max(max_y, point_from.y), point_to.y),
            )
        })
}
