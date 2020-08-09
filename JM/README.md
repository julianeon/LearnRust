# How This Is Organized

Because running 'cargo build' creates an executable and frankly a lot of junk, it's not easy to upload the whole directory.

What I'm doing now:

Just uploading a file with the Rust script.

The dependencies file rarely changes and likely won't be a blocker for anyone looking to learn to run it on their own anyway, but be aware it may not work if dependencies aren't there (95% of the time it will, however).

# The Format

In this directory, you'll find files of this format:

what-the-script-does.rs

The name tells you what it does and/or where it came from (see: AoC).

If it ends in .rs, that's what you run 'cargo build' on; that's Rust.

Files starting with "AoC" were created to answer the questions in Advent of Code, 2015 version.

https://adventofcode.com/2015

Occasionally, rarely, a filename ending in .rs also has a .toml version.

If it ends in .toml, that's got the dependencies in it; that's the config file for the Rust file.

# How To Run These Scripts

To get a what-the-script-does.rs/toml file to run:

Create a new project.

Copy, using the cp command, what-the-script-does.rs to src/main.rs

If there's a what_the_script_des.toml file, copy that to Cargo.toml. If there isn't, you're on your own to figure that out (usually not hard tho).

Run cargo build from the project base directory (should work fine, for everything I upload).

Check target/debug for the executable file name, or just run it directly using 'cargo build'.
