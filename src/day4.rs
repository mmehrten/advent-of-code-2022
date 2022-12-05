use crate::helpers::Solution;

struct Range<T: Ord> {
    x1: T,
    x2: T,
}

impl<T: Ord> Range<T> {
    fn fully_overlaps(&self, other: &Range<T>) -> bool {
        self.x1 <= other.x1 && self.x2 >= other.x2
    }

    fn partially_overlaps(&self, other: &Range<T>) -> bool {
        self.x1 <= other.x1 && other.x1 <= self.x2
    }
}
pub struct Day4;

impl Day4 {
    fn parse_ranges(&self, data: Vec<String>) -> Vec<Range<i32>> {
        let points = data
            .iter()
            .map(|pair| pair.split(","))
            .flatten()
            .map(|range| range.split("-"))
            .flatten()
            .map(|point| point.parse::<i32>().expect("Failed to parse point as i32"))
            .collect::<Vec<i32>>();
        let mut ranges = Vec::new();
        for idx in (0..points.len()).step_by(2) {
            let range = Range {
                x1: points[idx],
                x2: points[idx + 1],
            };
            ranges.push(range);
        }
        ranges
    }
}

impl Solution for Day4 {
    /// Returns the number of ranges that fully overlap given a list of pairs of ranges.
    ///
    /// For example, 1-4,2-3 fully overlaps, because 1-4 fully encloses 2-3.
    /// However, 1-4,4-6 does not fully overlap.
    fn part_one(&self, data: Vec<String>) -> String {
        let ranges = self.parse_ranges(data);
        let mut overlap_count = 0;
        for idx in (0..ranges.len()).step_by(2) {
            let r1 = &ranges[idx];
            let r2 = &ranges[idx + 1];

            if r1.fully_overlaps(r2) || r2.fully_overlaps(r1) {
                overlap_count += 1;
            }
        }
        overlap_count.to_string()
    }

    /// Returns the number of ranges that overlap at all given a list of pairs of ranges.
    ///
    /// For example, 1-4,2-3 overlaps, because 1-4 fully encloses 2-3.
    /// 1-4,4-6 overlaps on 4.
    /// 1-4,5-6 does not overlap.
    fn part_two(&self, data: Vec<String>) -> String {
        let ranges = self.parse_ranges(data);
        let mut overlap_count = 0;
        for idx in (0..ranges.len()).step_by(2) {
            let r1 = &ranges[idx];
            let r2 = &ranges[idx + 1];

            if r1.partially_overlaps(r2) || r2.partially_overlaps(r1) {
                overlap_count += 1;
            }
        }
        overlap_count.to_string()
    }
}

#[cfg(test)]
mod test_solution {
    use crate::day4::Day4;
    use crate::helpers::Solution;
    use crate::helpers::{get_input, InputType};

    #[test]
    fn example_data() {
        let day = Day4;
        let data = get_input(4, InputType::Example);
        assert_eq!(day.part_one(data.clone()), "2".to_string());
        assert_eq!(day.part_two(data), "4".to_string());
    }
}
