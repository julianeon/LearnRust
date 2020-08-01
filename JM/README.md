# How This Is Organized

I tried just uploading the Rust repo... didn't work, caused GitHub issues.

Because running 'cargo build' creates an executable and frankly a lot of junk, it's not easy to upload the whole directory.

What I'm doing now, is pretty simple.

I'm a beginner; everything I wrote in Rust now is just one script.

So basically every Rust directory is one script, plus one .toml file, describing its dependencies.

In this directory, you'll find files of this format:

what-the-script-does.rs
what-the-script-does.toml

The name tells you what it does.

If it ends in .rs, that's what you run 'cargo build' on.

If it ends in .toml, that's the dependency file.

To get it to run, create a new project, cp what-the-script-does.toml to Cargo.toml and what-the-script-does.rs to src/main.rs, run cargo build, and check target/debug for the file name, based on whatever's in Cargo.toml (which you can change).
