use aoc::board::Board;

pub fn solve17(_input: &[String]) -> (i128, i128) {
    // TODO
    // let re = Regex::new(r"^target area: x=(\d+)..(\d+), y=-(\d+)..(\d+)").unwrap();
    // let caps = re.captures(&input[0]).unwrap();

    let border: isize = 30;
    let x_min: isize = 20;
    let x_max: isize = 30;
    let y_min: isize = -10;
    let y_max: isize = -5;

    let mut board = Board::create_empty(
        (x_max - x_min + border * 2) as usize,
        (y_max - y_min + border * 2) as usize,
        '.',
    );

    let mut probe = (0, 0);
    board.cells[(probe.1 + border) as usize][(probe.0 + border) as usize] = 'S';

    for x in x_min..=x_max {
        for y in y_min..=y_max {
            board.cells[(y + border) as usize][(x + border) as usize] = 'T';
        }
    }

    // Board is upside down.
    // board.print();

    let mut velocity = (7, 2);

    loop {
        // Show previous probe position.
        board.cells[(probe.1 + border) as usize][(probe.0 + border) as usize] = '#';

        // Move probe and redraw it.
        probe = (probe.0 + velocity.0, probe.1 + velocity.1);
        if board.cells[(probe.1 + border) as usize][(probe.0 + border) as usize] == 'T' {
            board.cells[(probe.1 + border) as usize][(probe.0 + border) as usize] = 'S';
            // board.print();
            break;
        }
        board.cells[(probe.1 + border) as usize][(probe.0 + border) as usize] = 'S';

        // Due to drag, the probe's x velocity changes by 1 toward the value 0; that is, it decreases by 1 if it is greater than 0, increases by 1 if it is less than 0, or does not change if it is already 0.
        if velocity.0 > 0 {
            velocity = (velocity.0 - 1, velocity.1);
        } else if velocity.0 < 0 {
            velocity = (velocity.0 + 1, velocity.1);
        }
        // Due to gravity, the probe's y velocity decreases by 1.
        velocity = (velocity.0, velocity.1 - 1);

        // board.print();
    }

    (0_i128, 0_i128)
}
