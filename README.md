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
| [08](https://adventofcode.com/2024/day/8)  | Resonant Collinearity           | 58.1µs | 191µs  | [day08.rs](./src/day08.rs) | :nerd_face:      |
| [09](https://adventofcode.com/2024/day/9)  | Disk Fragmenter                 | 1.43ms | 40.8ms | [day09.rs](./src/day09.rs) | :relaxed::woozy_face: |
| [10](https://adventofcode.com/2024/day/10) | Hoof It                         | 757µs  | 182µs  | [day10.rs](./src/day10.rs) | :heart_eyes::partying_face: |
| [11](https://adventofcode.com/2024/day/11) | Plutonian Pebbles               | 341µs  | 13.0ms | [day11.rs](./src/day11.rs) | :cursing_face:   |
| [12](https://adventofcode.com/2024/day/12) | Garden Groups                   | 444µs  | 3.69ms | [day12.rs](./src/day12.rs) | :sob:            |
| [13](https://adventofcode.com/2024/day/13) | Claw Contraption                | 11.2µs | 11.8µs | [day13.rs](./src/day13.rs) | :nerd_face:      |
| [14](https://adventofcode.com/2024/day/14) | Restroom Redoubt                | 6.55µs | 48.7ms | [day14.rs](./src/day14.rs) | :relaxed::face_with_spiral_eyes: |
| [15](https://adventofcode.com/2024/day/15) | Warehouse Woes                  | 363µs  | 2.56ms | [day15.rs](./src/day15.rs) | :cry:            |
| [16](https://adventofcode.com/2024/day/16) | Reindeer Maze                   | 10.7ms | 29.0ms | [day16.rs](./src/day16.rs) | :weary:          |
| [17](https://adventofcode.com/2024/day/17) | Chronospatial Computer          | 3.32µs | 181µs  | [day17.rs](./src/day17.rs) | :smile::scream:  |
| [18](https://adventofcode.com/2024/day/18) | RAM Run                         | 2.47ms | 39.5ms | [day18.rs](./src/day18.rs) | :zany_face:      |
| [19](https://adventofcode.com/2024/day/19) | Linen Layout                    | 69.9ms | 61.9ms | [day19.rs](./src/day19.rs) | :thinking:       |
| [20](https://adventofcode.com/2024/day/20) | Race Condition                  | 4.97ms | 132ms  | [day20.rs](./src/day20.rs) | :smirk:          |
| [21](https://adventofcode.com/2024/day/21) | Keypad Conundrum                | 15.5µs | 193µs  | [day21.rs](./src/day21.rs) | :thinking:       |
| [22](https://adventofcode.com/2024/day/22) | Monkey Market                   | 2.18ms | 60.8ms | [day22.rs](./src/day22.rs) | :smiley::muscle: |
| [23](https://adventofcode.com/2024/day/23) | LAN Party                       | 589µs  | 10.2ms | [day23.rs](./src/day23.rs) | :exploding_head: |
| [24](https://adventofcode.com/2024/day/24) | Crossed Wires                   | 187µs  | XXXXXX | [day24.rs](./src/day24.rs) | :confounded:     |
| [25](https://adventofcode.com/2024/day/25) | Code Chronicle                  | XXXXXX | ------ | [day25.rs](./src/day25.rs) | :hourglass:      |

## Notes
1. Day 01 part 2 was improved, initially giving a result of 69.69µs
2. Two solutions for day 2 - one arithmetic, and another using a state machine. Both are comparable.
3. Regex was an obvious goto for this, but I really detest using it
4. Some hits for day part 1 were palindromes, and part 2 required only diagonal crosses
5. I had quite a few false positives for day 5 part 2 when trying `easier' solutions
6. Day 6 part 1 was fairly straightforward, but part 2 kept encountering false positives from 8 attempted solutions.
7. Day 7... good old ADS classes. It could be a bit faster though. *Now 10x faster*
8. Solutions work better when there are no typos. Who knew?
9. Day 10 part 2 was inadvertently solved before part 1 :sweat_smile:
10. Day 11 - The order of the stones being preserved is irrelevant. Ignore it.
11. Day 12 - Finally a day I was dreading, when the puzzles get very hard... like me.
12. Day 14 can take a lot longer if you make the wrong assumptions about what the Christmas tree is supposed to look like.
13. Day 15 part 2 was just nasty.
14. Day 17 was really cool to do.
15. Days 21-23 will have to be skipped for now :sob:
16. Day 22 part 2 often has run times of 69 ms.