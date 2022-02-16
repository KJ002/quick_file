# quick_file
Quickly read and write to files without needing to worry about file paths and operating systems

## Usage

Usage is pretty easy, as this is what this crate is designed for. Your code should look something like this:

``` rust
let config = ConfigManager::new("foobar");

let lorem_ipsum = "
Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.
";

config.write("lorem.txt", lorem_ipsum);
```

If this is ran on Windows this would create a folder in the location of the users appdata (`%APPDATA%`) named `foobar`. However, on Linux and MacOS a folder named `foobar` would be created at the location of the users home (`~`), this is determined by the `HOME` environment variable and likewise the location of `%APPDATA%` is determined by the `APPDATA` environment variable.
