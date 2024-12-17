use aoc::solution::{Solution, Solutions};
use regex::Regex;
use std::cmp::max;

struct Ingredient<'a> {
    name: &'a str,
    capacity: isize,
    durability: isize,
    flavor: isize,
    texture: isize,
    calories: isize,
}

pub fn solve15(input: &[String]) -> Solutions {
    let mut ingredients: Vec<Ingredient> = vec![];

    let re = Regex::new(
        r"^(\w+): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)$",
    )
    .unwrap();

    for line in input {
        let captures = re.captures(line).unwrap();

        ingredients.push(Ingredient {
            name: captures.get(1).unwrap().as_str(),
            capacity: captures.get(2).unwrap().as_str().parse::<isize>().unwrap(),
            durability: captures.get(3).unwrap().as_str().parse::<isize>().unwrap(),
            flavor: captures.get(4).unwrap().as_str().parse::<isize>().unwrap(),
            texture: captures.get(5).unwrap().as_str().parse::<isize>().unwrap(),
            calories: captures.get(6).unwrap().as_str().parse::<isize>().unwrap(),
        });
    }

    // Assumed in the loop below.
    assert_eq!("Frosting", ingredients[0].name);
    assert_eq!("Candy", ingredients[1].name);
    assert_eq!("Butterscotch", ingredients[2].name);
    assert_eq!("Sugar", ingredients[3].name);

    // Brute force all possible combinations of number of teaspoons of each ingredient.
    let mut best_score = 0;
    let mut best_score_with_500_calories = 0;

    for frosting in 1..=97 {
        for candy in 1..=98 - frosting {
            for butterscoth in 1..=99 - frosting - candy {
                let sugar = 100 - frosting - candy - butterscoth;
                assert_eq!(100, frosting + candy + butterscoth + sugar);

                let capacity = max(
                    0,
                    frosting * ingredients[0].capacity
                        + candy * ingredients[1].capacity
                        + butterscoth * ingredients[2].capacity
                        + sugar * ingredients[3].capacity,
                );
                let durability = max(
                    0,
                    frosting * ingredients[0].durability
                        + candy * ingredients[1].durability
                        + butterscoth * ingredients[2].durability
                        + sugar * ingredients[3].durability,
                );
                let flavor = max(
                    0,
                    frosting * ingredients[0].flavor
                        + candy * ingredients[1].flavor
                        + butterscoth * ingredients[2].flavor
                        + sugar * ingredients[3].flavor,
                );
                let texture = max(
                    0,
                    frosting * ingredients[0].texture
                        + candy * ingredients[1].texture
                        + butterscoth * ingredients[2].texture
                        + sugar * ingredients[3].texture,
                );
                let calories = max(
                    0,
                    frosting * ingredients[0].calories
                        + candy * ingredients[1].calories
                        + butterscoth * ingredients[2].calories
                        + sugar * ingredients[3].calories,
                );
                let score = capacity * durability * flavor * texture;
                if score > best_score {
                    best_score = score;
                }

                if (calories == 500) && score > best_score_with_500_calories {
                    best_score_with_500_calories = score;
                }
            }
        }
    }

    (
        Solution::ISIZE(best_score),
        Solution::ISIZE(best_score_with_500_calories),
    )
}
