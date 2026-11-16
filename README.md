# cap-cmd

Simple rust library that allows you to run a command, streaming its stdout and stderr to writers while also capturing them for the result or error.

Does not have any dependencies.

I wrote this just to have simple function that I can use for my CI tools so it outputs in real time, but also includes all the output into the error when it fails.

## Usage

Usage can be found in [examples](examples)-directory.
