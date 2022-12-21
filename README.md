# Directory Finder

## Usage

For a directory structure such as:
- testdir
    - Foo
    - Bar
    - Baz

All usages are of the form `funcname <input>` where `funcname` is the name of your defined shell function (see **Example Setup**).

### Single Match
```shell
funcname f
```
will drop you in the *Foo* directory.

### Multiple Matches
```shell
funcname b
```
will prompt you to select 
```shell
1: Bar
2: Baz
Select a directory:
```
Entering **2** will drop you in the *Baz* directory.

### No Matches
If no matches are found for your given input, you will be dropped in the base directory that was defined (`$DIRECTORY` in the example setup).

## Example Setup
### Compile **directory_finder**

``` shell
rustc src/main.rs
```

### Add Function to .bashrc or .zshrc

``` shell
funcname () {
    DIRECTORY="/home/your_name/path/to/dir"
    SUBDIRECTORY_MATCH=$("/path/to/directory_finder/main" "$DIRECTORY" "$@")

    cd "$DIRECTORY$SUBDIRECTORY_MATCH" || return 
}
```
