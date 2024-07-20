# load and store
set x0 10
set x1 1
set x2 0
set x3 2
store x1 x0
store x3 x2
load x2 x0
out x2 0

# add
set x1 2
set x2 1
add x2 x2 x1
set x1 7
add x2 x2 x1
out x2 1
set x1 -9
add x2 x2 x1
out x2 1

# jump and return
set x0 32
ssp x0
set x0 jump_test
set x1 -1
set x2 0
set x3 1
set x4 6
jal x0 x1 < x2
out x4 2
jal x0 x2 < x1
out x4 2
jal x0 x2 < x3
out x4 2
jal x0 x1 = x1
out x4 2
jal x0 x1 = x2
out x4 2
jal x0 x1 = x3
out x4 2
jal x0 x3 > x2
out x4 2
jal x0 x2 > x1
out x4 2
jal x0 x1 > x2
out x4 2
set x0 jump_end
j x0
:jump_test
add x4 x4 x1
ret
:jump_end
out x4 2

# add immediate
set x0 5
addi x0 5
out x0 3
addi x0 -9
out x0 3

# subtract
set x0 5
set x1 10
sub x0 x0 x1
out x0 4
set x1 -6
sub x0 x0 x1
out x0 4

# and
set x0 -1
set x1 1
and x0 x0 x1
out x0 5

# xor
set x0 85
xor x0 x0 x0
set x1 1
xor x0 x1 x0
out x0 6

# shift
set x0 8
set x1 3
sft x0 x0 >> x1
out x0 7


halt