# kcli
A command line layout analysis tool made using [Keynergy](https://github.com/keynergy/keynergy). 
It serves as a more efficient and extensible version of [genkey](https://github.com/semilin/genkey). 
## Installation
kcli is in beta right now, so there are no binary releases available yet. 
If you want to try it out anyway, you can build it from source.

First, make sure that [Rust and cargo are installed](https://www.rust-lang.org/tools/install).
Then clone the repo and build.
```sh
git clone https://github.com/keynergy/kcli
cd kcli
cargo install --path .
```
If you haven't set up your cargo environment, your cargo path might not be set up, and it won't be able to install.
## Usage
### Setup
First run `kcli` with no arguments to set up the data directories (this will download the default metrics and layouts from git).

Then you need a corpus to analyze layout stats with. Soon there will be corpora data built in, but for now I recommend using the 
[full list of Typeracer quotes](https://cdn.discordapp.com/attachments/831412851599343636/915066831448965191/tr_quotes.txt).

Download that and run `kcli corpus load ./tr_quotes.txt`. Then run `kcli refresh` to bake the metric stats for the corpus. You're good to go!

### Commands
#### Analyze
```
kcli analyze LAYOUT
```
This prints the layout grid and returns all the statistics you have defined.
#### Most/Least
```
kcli least sfb\ distance
```
```
kcli most alternation
```
Prints the top 15 layouts with the most or least of the given metric. 
The number can optionally be changed by appending the number to the end of the command.
```
kcli least sfs 20
```

## Extending Metrics
Metrics are defined in your Keynergy data folder. They are written in the [Ketos](https://docs.rs/ketos/0.12.0/ketos/) extension lisp language,
and described in a toml file. I recommend looking at existing metric functions to figure out how it works;
at some point I'll write some better documentation here. The same function can be used for bigrams or skipgrams by simply creating a separate entry
that takes skipgrams instead - see sfb and sfs as an example.
