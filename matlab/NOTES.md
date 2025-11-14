Key point: matlab is about discrete vectors/matrices, it does not have any
understanding of continuous maths in its core feature set. That's why
when you plot a function, you need to first map it onto a discrete linear
space, then plot that.

Use semicolon at the end of a command to suppress output.
Shift+Enter for multi-line statements (or use ellipsis `...`)
Comma to put multiple statements on a single line.
Can evaluate any statement in the command window by selecting, right clicking, `Evaluate Selection`

Function calls support named arguments e.g. `plot(x, y, LineWidth=2)`
Alternative syntax: `plot(x, y, "LineWidth", 2)`
Return values support destructuring e.g. `[foo, bar] = func();` (use `~` to ignore a value).

There is command and function syntax e.g. `open foo` vs `open("foo")`.
Think it's best just to use function syntax everywhere?

## Persistence

```
save foo.mat
load foo.mat
```

Change working directory with `cd`.

##

Output modes

```
format long
format short
format loose
format compact
```

No idea what the differences are, lookup `format` documentation when necessary.


Define matrices

```
a = [1 2 3]
a = [1; 2; 3]
a = [1 2 3; 4 5 6; 7 8 9]
a = [1 2 3
     4 5 6
     7 8 9];
```

Concatenation

```
A = [a, a] % horizontal concat (must have same number of rows)
A = [a; a] % vertical concat (must have same no of cols)
```

## Element-wise operations

```
a.*a
a.^2 % raise each element to 2nd power
```

These operators will do implicit expansion e.g.:

```
x = [1 2 3 4 5]
y = x'
z = x - y

x .* z % will repeat with x for each row
y .* z % will repeat with y for each column
```

Automatic expansion ony happens for dimensions of size 1.

## Subscripting

Indexes seem to start from 1

```
% row, col
A(5, 3)

% traverse down each col in order until the specified index is found
A(10)

% assignment is allowed
A(2,2) = 100

% if the assignment is out of bounds, the matrix is expanded (filled with zeros)
A(100,10) = 1

% extract rows 1-3 (inclusive), col 2
A(1:3, 2)

A(2:4, 2:3)

% ranges are assignable
% here we're taking part of a row vector, transposing, and assigning to a
% vertical slice/range of A
A(1:3, 2) = a(1, 3:5)'
```

## Functions

If a function returns multiple values you can destructure e.g.:

```
[minA, maxA] = bounds(A)
```

If a function has no inputs, it can be called without parens e.g. `clc`

Here's a list of the builtin functions I've come across:

```
inv
zeros
sin, cos, tan
sqrt
size

max

ans % previous answer
diary % log command window to text file


% meta stuff
whos % view contents of workspace
clear % clear all variables from workspace
clc % clears the command window
```

Complex numbers:

```
3+4j
2-4i
i
-i
10j
```

Transpose a matrix with e.g. `a'`

Last evaluation is stored in `ans`.

## Strings

```
str = "foo"
x = 100
str2 = "x is " + x % conversion to string is implicit

% strings can be put into arrays
Names = ["Jason" "Bob" "Alice"; "Sue" "Betty" "Ralph"]
```

Quote using double-double quotes eg. `"i said ""hello"" to him"`

## Character Arrays

```
% this creates a 1-row character array:
seq = 'ACGTGTGAC'

seq(1,3) => 'G'
seq(2) => 'C'

% can be concatenated as normal:
seq2 = [seq, 'ACTGGGG']
seq3 = [seq; seq; seq]
```

## Plotting

### 2D

```
% generate 1000 points from 0..2*pi
% the start/end is inclusive
% if the 1000 is omitted it defaults too 100
x = linspace(0, 2 * pi, 1000)

y = sin(x)

% this plots the graph in another window
plot(x, y)
plot(x, y, "r--") % "r--" => red dashed line
plot(x, y, "g:*") % "g:*" => green with * markers at data points

% functions for modifying the plot (call after plot())
xlabel("x")
ylabel("f(x)")
title("a plot of the f function")

% by default, the "figure window" is cleared each time you plot
hold on % add plots to existing figure window
% ... do some plotting ...
hold off % revert to normal behaviour
```

### 3D

```
x = linspace(-2, 2, 20)
y = x'

% a lot is happening here
% the expression inside the parens involves a row vector and a column vector
% so the result is expanded to a 2D matrix, one value for each combination of x and y
% (this is called "implicit expansion")
z = x .* exp(-x.^2 - y.^2)

surf(x, y, z)
```

### Multiple Plots

```
x = linspace(0, 30)

t = tiledlayout(2, 2)
title(t, "Trig functions")

nexttile
plot(x, sin(x))
title("Sine")

nexttile
plot(x, cos(x))
title("Cosine")

% and so on...
```

## Scripts

To edit a script: `edit foo`.

To edit a _live script_: `edit foo.mlx`.

Live scripts are some sort of literate/Jupyter hybrid? I dunno.

# Tutorial

## Matrices & Arrays

Functions for creating and combining arrays:

```
zeros
ones
rand

% these create something called "logical arrays"
true
false

eye % identity matrix

diag % e.g. A = [1 2 3 4], diag(A)

blkdiag % combine matrices by combining along the diagonal

cat % concatenate on arbitrary dimension (dimension no. is 1st arg)
horzcat % horizontal concat
vertcat % vertical concat
repelem % repeat copies of array elements
repmat % repeat copies of matrices
combinations
```

Create grids:

```
linspace
logspace
freqspace
meshgrid
ndgrid
```

Determine size, shape, order:

```
length % length of largest dimension
size % array size
ndims % number of array dimensions
numel % number of array elements
isscalar
isvector
ismatrix
isrow % is row vector?
iscolumn % is column vector?
isempty
issorted
issortedrows
isuniform
```

Resize:

```
head
tail
resize
paddata
trimdata
```

Reshape:

```
permute
ipermute
shiftdim
reshape
squeeze
```

Rearrange:

```
sort
sortrows
flip
fliplr
flipud
rot90
transpose
ctranspose
circshift
```

Indexing:

```
colon
end
ind2sub
sub2ind
```

Basic creation etc.:

```
[1 2 3]
[1 2 3; 4 5 6]
[A B]     % horizontal concat
[A; B]    % vertical concat
1:10      % 1 2 3 4 5 6 7 8 9 10
0:2:10    % 0 2 4 6 8 10

```
