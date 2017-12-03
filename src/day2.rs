//  --- Day 2: Corruption Checksum ---
//
//  As you walk through the door, a glowing humanoid shape yells in your direction. "You there! Your state appears
//  to be idle. Come help us repair the corruption in this spreadsheet - if we take another millisecond, we'll have
//  to display an hourglass cursor!"
//
//  The spreadsheet consists of rows of apparently-random numbers. To make sure the recovery process is on the
//  right track, they need you to calculate the spreadsheet's checksum. For each row, determine the difference
//  between the largest value and the smallest value; the checksum is the sum of all of these differences.
//
//  For example, given the following spreadsheet:
//
//  5 1 9 5
//  7 5 3
//  2 4 6 8
//  The first row's largest and smallest values are 9 and 1, and their difference is 8.
//  The second row's largest and smallest values are 7 and 3, and their difference is 4.
//  The third row's difference is 6.
//  In this example, the spreadsheet's checksum would be 8 + 4 + 6 = 18.
//
//  What is the checksum for the spreadsheet in your puzzle input?
//
//  --- Part Two ---
//
//  "Great work; looks like we're on the right track after all. Here's a star for your effort."
//  However, the program seems a little worried. Can programs be worried?
//
//  "Based on what we're seeing, it looks like all the User wanted is some information about
//  the evenly divisible values in the spreadsheet. Unfortunately, none of us are equipped for
//  that kind of calculation - most of us specialize in bitwise operations."
//
//  It sounds like the goal is to find the only two numbers in each row where one evenly divides
//  the other - that is, where the result of the division operation is a whole number. They would
//  like you to find those numbers on each line, divide them, and add up each line's result.
//
//  For example, given the following spreadsheet:
//
//  5 9 2 8
//  9 4 7 3
//  3 8 6 5
//  In the first row, the only two numbers that evenly divide are 8 and 2; the result of this division is 4.
//  In the second row, the two numbers are 9 and 3; the result is 3.
//  In the third row, the result is 2.
//  In this example, the sum of the results would be 4 + 3 + 2 = 9.
//
//  What is the sum of each row's result in your puzzle input?

use lib::Puzzle;

pub struct Day2A {}

pub struct Day2B {}

impl Puzzle for Day2A {
    fn name(&self) -> String {
        "Day 2: Corruption Checksum 1".to_string()
    }
    fn solution(&self) -> String {
        calculate_checksum(INPUT, &max_min_checksum).to_string()
    }
}

impl Puzzle for Day2B {
    fn name(&self) -> String {
        "Day 2: Corruption Checksum 2".to_string()
    }
    fn solution(&self) -> String {
        calculate_checksum(INPUT, &cleanly_divisible_checksum).to_string()
    }
}


static INPUT: &'static str = r#"
179	2358	5197	867	163	4418	3135	5049	187	166	4682	5080	5541	172	4294	1397
2637	136	3222	591	2593	1982	4506	195	4396	3741	2373	157	4533	3864	4159	142
1049	1163	1128	193	1008	142	169	168	165	310	1054	104	1100	761	406	173
200	53	222	227	218	51	188	45	98	194	189	42	50	105	46	176
299	2521	216	2080	2068	2681	2376	220	1339	244	605	1598	2161	822	387	268
1043	1409	637	1560	970	69	832	87	78	1391	1558	75	1643	655	1398	1193
90	649	858	2496	1555	2618	2302	119	2675	131	1816	2356	2480	603	65	128
2461	5099	168	4468	5371	2076	223	1178	194	5639	890	5575	1258	5591	6125	226
204	205	2797	2452	2568	2777	1542	1586	241	836	3202	2495	197	2960	240	2880
560	96	336	627	546	241	191	94	368	528	298	78	76	123	240	563
818	973	1422	244	1263	200	1220	208	1143	627	609	274	130	961	685	1318
1680	1174	1803	169	450	134	3799	161	2101	3675	133	4117	3574	4328	3630	4186
1870	3494	837	115	1864	3626	24	116	2548	1225	3545	676	128	1869	3161	109
890	53	778	68	65	784	261	682	563	781	360	382	790	313	785	71
125	454	110	103	615	141	562	199	340	80	500	473	221	573	108	536
1311	64	77	1328	1344	1248	1522	51	978	1535	1142	390	81	409	68	352
"#;

fn calculate_checksum(spreadsheet: &str, line_func: &Fn(Vec<u32>) -> u32) -> u32 {
    let lines: Vec<&str> = spreadsheet.split("\n").collect();

    lines.iter()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.split("\t").collect::<Vec<&str>>())
        .map(|splat_line| {
            splat_line.into_iter()
                .map(|s| s.trim().parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .fold(0, |acc, curr| {
            acc + line_func(curr)
        })
}

fn max_min_checksum(vec: Vec<u32>) -> u32 {
    let max = vec.iter().max().unwrap(); // unsafe
    let min = vec.iter().min().unwrap(); // unsafe

    max - min
}

fn cleanly_divisible_checksum(mut vec: Vec<u32>) -> u32 {
    vec.sort_by(|a, b| b.cmp(a));

    let mut matches: Vec<u32> = vec![];

    for i in 0..vec.len() {
        for j in (i + 1)..vec.len() {
            if vec[i] % vec[j] == 0 {
                matches.push(vec[i] / vec[j]);
            }
        }
    }

    matches.iter().fold(0, |acc, curr| acc + curr)
}

#[cfg(test)]
mod day2test {
    use day2::calculate_checksum;
    use day2::max_min_checksum;
    use day2::cleanly_divisible_checksum;

    #[test]
    fn test_spreadsheet_checksum_based_on_max_minus_min_per_line() {
        let input = r#"
            5	1	9	5
            7	5	3
            2	4	6	8
            "#;

        let result = calculate_checksum(input, &max_min_checksum);

        assert_eq!(result, 18);
    }

    #[test]
    fn test_spreadsheet_checksum_based_on_division_of_cleanly_divisible_row_members() {
        let input = r#"
            5	9	2	8
            9	4	7	3
            3	8	6	5
            "#;

        let result = calculate_checksum(input, &cleanly_divisible_checksum);

        assert_eq!(result, 9);
    }
}