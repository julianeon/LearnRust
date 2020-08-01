# How This Is Organized

I tried just uploading the Rust repo... didn't work, caused GitHub issues.

Because running 'cargo build' creates an executable and frankly a lot of junk, it's not easy to upload the whole directory.

What I'm doing now, is pretty simple.

I'm a beginner; everything I wrote in Rust now is just one script.

So basically every Rust directory is one script, plus one .toml file, describing its dependencies.

# The Format

In this directory, you'll find files of this format:

what-the-script-does.rs
what-the-script-does.toml

For every what-the-script-does.* there will be an .rs file and a .toml file.

The name tells you what it does.

If it ends in .rs, that's what you run 'cargo build' on; that's Rust.

If it ends in .toml, that's got the dependencies in it; that's the config file for the Rust file.

# How To Run These Scripts

To get a what-the-script-does.rs/toml file pair to run:

Create a new project.

Copy, using the cp command, what-the-script-does.toml to Cargo.toml and what-the-script-does.rs to src/main.rs

Run cargo build from the project base directory (should work fine, for everything I upload).

Check target/debug for the executable file name, based on whatever's in Cargo.toml (which you can change).
