use regex::Regex;
use std::cmp::{max, min};

#[derive(Clone, Debug)]
pub struct MovingPoint {
    pub px: isize,
    pub py: isize,
    pub vx: isize,
    pub vy: isize,
}

// Read input where each line is one point, formatted "position=< 1,  2> velocity=< 3,  4>".
pub fn read_points(input: &[String]) -> Vec<MovingPoint> {
    let re = Regex::new(r"^position=<( *)(-?\d+),( *)(-?\d+)> velocity=<( *)(-?\d+),( *)(-?\d+)>$")
        .unwrap();

    input
        .iter()
        .map(|line| {
            let captures = re.captures(line).unwrap();
            MovingPoint {
                px: captures.get(2).unwrap().as_str().parse::<isize>().unwrap(),
                py: captures.get(4).unwrap().as_str().parse::<isize>().unwrap(),
                vx: captures.get(6).unwrap().as_str().parse::<isize>().unwrap(),
                vy: captures.get(8).unwrap().as_str().parse::<isize>().unwrap(),
            }
        })
        .collect()
}

pub fn min_x_y(points: &[MovingPoint]) -> (isize, isize) {
    points
        .iter()
        .fold((isize::MAX, isize::MAX), |(min_x, min_y), point| {
            (min(min_x, point.px), min(min_y, point.py))
        })
}

pub fn max_x_y(points: &[MovingPoint]) -> (isize, isize) {
    points.iter().fold((0, 0), |(max_x, max_y), point| {
        (max(max_x, point.px), max(max_y, point.py))
    })
}

pub fn tick(points: &mut [MovingPoint]) {
    points.iter_mut().for_each(|point| {
        point.px += point.vx;
        point.py += point.vy;
    });
}
