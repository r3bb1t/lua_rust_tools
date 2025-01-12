-- BYTECODE -- sample1.lua:2-6
0001    GGET     2   0      ; "print"
0002    KSTR     4   1      ; "hello1"
0003    CALL     2   1   2
0004    GGET     2   0      ; "print"
0005    KSTR     4   2      ; "hello"
0006    CALL     2   1   2
0007    ADDVV    2   0   1
0008    RET1     2   2

-- BYTECODE -- sample1.lua:9-11
0001    SUBVV    2   0   1
0002    RET1     2   2

-- BYTECODE -- sample1.lua:14-16
0001    MULVV    2   0   1
0002    RET1     2   2

-- BYTECODE -- sample1.lua:19-25
0001    ISNEN    1   0      ; 0
0002    JMP      2 => 0006
0003    KSTR     2   0      ; "Error: Division by zero!"
0004    RET1     2   2
0005    JMP      2 => 0008
0006 => DIVVV    2   0   1
0007    RET1     2   2
0008 => RET0     0   1

-- BYTECODE -- sample1.lua:28-46
0001    ISNEN    0   0      ; 1
0002    JMP      1 => 0006
0003    KSTR     1   0      ; "Monday: Start of the work week."
0004    RET1     1   2
0005    JMP      1 => 0038
0006 => ISNEN    0   1      ; 2
0007    JMP      1 => 0011
0008    KSTR     1   1      ; "Tuesday: Second day of the work week."
0009    RET1     1   2
0010    JMP      1 => 0038
0011 => ISNEN    0   2      ; 3
0012    JMP      1 => 0016
0013    KSTR     1   2      ; "Wednesday: Midweek day."
0014    RET1     1   2
0015    JMP      1 => 0038
0016 => ISNEN    0   3      ; 4
0017    JMP      1 => 0021
0018    KSTR     1   3      ; "Thursday: Almost the weekend."
0019    RET1     1   2
0020    JMP      1 => 0038
0021 => ISNEN    0   4      ; 5
0022    JMP      1 => 0026
0023    KSTR     1   4      ; "Friday: Last work day of the week!"
0024    RET1     1   2
0025    JMP      1 => 0038
0026 => ISNEN    0   5      ; 6
0027    JMP      1 => 0031
0028    KSTR     1   5      ; "Saturday: Weekend!"
0029    RET1     1   2
0030    JMP      1 => 0038
0031 => ISNEN    0   6      ; 7
0032    JMP      1 => 0036
0033    KSTR     1   6      ; "Sunday: Weekend!"
0034    RET1     1   2
0035    JMP      1 => 0038
0036 => KSTR     1   7      ; "Invalid day! Please enter a number betwe"~
0037    RET1     1   2
0038 => RET0     0   1

-- BYTECODE -- sample1.lua:49-51
0001    MOV      3   0
0002    TGETS    1   0   0  ; "reverse"
0003    CALLT    1   2

-- BYTECODE -- sample1.lua:0-61
0001    FNEW     0   0      ; sample1.lua:2
0002    GSET     0   1      ; "add"
0003    FNEW     0   2      ; sample1.lua:9
0004    GSET     0   3      ; "subtract"
0005    FNEW     0   4      ; sample1.lua:14
0006    GSET     0   5      ; "multiply"
0007    FNEW     0   6      ; sample1.lua:19
0008    GSET     0   7      ; "divide"
0009    FNEW     0   8      ; sample1.lua:28
0010    GSET     0   9      ; "dayType"
0011    FNEW     0  10      ; sample1.lua:49
0012    GSET     0  11      ; "reverseString"
0013    GGET     0  12      ; "print"
0014    KSTR     2  13      ; "Addition: "
0015    GGET     3   1      ; "add"
0016    KSHORT   5   5
0017    KSHORT   6   3
0018    CALL     3   2   3
0019    CAT      2   2   3
0020    CALL     0   1   2
0021    GGET     0  12      ; "print"
0022    KSTR     2  14      ; "Subtraction: "
0023    GGET     3   3      ; "subtract"
0024    KSHORT   5  10
0025    KSHORT   6   4
0026    CALL     3   2   3
0027    CAT      2   2   3
0028    CALL     0   1   2
0029    GGET     0  12      ; "print"
0030    KSTR     2  15      ; "Multiplication: "
0031    GGET     3   5      ; "multiply"
0032    KSHORT   5   3
0033    KSHORT   6   7
0034    CALL     3   2   3
0035    CAT      2   2   3
0036    CALL     0   1   2
0037    GGET     0  12      ; "print"
0038    KSTR     2  16      ; "Division: "
0039    GGET     3   7      ; "divide"
0040    KSHORT   5   8
0041    KSHORT   6   2
0042    CALL     3   2   3
0043    CAT      2   2   3
0044    CALL     0   1   2
0045    GGET     0  12      ; "print"
0046    KSTR     2  17      ; "Division by zero: "
0047    GGET     3   7      ; "divide"
0048    KSHORT   5   8
0049    KSHORT   6   0
0050    CALL     3   2   3
0051    CAT      2   2   3
0052    CALL     0   1   2
0053    GGET     0  12      ; "print"
0054    GGET     2   9      ; "dayType"
0055    KSHORT   4   3
0056    CALL     2   0   2
0057    CALLM    0   1   0
0058    GGET     0  12      ; "print"
0059    GGET     2  11      ; "reverseString"
0060    KSTR     4  18      ; "Hello, World!"
0061    CALL     2   0   2
0062    CALLM    0   1   0
0063    RET0     0   1

