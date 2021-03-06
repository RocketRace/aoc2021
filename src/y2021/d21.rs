use std::collections::HashMap;

fn to_digit(input: &str) -> usize {
    if input.len() == 30 {
        9
    }
    else {
        (input.as_bytes().last().unwrap() - b'0' - 1) as usize
    }
}

#[aoc_generator(day21)]
fn positions(input: &str) -> (usize, usize) {
    let (a, b) = input.split_once("\n").unwrap();
    (to_digit(a), to_digit(b))
}

#[aoc(day21, part1)]
fn deterministic_die(&(a_start, b_start): &(usize, usize)) -> usize {
    // Player A's moves:
    // 1 + 2 + 3, 7 + 8 + 9, 13 + 14 + 15, 19 + 20 + 21, ...
    // 6,         24,        42,           60            ...
    // Player B's moves:
    // 4 + 5 + 6, 10 + 11 + 12, 16 + 17 + 18, ...
    // 15,        33,           51,           ...
    //
    // Modulo 10, we get
    // A: 6, 4, 2, 0, 8, ...
    // B: 5, 3, 1, 9, 7, ...
    // 
    // While the dice roll around from 100 to 1, and this does in
    // fact influence the sum of the dice, this does not influence
    // the residue of the sum modulo 10. Therefore, we can assume
    // that this pattern continues indefinitely.
    //
    // If A begins at tile N, they visit
    // (N, ) N + 6, N, N + 2, N + 2, N, ...
    // repeating, mod 10. 
    //
    // If B begins at tile M, they visit
    // (M, ) M + 5, M + 8, M + 9, M + 8, M + 5, M, M + 3, M + 4, M + 5, M
    // repeating, mod 10.
    //
    // We take the sum of 10-length cycles for every possible value of N, M.
    // Note that every tile has an extra score of 1, so possible scores
    // range from 1 to 10 rather than 0 to 9.
    // A: 
    // 7, 1, 3, 3, 1   x2 => 30
    // 8, 2, 4, 4, 2   x2 => 40
    // 9, 3, 5, 5, 3   x2 => 50
    // 10, 4, 6, 6, 4  x2 => 60
    // 1, 5, 7, 7, 5   x2 => 50
    // 2, 6, 8, 8, 6   x2 => 60
    // 3, 7, 9, 9, 7   x2 => 70
    // 4, 8, 10, 10, 8 x2 => 80
    // 5, 9, 1, 1, 9   x2 => 50
    // 6, 10, 2, 2, 10 x2 => 60
    // 
    // B: 
    // 6, 9, 10, 9, 6, 1, 4, 5, 4, 1  => 55
    // 7, 10, 1, 10, 7, 2, 5, 6, 5, 2 => 55
    // 8, 1, 2, 1, 8, 3, 6, 7, 6, 3   => 45
    // 9, 2, 3, 2, 9, 4, 7, 8, 7, 4   => 55
    // 10, 3, 4, 3, 10, 5, 8, 9, 8, 5 => 65
    // 1, 4, 5, 4, 1, 6, 9, 10, 9, 6  => 55
    // 2, 5, 6, 5, 2, 7, 10, 1, 10, 7 => 55
    // 3, 6, 7, 6, 3, 8, 1, 2, 1, 8   => 45
    // 4, 7, 8, 7, 4, 9, 2, 3, 2, 9   => 55
    // 5, 8, 9, 8, 5, 10, 3, 4, 3, 10 => 65
    //
    // Note: while both players gain on average 55 points per cycle,
    // B is more consistently at 55 while A's scores are further
    // from the mean.
    //
    // Given these cycles, we can count the Kth full cycle after which
    // one or both of A, B surpass 1000 points for every possible
    // starting square.
    //
    // A: 34, 25, 20, 17, 20, 17, 15, 13, 20, 17
    // B: 19, 19, 23, 19, 16, 19, 19, 23, 19, 16
    //
    // We can also determine the exact dice roll that the player
    // crossed the finish line by taking the approximate cycle,
    // and simply stepping back until we find the tipping point.
    // Notice that both A and B's dice rolls are considered for 
    // the total count. This is because each player rolls after the
    // other. Every complete cycle (in which both players move 10 times)
    // contains exactly 60 dice rolls.
    // 
    // A:
    // 1:  33 * 60 + 7  * 3 = 2001
    // 2:  24 * 60 + 19 * 3 = 1497
    // 3:  19 * 60 + 19 * 3 = 1197
    // 4:  16 * 60 + 13 * 3 = 999
    // 5:  19 * 60 + 19 * 3 = 1197
    // 6:  16 * 60 + 13 * 3 = 999
    // 7:  14 * 60 + 7  * 3 = 861
    // 8:  12 * 60 + 9  * 3 = 747
    // 9:  19 * 60 + 19 * 3 = 1197
    // 10: 16 * 60 + 13 * 3 = 999
    //
    // B:
    // 1:  18 * 60 + 2  * 3 = 1086
    // 2:  18 * 60 + 2  * 3 = 1086
    // 3:  22 * 60 + 6  * 3 = 1338
    // 4:  18 * 60 + 2  * 3 = 1086
    // 5:  15 * 60 + 8  * 3 = 924
    // 6:  18 * 60 + 2  * 3 = 1086
    // 7:  18 * 60 + 2  * 3 = 1086
    // 8:  22 * 60 + 6  * 3 = 1338
    // 9:  18 * 60 + 2  * 3 = 1086
    // 10: 15 * 60 + 8  * 3 = 924
    // 
    // The winner given N, M is the player with a lower roll count in the 
    // table above. We can then compute the score that each player has for
    // each of the opponent's winning die roll count, for every possible 
    // combination of N and M. For brevity's sake, I haven't included
    // all the computations below.
    //
    // N,  M:  Roll * Loser score
    // 1,  1:  1086 * 547 = 594,042
    // 1,  2:  1086 * 547 = 594,042
    // 1,  3:  1338 * 671 = 897,798
    // ...
    // 10, 10: 924  * 920 = 850,080
    // 
    // We finally place these values in a lookup table. Mmm, performance
    //
    // Generalizations: 
    // - We can generalize on the number of steps (from 10) as long as the 
    // maximum dice roll (here 100) is set to be divisible by the step count. An
    // updated LUT can be derived using this same algorithm as long as this
    // constraint holds.
    // - We can generalize on the winning score (from 1000). This will update 
    // the full & partial cycles at which each player wins. The rest of the 
    // algorithm is unchanged, using these new values.
    // - We can probably generalize on the number of players (from 2). The winner is
    // the player with the minimum rolls until the win condition, and the loser
    // is the player with the maximum rolls until the win condition. The roll counts
    // will also have to be updated, keeping in mind the order of players.
    let i = a_start * 10 + b_start;
    [
        594_042, 594_042,   897_798, 594_042, 428_736, 594_042, 594_042,   897_798, 594_042, 428_736,
        790_608, 790_608, 1_196_172, 790_608, 571_032, 790_608, 790_608, 1_196_172, 790_608, 571_032,
        987_174, 987_174, 1_073_709, 987_174, 713_328, 987_174, 987_174, 1_067_724, 987_174, 713_328,
        920_079, 916_083,   742_257, 908_091, 855_624, 900_099, 906_093,   752_247, 918_081, 855_624,
        978_486, 978_486, 1_073_709, 978_486, 711_480, 978_486, 978_486, 1_067_724, 978_486, 711_480,
        920_079, 916_083,   742_257, 908_091, 853_776, 900_099, 906_093,   752_247, 918_081, 853_776,
        684_495, 678_468,   551_901, 675_024, 798_147, 671_580, 674_163,   556_206, 679_329, 802_452,
        518_418, 513_936,   412_344, 504_972, 597_600, 503_478, 506_466,   419_814, 512_442, 605_070,
        982_830, 982_830, 1_073_709, 982_830, 707_784, 982_830, 982_830, 1_067_724, 982_830, 707_784,
        920_079, 916_083,   742_257, 908_091, 850_080, 900_099, 906_093,   752_247, 918_081, 850_080,
    ][i]
}

