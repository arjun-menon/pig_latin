## Pig Latin tool
This program takes a text file as input, transforms it to [pig latin](https://en.wikipedia.org/wiki/Pig_Latin), and writes that to a text file as output.

### Source Code

See all of the code at: https://github.com/arjun-menon/pig_latin/blob/master/src/main.rs

### Compiling

Steps:
1. Install Rust. [See this guideline](https://doc.rust-lang.org/book/ch01-01-installation.html).
2. Clone this repo. Inside the repo, run `cargo build --release`.
3. The compiled exectuable should be at `target/release/pig_latin`.

### Usage

Run the `pig_latin` exectuable (from above, with no arguments) to do a test run against `t8.shakespeare.txt`. Provide a different input file as an argument to `pig_latin` (see CLI section below), for example: `pig_latin alexis_de_tocqueville.txt`.

By default, the output ends up at `output.txt`, but a different output file can be specified using the `--output` flag.

### Low Memory Option

Run `pig_latin` with `--lowmem` to use less memory. When running with this option enabled, `pig_latin` will process the text file line-by-line (while also limiting processing ***to a single thread***), and hence the overall and peak memory usage **will be significantly lower**. 

### Comamnd-Line Interface (CLI)

`pig_latin` comes with a simple CLI. Run `pig_latin --help` to get this overview of the command-line arguments:

    Transform text into pig latin

    USAGE:
        pig_latin [OPTIONS] [FILENAME]

    ARGS:
        <FILENAME>    File name to process

    OPTIONS:
        -h, --help               Print help information
        -l, --lowmem             Use less RAM (random access memory)
        -o, --output <OUTPUT>    Output file name [default: output.txt]

