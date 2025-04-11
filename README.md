# Quaking Quack
This is a **simple Rust stemmer** implemented as a DuckDB extension. It provides a scalar function that operates on `VARCHAR` columns, allowing users to perform stemming on text data directly within DuckDB.

This project is based on the Rust DuckDB extension template, which provides a foundation for building pure-Rust DuckDB extensions.

Features:
- No DuckDB build required
- No C++ or C code required
- CI/CD chain preconfigured
- (Coming soon) Works with community extensions

## Cloning

Clone the repo with submodules:

```shell
git clone --recurse-submodules git@github.com:martin-conur/quacking-quack.git
```

## Dependencies
In principle, this extension can be compiled with the Rust toolchain alone. However, this project relies on some additional tooling to make life easier and to share CI/CD infrastructure with other DuckDB extensions:

- Python3
- Python3-venv
- [Make](https://www.gnu.org/software/make)
- Git

Installing these dependencies will vary per platform:
- For Linux, these come generally pre-installed or are available through the distro-specific package manager.
- For MacOS, [homebrew](https://formulae.brew.sh/).
- For Windows, [chocolatey](https://community.chocolatey.org/).

## Building
After installing the dependencies, building is a two-step process. Firstly, run:
```shell
make configure
```
This will ensure a Python venv is set up with DuckDB and DuckDB's test runner installed. Additionally, depending on configuration, DuckDB will be used to determine the correct platform for which you are compiling.

Then, to build the extension, run:
```shell
make debug
```
This delegates the build process to cargo, which will produce a shared library in `target/debug/<shared_lib_name>`. After this step, a script is run to transform the shared library into a loadable extension by appending a binary footer. The resulting extension is written to the `build/debug` directory.

To create optimized release binaries, simply run `make release` instead.

## Testing
This extension uses the DuckDB Python client for testing. This should be automatically installed in the `make configure` step.
The tests themselves are written in the SQLLogicTest format, just like most of DuckDB's tests. A sample test can be found in
`test/sql/<extension_name>.test`. To run the tests using the *debug* build:

```shell
make test_debug
```

or for the *release* build:
```shell
make test_release
```

### Version Switching 
Testing with different DuckDB versions is really simple:

First, run:
```shell
make clean_all
```
to ensure the previous `make configure` step is deleted.

Then, run:
```shell
DUCKDB_TEST_VERSION=v1.1.2 make configure
```
to select a different DuckDB version to test with.

Finally, build and test with:
```shell
make debug
make test_debug
```

### Known Issues
This is a bit of a footgun, but the extensions produced by this project may (or may not) be broken on Windows on Python 3.11 with the following error on extension load:
```shell
IO Error: Extension '<name>.duckdb_extension' could not be loaded: The specified module could not be found
```
This was resolved by using Python 3.12.
