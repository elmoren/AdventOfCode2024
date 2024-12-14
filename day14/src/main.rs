struct Room {
    height: isize,
    width: isize,
    robots: Vec<Robot>,
}

impl Room {
    fn tick(&mut self, ticks: isize) {
        for r in &mut self.robots {
            r.tick(self.width, self.height, ticks);
        }
    }

    fn safety_factor(&self) -> isize {
        let mut zones = (0, 0, 0, 0);

        for r in &self.robots {
            if r.position.x < self.width / 2 && r.position.y < self.height / 2 {
                zones.0 += 1;
            } else if r.position.x >= self.width / 2 + 1 && r.position.y < self.height / 2 {
                zones.1 += 1;
            } else if r.position.x < self.width / 2 && r.position.y >= self.height / 2 + 1 {
                zones.2 += 1;
            } else if r.position.x >= self.width / 2 + 1 && r.position.y >= self.height / 2 + 1 {
                zones.3 += 1;
            } else {
            }
        }

        zones.0 * zones.1 * zones.2 * zones.3
    }

    fn robots_at(&self, x: isize, y: isize) -> usize {
        self.robots.iter().filter(|r| r.position.x == x && r.position.y == y ).count()
    }

    fn print(&self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let c = self.robots_at(x, y);
                if c > 0 {

                    print!("{}", c);
                } else {
                    print!(" ");
                }
            }
            println!();
        }
        println!();
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Robot {
    position: Point,
    vx: isize,
    vy: isize,
}

impl Robot {
    fn tick(&mut self, room_width: isize, room_height: isize, ticks: isize) {
        let x_vel_converted: isize = (self.vx % room_width) + room_width;
        let y_vel_converted: isize = (self.vy % room_height) + room_height;

        self.position.x = (self.position.x + x_vel_converted * ticks) % room_width;
        self.position.y = (self.position.y + y_vel_converted * ticks) % room_height;
    }
}

fn parse_line(line: &str) -> Robot {
    let parts: Vec<isize> = line
        .split([' ', ',', '=', 'p', 'v'])
        .filter(|&s| !s.is_empty())
        .map(|p| p.parse::<isize>().unwrap())
        .collect();

    Robot {
        position: Point {
            x: parts[0],
            y: parts[1],
        },
        vx: parts[2],
        vy: parts[3],
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

fn main() {
    let input = include_str!("../input.txt");

    let mut room = Room {
        height: 103,
        width: 101,
        robots: input
            .lines()
            .map(|l| parse_line(l))
            .collect::<Vec<Robot>>()
    };

    room.tick(100);
    println!("Part 1: {}", room.safety_factor());

}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn test_parse() {
        assert_eq!(
            parse_line("p=0,4 v=3,-3"),
            Robot {
                position: Point { x: 0, y: 4 },
                vx: 3,
                vy: -3
            }
        );
    }

    #[test]
    fn test_movement_robot_tick() {
        let mut r = Robot {
            position: Point { x: 6, y: 3 },
            vx: -1,
            vy: -3,
        };
        r.tick(7, 11, 1);
        assert_eq!(r.position, Point { x: 5, y: 0 });
    }

    #[test]
    fn test_sample_with_100_ticks() {
        let mut room = Room {
            height: 7,
            width: 11,
            robots: INPUT
                .lines()
                .map(|l| parse_line(l))
                .collect::<Vec<Robot>>()
        };

        room.tick(100);
        assert_eq!(room.robots_at(6, 0), 2);
        assert_eq!(room.robots_at(9, 0), 1);
        assert_eq!(room.robots_at(0, 2), 1);
        assert_eq!(room.robots_at(1, 3), 1);
        assert_eq!(room.robots_at(2, 3), 1);
        assert_eq!(room.robots_at(5, 4), 1);
        assert_eq!(room.robots_at(3, 5), 1);
        assert_eq!(room.robots_at(4, 5), 2);
        assert_eq!(room.robots_at(1, 6), 1);
        assert_eq!(room.robots_at(6, 6), 1);

        println!("Robots: {:#?}", room.robots);
        assert_eq!(room.safety_factor(), 12);
    }

}
