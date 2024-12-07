# :gift::christmas_tree: Advent of Code 2024 :christmas_tree::sparkles:

These are my solutions to this year's [Advent of Code](https://adventofcode.com/2024/).

Solutions make use of `cargo-aoc` code helper ([here](https://github.com/gobanos/cargo-aoc)).

## Solutions

All solutions linked below:
| Day | Title | 1 :star: | 2 :star: | Solution | Rating |
|:-|:-|:-|:-|:-|:-|
| [01](https://adventofcode.com/2024/day/1)  | Historian Hysteria              | 60.3µs | 66.5µs | [day01.rs](./src/day01.rs) | :christmas_tree: |
| [02](https://adventofcode.com/2024/day/2)  | Red-Nosed Reports               | 20.1µs | 98.8µs | [day02.rs](./src/day02.rs) | :grin::sweat:    |
| [03](https://adventofcode.com/2024/day/3)  | Mull it Over                    | 1.41µs | 2.28µs | [day03.rs](./src/day03.rs) | :sob:            |
| [04](https://adventofcode.com/2024/day/4)  | Ceres Search                    | 1.24ms | 2.56ms | [day04.rs](./src/day04.rs) | :woozy_face:     |
| [05](https://adventofcode.com/2024/day/5)  | Print Queue                     | 253µs  | 474µs  | [day05.rs](./src/day05.rs) | :sunglasses::frowning_face: |
| [06](https://adventofcode.com/2024/day/6)  | Guard Gallivant                 | 1.35ms | 1.775s | [day06.rs](./src/day06.rs) | :hugs::cursing_face:        |
| [07](https://adventofcode.com/2024/day/7)  | Bridge Repair                   | 1.70ms | 103ms  | [day07.rs](./src/day07.rs) | :smiling_face_with_three_hearts: |

## Notes
1. Day 01 part 2 was improved, initially giving a result of 69.69µs
2. Two solutions for day 2 - one arithmetic, and another using a state machine. Both are comparable.
3. Regex was an obvious goto for this, but I really detest using it
4. Some hits for day part 1 were palindromes, and part 2 required only diagonal crosses
5. I had quite a few false positives for day 5 part 2 when trying `easier' solutions
6. Day 6 part 1 was fairly straightforward, but part 2 kept encountering false positives from 8 attempted solutions.
7. Day 7... good old ADS classes. It could be a bit faster though. *Now 10x faster*