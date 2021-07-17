# problem-tracker

A tool which I made to track the number of physics problems which I've solved every day.

## Usage

To use the tool, specify the data using commands (see [Commands](#commands)) in a text file, and direct the text file to the standard input of the program.

For example, if the data is in `problems.txt`:

```bash
./problem-tracker < problems.txt
```

## Commands

Commands supported for problem-tracker files:

```
set problem goal <Num>
set penalty <Num>
begin <y:Num> - <m:Num> - <d:Num>
begin chapter <Num>
assigned <Num> ProblemUnit
finished <Num> ProblemUnit
add <ProblemList>
add even <ProblemList>
add odd <ProblemList>
need to fix <ProblemList>
fixed <ProblemList>
penalty
```

`Num` is a non-negative integer

`ProblemList` is a list of problem numbers - supports both ranges and comma separated values (e.g. `3, 5-10, 2, 12`). *Note: the parser doesn't care about spaces.*

`ProblemUnit` is a word: either `problem` or `problems`

### Comments

Problem-tracker also supports coomments. Comments begin with a `#` character and continue to the end of the line. For example:

```
set problem goal 400 # This is a comment
# This is also a comment
```

## Example problems.txt file

```
set problem goal 400 # hello
# This is a comment
set penalty 10

begin 2021-05-22
begin chapter 11
add 42-47
assigned 6 problems
penalty

begin 2021-05-23
assigned 6 problems
finished 6 problems
need to fix 44,46

begin 2021-05-24 # Victoria Day
add 48-57
assigned 8 problems
finished 1 problem
penalty
```
