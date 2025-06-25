# n = 1;
# d = 1;
# loop {
#     if (n % d == 0) {
#         if (d <= 1) {
#             out(n);
#         }
#         n++;
#         d = n;
#         d--;
#     } else {
#         d--;
#     }
# }

set x0 1
set x1 1
set x6 5
set x7 31
sft x7 x7 << x6
addi x7 31
sft x7 x7 << x6
addi x7 31

:loop
set x2 modulus
jal x2
set x3 main_mod_branch
set x4 0
j x3 x2 = x4
set x4 1
sub x1 x1 x4
set x3 loop
j x3
:main_mod_branch
set x2 1
set x3 main_next_num
j x3 x1 > x2
out x0 0
:main_next_num
addi x0 1
set x1 0
add x1 x1 x0
addi x1 -1
set x2 loop
j x2 x0 < x7
halt

:modulus
set x2 0
add x2 x2 x0
set x3 modulus_end
set x4 modulus_loop
:modulus_loop
j x3 x2 < x1
sub x2 x2 x1
j x4
:modulus_end
ret