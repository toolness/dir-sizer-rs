This is a simple Rust program that generates information about
the directories taking up lots of space on your system.

It generates two CSV files as output:

* `dirs.csv` is both a form of output, and a cache. It lists *all*
  directories and their sizes. You can interrupt the data collection
  process and the program will pick up where it left off by using this
  CSV file as a cache.

* `big_dirs.csv` only lists directories whose contents are at least
  a certain size.

## Motivation

I'm sure there are tools that do the same thing faster and/or in a
more user-friendly manner, but I needed an excuse to brush up on my
Rust skills.

## Documentation

```
dir-sizer 0.1.0
Atul Varma <varmaa@gmail.com>
Generates information about the directories taking up lots of space on your system.

USAGE:
    dir-sizer.exe [OPTIONS] [PATH]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --big-csv-file <FILE>    Name of CSV file in which to list big directories [default: big_dirs.csv]
    -s, --big-size <SIZE>        A dir must be this big (in bytes) to be considered 'big' [default: 100,000,000]
    -c, --csv-file <FILE>        Name of CSV file in which to store/cache all directory info [default: dirs.csv]

ARGS:
    <PATH>    The directory to profile (defaults to current working dir)

```

## License

CC0 1.0 (universal public domain).