#[aoc(day21, part2)]
fn quantum_dice(&(a_start, b_start): &(usize, usize)) -> usize {
    // This one is interesting!
    // 
    // Some bounds:
    // A game lasts at least 3 turns and at most 21.
    // if all games are the maximum length, there would be
    // ~ 10^20 games. Too many to brute force, obviously.
    //
    // Because the game tree depth is variable, it's 
    // not really feasible to generate a LUT like for the
    // first part. At least it'd take more than a few hours
    // of pondering from me.

    // The number of winning universes of A at roll 1 is the sum of
    // winning universes of A given every possible output of the
    // dice rolls. The number of winning universes of A at the
    // roll at which one player wins is either 0 or 1. This gives
    // a pretty robust base on which to exercise dynamic programming.
    let mut cache = HashMap::new();
    let (a, b) = win_counts(&mut cache, (0, 0, a_start, b_start, true));
    a.max(b)
}

type State = (usize, usize, usize, usize, bool);
type Counts = (usize, usize);

// Rolling three dice 1..=3 gives the following binomial
// distribution for sums. This is used to reduce the search 
// space; no need to check each of the 27 possible outcomes 
// when you can check 7 and multiply accordingly.
const THREE_ROLL_DISTRIBUTION: [usize; 7] = [1, 3, 6, 7, 6, 3, 1];

