# Advent Day of Code 2022

https://adventofcode.com/2022

This year I'm working through the problems in Rust. You can run any of the 
problems by passing in the day and the part (1 or 2) as params.

Here are a few things that I've learned about Rust in this process:

1. In Day1, I used a `BinaryHeap` from std::collections to track the calories
   each elf was carrying so that I could pull out the top three easily. The API
   for this wasn't as ergonomic as one would hope, but it worked.
2. In Day2, Rust's matching syntax really shined. In other languages I would've just
   created a static Map, but with `match` I was able to easily find the points associated
   with each of the inputs.
2. [itertools](https://github.com/rust-itertools/itertools) provides
   very helpful extensions to Rust iterators. For example, in Day 3
   we needed to group every three lines together to be processed in a 
   group. The `batching` method was very useful for this, although after
   the fact I found `chunks` may have been better since that takes a static
   number whereas `batching` could be more dynamic.
3. Turn on Clippy in Visual Study Code by setting `rust-analyzer.checkOnSave.command` to 
   `clippy`. This provides way better suggestions than the default.
4. Day 13 seemed like a parsing nightmare at first glance, but instead of trying
   to parse this by hand I turned to [nom](https://github.com/Geal/nom). I was very 
   surprised at how easy the parsing turned out to be. It also helped that I was reviewing
   [Eric Burden's AOC Solution](https://github.com/ericwburden/advent_of_code_2022) to the
   Day 11 problem, which used Nom to parse the input. I also took a page from him with 
   creating a module for the day instead of craming everything into a single file.