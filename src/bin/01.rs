advent_of_code::solution!(1);

pub struct CircularRange {
    start: i64,
    end: i64,
    current: i64
}

impl CircularRange {
    pub fn new(start: i64, end: i64, current: i64) -> Self
    {
        CircularRange { start, end, current}
    }

    pub fn prev_v1(&mut self, num: i64) -> u64{
        for _i in 1..=num {
            if self.current == self.start {
                self.current = self.end
            }
            else {
                self.current -= 1
            }
        }

        if self.current == self.start
        {
            1
        }
        else {
            0
        }
    }
    
    pub fn next_v1(&mut self, num: i64) -> u64{
        for _i in 1..=num {
            if self.current == self.end {
                self.current = self.start
            }
            else {
                self.current += 1
            }
        }

        if self.current == self.start
        {
            1
        }
        else {
            0
        }
    }

    pub fn move_by_v1(&mut self, instruction: &str) -> u64 {
        if instruction.starts_with("L") 
        {
            //println!("{} {}", self.current, instruction);
            self.prev_v1(instruction[1..].parse::<i64>().unwrap())
        }
        else if instruction.starts_with("R") 
        {
            //println!("{} {}", self.current, instruction);
            self.next_v1(instruction[1..].parse::<i64>().unwrap())
        }
        else {
            panic!("unkown instruction")
        }
    }

    pub fn prev_v2(&mut self, num: i64) -> u64{
        let mut total: u64 = 0;

        for _i in 1..=num {
            if self.current == self.start {
                self.current = self.end
            }
            else {
                self.current -= 1
            }

            if self.current == self.start
            {
                total += 1
            }
        }

        total
    }
    
    pub fn next_v2(&mut self, num: i64) -> u64{
        let mut total: u64 = 0;

        for _i in 1..=num {
            if self.current == self.end {
                self.current = self.start
            }
            else {
                self.current += 1
            }

            if self.current == self.start
            {
                total += 1
            }
        }

        total
    }

    pub fn move_by_v2(&mut self, instruction: &str) -> u64 {
        if instruction.starts_with("L") 
        {
            //println!("{} {}", self.current, instruction);
            self.prev_v2(instruction[1..].parse::<i64>().unwrap())
        }
        else if instruction.starts_with("R") 
        {
            //println!("{} {}", self.current, instruction);
            self.next_v2(instruction[1..].parse::<i64>().unwrap())
        }
        else {
            panic!("unkown instruction")
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut total: u64 = 0;

    let mut range = CircularRange::new(0, 99, 50);

    for line in input.lines() {
        
        total += range.move_by_v1(line);
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut total: u64 = 0;

    let mut range = CircularRange::new(0, 99, 50);

    for line in input.lines() {
        
        total += range.move_by_v2(line);
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