// The current scores & positions of players uniquely identifies the current state.
// We assume that recursion is done at full-turn intervals.
fn win_counts(cache: &mut HashMap<State, Counts>, state @ (a_score, b_score, a_pos, b_pos, first_half): State) -> Counts {
    if cache.contains_key(&state) {
        cache[&state]
    }
    else {
        let result = {
            // A gets priority since they roll first
            if a_score >= 21 {
                (1, 0)
            }
            else if b_score >= 21 {
                (0, 1)
            }
            else {
                if first_half {
                    (3..=10)
                        .zip(THREE_ROLL_DISTRIBUTION.into_iter())
                        .fold((0, 0), |(a_c, b_c), (sum, weight)| {
                            let new_pos = (a_pos + sum) % 10;
                            let (a_cn, b_cn) = win_counts(cache, (a_score + new_pos + 1, b_score, new_pos, b_pos, false));
                            (a_c + weight * a_cn, b_c + weight * b_cn)
                        })
                    }
                else {
                    (3..=10)
                        .zip(THREE_ROLL_DISTRIBUTION.into_iter())
                        .fold((0, 0), |(a_c, b_c), (sum, weight)| {
                            let new_pos = (b_pos + sum) % 10;
                            let (a_cn, b_cn) = win_counts(cache, (a_score, b_score + new_pos + 1, a_pos, new_pos, true));
                            (a_c + weight * a_cn, b_c + weight * b_cn)
                        })
                }
            }
        };
        cache.insert(state, result);
        result
    }
}

#[aoc(day21, part2, cheating)]
fn cheating(&(a_start, b_start): &(usize, usize)) -> usize {
    let i = 10 * a_start + b_start;
    [
         32_491_093_007_709,  27_674_034_218_179,  48_868_319_769_358,  97_774_467_368_562, 13_850_8043_837_521,   157_253_621_231_420, 141_740_702_114_011, 115_864_149_937_553,  85_048_040_806_299,  57_328_067_654_557, 
         27_464_148_626_406,  24_411_161_361_207,  45_771_240_990_345,  93_049_942_628_388, 13_188_8061_854_776,   149_195_946_847_792, 133_029_050_096_658, 106_768_284_484_217,  76_262_326_668_116,  49_975_322_685_009, 
         51_863_007_694_527,  45_198_749_672_670,  93_013_662_727_308, 193_753_136_998_081, 27_506_7741_811_212,   309_991_007_938_181, 273_042_027_784_929, 214_368_059_463_212, 147_573_255_754_448,  92_399_285_032_143, 
        110_271_560_863_819,  91_559_198_282_731, 193_170_338_541_590, 404_904_579_900_696, 57_511_1835_924_670,   647_608_359_455_719, 568_867_175_661_958, 444_356_092_776_315, 303_121_579_983_974, 187_451_244_607_486, 
        156_667_189_442_502, 129_742_452_789_556, 274_195_599_086_465, 575_025_114_466_224, 81_680_0855_030_343,   919_758_187_195_363, 807_873_766_901_514, 630_947_104_784_464, 430_229_563_871_565, 265_845_890_886_828, 
        175_731_756_652_760, 146_854_918_035_875, 309_196_008_717_909, 647_920_021_341_197, 92_034_2039_518_611, 1_036_584_236_547_450, 911_090_395_997_650, 712_381_680_443_927, 486_638_407_378_784, 301_304_993_766_094, 
        152_587_196_649_184, 131_180_774_190_079, 272_847_859_601_291, 570_239_341_223_618, 80_995_3813_657_517,   912_857_726_749_764, 803_934_725_594_806, 630_797_200_227_453, 433_315_766_324_816, 270_005_289_024_391, 
        116_741_133_558_209, 105_619_718_613_031, 214_924_284_932_572, 446_968_027_750_017, 63_476_9613_696_613,   716_241_959_649_754, 632_979_211_251_440, 499_714_329_362_294, 346_642_902_541_848, 218_433_063_958_910, 
         83_778_196_139_157,  75_823_864_479_001, 148_747_830_493_442, 306_621_346_123_766, 43_528_8918_824_107,   492_043_106_122_795, 437_256_456_198_320, 348_577_682_881_276, 245_605_000_281_051, 157_595_953_724_471, 
         56_852_759_190_649,  49_982_165_861_983,  93_726_416_205_179, 190_897_246_590_017, 27_080_3396_243_039,   306_719_685_234_774, 274_291_038_026_362, 221_109_915_584_112, 158_631_174_219_251, 104_001_566_545_663
    ][i]
}