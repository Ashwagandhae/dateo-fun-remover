# dateo-fun-remover

Date-o fun remover: a solver for the hit online game [date-o](https://dateo-math-game.com/).

## Prerequisites

### Windows

1. Download [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/).
2. Install [Rust](https://www.rust-lang.org/tools/install).

### MacOS

1. Install CLang and macOS Development Dependencies.

```bash
xcode-select --install
```

2. Install [Rust](https://www.rust-lang.org/tools/install).

### Linux

1. Install a C compiler, depending on the distro.
2. Install [Rust](https://www.rust-lang.org/tools/install).

## Usage

```bash
git clone https://github.com/Ashwagandhae/dateo-fun-remover.git
cd dateo-fun-remover
```

### Running the solver

Will calculate goal & numbers from the date, based the [source code](https://dateo-math-game.com/setNumbers.js).

```bash
cargo run --release
```

### Running with custom numbers

Usage:

```bash
cargo run --release -- [OPTIONS]
```

Options:

```
-n, --nums <NUMS>            Given numbers (prioritized over date generated numbers)
-g, --goal <GOAL>            Goal number (prioritized over date generated numbers)
-f, --full-date <FULL_DATE>  Full date to use for generating numbers
-d, --day <DAY>              Day of month to use for generating numbers
-m, --month <MONTH>          Month of year to use for generating numbers
-y, --year <YEAR>            Year to use for generating numbers
-h, --help                   Print help
-V, --version                Print version
```

Examples:

You can input the goal number and the numbers to use...

```bash
cargo run --release -- --goal 1 --numbers "1 2 3 4 5"
cargo run --release -- -g 1 -n "1 2 3 4 5"
```

...or you can input the date in the format `YYYY-MM-DD` to calculate them.

```bash
cargo run --release -- --date 2021-10-01
cargo run --release -- -d 2021-10-01
```

...or you can specify month, day, and year, and let today's date fill in the rest.

```bash
# specify month, day, and year
cargo run --release -- --month 10 --day 1 --year 2021
cargo run --release -- -m 10 -d 1 -y 2021
# specify only day
cargo run --release -- --day 1
cargo run --release -- -d 1
```

## Optimizations

At its core, the solver uses a brute force approach to solve the game, looping through all possible equations. It first generates all possible equation structures with no functions, from `(1 + (2 + (3 + (4 + 5))))`, to `(3 ^ 4)`. Then, the solver distributes functions amongst the nodes based on the current target score and checks these new equations. However, the solver uses a few optimizations to speed up the process, which I have listed below so I remember how this project works in 2 weeks, and so that Github Copilot can give me suggestions on more.

### Equal equations removal

The solver removes all identical equations, allowing for speedups when the input numbers generate as the same number multiple times.

### Equivalent equations removal

The solver removes all equivalent equations, for example by only including the equation `1 + 2` and not `2 + 1`, leading to a small decrease in the number of base equations.

### Impossible equations removal

The solver removes all equations impossible to solve equations, removing equations like `0 ^ -1`. This removal only happens when the input numbers reveal themselves as "immune" to all functions (square root, factoral, summation), because numbers without immunity may become possible when combined with those functions. In practice, all negative numbers and equations with only negative numbers have immunity, leading to a significant speedup with many negative numbers.

### Immune node functions removal

The solver removes all functions from nodes that are immune to all functions, leading to a significant speedup with any number of negative numbers.

### Conversion to instructions list

The solver converts the equations from a tree structure to a list of instructions, allowing for a slighly faster equation eval, while also
making it easier to implement other optimizations.

### Instruction reordering

The solver tries to put failure-prone instructions, like `^` and `???`, at the start of the list, allowing for a faster failure and reducing the number of unnecessarily evaluated instructions.

### Useless score checking prevention

The solver only checks base equations with scores above the current best score, preventing useless score checking.

### Goal paths generation

The solver generates possible paths to a goal number using only functions, creating a larger net of numbers for the solver to compare to, and allowing the solver to "jump" score points by using functions to get to the goal number (credit to Finn for telling me to use dict).

### Parallelization

The solver uses rayon to parallelize the equation generation, allowing for a significant speedup on multi-core machines.
