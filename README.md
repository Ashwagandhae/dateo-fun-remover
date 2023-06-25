# dateo-fun-remover

Date-o fun remover: a solver for the hit online game [date-o](https://dateo-math-game.com/). Made in collaboration with Finn McKibbin. Thank you to [this blog post](https://blog.logrocket.com/integrating-svelte-app-rust-webassembly/) for the Svelte and Wasm setup.

## Usage

Go to [the website](https://ashwagandhae.github.io/dateo-fun-remover/)... or run it locally in your terminal:

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

## Usage in terminal

```bash
git clone https://github.com/Ashwagandhae/dateo-fun-remover.git
cd dateo-fun-remover
# we cd into solver to ignore the web app
cd solver
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
