# Prerequisites:
- **Rust compiler** ([installation instructions](https://www.rust-lang.org/tools/install), it is important to allow installation of Visual Studio C++ Build tools on Windows - this is the default installation procedure),
- **Python** version at least 3.7

The same tools are required for each operating system. "Visual Studio C++ Build tools" in the case of Windows are required due to the use of the linker (it should be possible to bypass it with the appropriate flags). Check that all required tools are in the path (they are available from the terminal level without having to go to the directories where they were installed).

# Installation:
- Create a python virtual environment and activate it,
- In the activated environment, install:
    - **Maturin** - the build manager for native python extensions (**pip install maturin**),
    - **Pdoc** - the Python documentation generator (**pip install pdoc**),
- Go to the project directory (where the **Cargo.toml** file is located),
- Use the command **maturin develop --release** (the library will be compiled with optimizations enabled and installed in the activated environment)

Now the library is ready to use (make sure that the environment in which the library was installed is active). The library can also be compiled into a **.whl** installation package using the **maturin build** command ([instructions](https://github.com/PyO3/maturin)).

# Documentation:
The HTML documentation for the nanoPDB library is available in the **docs** directory.

# Useful VSCode extensions:
- rust-analyzer (The Rust Programming Language)
- Even Better TOML (tamasfe)
- Python (as extension pack)
