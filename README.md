#shzhrs

[![Travis CI Status](https://travis-ci.org/dten/shzhrs.svg?branch=master)](https://travis-ci.org/dten/shzhrs)

Usage:

`cargo run --release --example solve`

```
Enter board:
;;;;;;;rDg5bDg2b6;r8ffg1b4r6;g3bDg7r7r4;r5b1gDr2b2;bDb8g4gDr9;r3g9r1b7b5;g8gDrDb3rD;b9rDgDg6bD
Solved in Duration { secs: 0, nanos: 1719064 }

Move Red 4 from pile 3 to pile 6
;;;;;;;rDg5bDg2b6;r8ffg1b4r6;g3bDg7r7;r5b1gDr2b2;bDb8g4gDr9;r3g9r1b7b5r4;g8gDrDb3rD;b9rDgDg6bD

Move Black 6 from pile 1 to pile 3
;;;;;;;rDg5bDg2;r8ffg1b4r6;g3bDg7r7b6;r5b1gDr2b2;bDb8g4gDr9;r3g9r1b7b5r4;g8gDrDb3rD;b9rDgDg6bD

Move Black 5 from pile 6 to pile 2
;;;;;;;rDg5bDg2;r8ffg1b4r6b5r4;g3bDg7r7b6;r5b1gDr2b2;bDb8g4gDr9;r3g9r1b7;g8gDrDb3rD;b9rDgDg6bD

Move Red 6 from pile 2 to pile 6
;;;;;;;rDg5bDg2;r8ffg1b4;g3bDg7r7b6;r5b1gDr2b2;bDb8g4gDr9;r3g9r1b7r6b5r4;g8gDrDb3rD;b9rDgDg6bD

Move Green 2 from pile 1 to spares
g2;;;;;;;rDg5bD;r8ffg1b4;g3bDg7r7b6;r5b1gDr2b2;bDb8g4gDr9;r3g9r1b7r6b5r4;g8gDrDb3rD;b9rDgDg6bD

Move Black Dragon from pile 1 to spares
g2;bD;;;;;;rDg5;r8ffg1b4;g3bDg7r7b6;r5b1gDr2b2;bDb8g4gDr9;r3g9r1b7r6b5r4;g8gDrDb3rD;b9rDgDg6bD

Move Green 5 from pile 1 to pile 3
g2;bD;;;;;;rD;r8ffg1b4;g3bDg7r7b6g5;r5b1gDr2b2;bDb8g4gDr9;r3g9r1b7r6b5r4;g8gDrDb3rD;b9rDgDg6bD

Move Black 4 from pile 2 to pile 3
g2;bD;;;;;;rD;r8ffg1;g3bDg7r7b6g5b4;r5b1gDr2b2;bDb8g4gDr9;r3g9r1b7r6b5r4;g8gDrDb3rD;b9rDgDg6bD

Place Green 1
g2;bD;;;;g1;;rD;r8ff;g3bDg7r7b6g5b4;r5b1gDr2b2;bDb8g4gDr9;r3g9r1b7r6b5r4;g8gDrDb3rD;b9rDgDg6bD

Flower power!
g2;bD;;ff;;g1;;rD;r8;g3bDg7r7b6g5b4;r5b1gDr2b2;bDb8g4gDr9;r3g9r1b7r6b5r4;g8gDrDb3rD;b9rDgDg6bD

Place Green 2
;bD;;ff;;g2;;rD;r8;g3bDg7r7b6g5b4;r5b1gDr2b2;bDb8g4gDr9;r3g9r1b7r6b5r4;g8gDrDb3rD;b9rDgDg6bD

Move Black 7 from pile 6 to pile 2
;bD;;ff;;g2;;rD;r8b7r6b5r4;g3bDg7r7b6g5b4;r5b1gDr2b2;bDb8g4gDr9;r3g9r1;g8gDrDb3rD;b9rDgDg6bD

Place Red 1
;bD;;ff;;g2;r1;rD;r8b7r6b5r4;g3bDg7r7b6g5b4;r5b1gDr2b2;bDb8g4gDr9;r3g9;g8gDrDb3rD;b9rDgDg6bD

Move Black 2 from pile 4 to spares
b2;bD;;ff;;g2;r1;rD;r8b7r6b5r4;g3bDg7r7b6g5b4;r5b1gDr2;bDb8g4gDr9;r3g9;g8gDrDb3rD;b9rDgDg6bD

Place Red 2
b2;bD;;ff;;g2;r2;rD;r8b7r6b5r4;g3bDg7r7b6g5b4;r5b1gD;bDb8g4gDr9;r3g9;g8gDrDb3rD;b9rDgDg6bD

Move Green Dragon from pile 4 to spares
b2;bD;gD;ff;;g2;r2;rD;r8b7r6b5r4;g3bDg7r7b6g5b4;r5b1;bDb8g4gDr9;r3g9;g8gDrDb3rD;b9rDgDg6bD

Place Black 1
b2;bD;gD;ff;b1;g2;r2;rD;r8b7r6b5r4;g3bDg7r7b6g5b4;r5;bDb8g4gDr9;r3g9;g8gDrDb3rD;b9rDgDg6bD

Place Black 2
;bD;gD;ff;b2;g2;r2;rD;r8b7r6b5r4;g3bDg7r7b6g5b4;r5;bDb8g4gDr9;r3g9;g8gDrDb3rD;b9rDgDg6bD

Move Red Dragon from pile 7 to spares
rD;bD;gD;ff;b2;g2;r2;rD;r8b7r6b5r4;g3bDg7r7b6g5b4;r5;bDb8g4gDr9;r3g9;g8gDrDb3;b9rDgDg6bD

Place Black 3
rD;bD;gD;ff;b3;g2;r2;rD;r8b7r6b5r4;g3bDg7r7b6g5b4;r5;bDb8g4gDr9;r3g9;g8gDrD;b9rDgDg6bD

Place Black 4
rD;bD;gD;ff;b4;g2;r2;rD;r8b7r6b5r4;g3bDg7r7b6g5;r5;bDb8g4gDr9;r3g9;g8gDrD;b9rDgDg6bD

Move Red 8 from pile 2 to pile 6
rD;bD;gD;ff;b4;g2;r2;rD;;g3bDg7r7b6g5;r5;bDb8g4gDr9;r3g9r8b7r6b5r4;g8gDrD;b9rDgDg6bD

Move Green 9 from pile 6 to pile 2
rD;bD;gD;ff;b4;g2;r2;rD;g9r8b7r6b5r4;g3bDg7r7b6g5;r5;bDb8g4gDr9;r3;g8gDrD;b9rDgDg6bD

Place Red 3
rD;bD;gD;ff;b4;g2;r3;rD;g9r8b7r6b5r4;g3bDg7r7b6g5;r5;bDb8g4gDr9;;g8gDrD;b9rDgDg6bD

Place Red 4
rD;bD;gD;ff;b4;g2;r4;rD;g9r8b7r6b5;g3bDg7r7b6g5;r5;bDb8g4gDr9;;g8gDrD;b9rDgDg6bD

Place Black 5
rD;bD;gD;ff;b5;g2;r4;rD;g9r8b7r6;g3bDg7r7b6g5;r5;bDb8g4gDr9;;g8gDrD;b9rDgDg6bD

Place Red 5
rD;bD;gD;ff;b5;g2;r5;rD;g9r8b7r6;g3bDg7r7b6g5;;bDb8g4gDr9;;g8gDrD;b9rDgDg6bD

Move Black Dragon from pile 8 to pile 4
rD;bD;gD;ff;b5;g2;r5;rD;g9r8b7r6;g3bDg7r7b6g5;bD;bDb8g4gDr9;;g8gDrD;b9rDgDg6

Move Green 6 from pile 8 to pile 6
rD;bD;gD;ff;b5;g2;r5;rD;g9r8b7r6;g3bDg7r7b6g5;bD;bDb8g4gDr9;g6;g8gDrD;b9rDgD

Place Red 6
rD;bD;gD;ff;b5;g2;r6;rD;g9r8b7;g3bDg7r7b6g5;bD;bDb8g4gDr9;g6;g8gDrD;b9rDgD

Move Green 6 from pile 6 to pile 2
rD;bD;gD;ff;b5;g2;r6;rD;g9r8b7g6;g3bDg7r7b6g5;bD;bDb8g4gDr9;;g8gDrD;b9rDgD

Move Green Dragon from pile 8 to pile 6
rD;bD;gD;ff;b5;g2;r6;rD;g9r8b7g6;g3bDg7r7b6g5;bD;bDb8g4gDr9;gD;g8gDrD;b9rD

Stack Red Dragons
rDrDrDrD;bD;gD;ff;b5;g2;r6;;g9r8b7g6;g3bDg7r7b6g5;bD;bDb8g4gDr9;gD;g8gD;b9

Move Red 9 from pile 5 to pile 1
rDrDrDrD;bD;gD;ff;b5;g2;r6;r9;g9r8b7g6;g3bDg7r7b6g5;bD;bDb8g4gD;gD;g8gD;b9

Stack Green Dragons
rDrDrDrD;bD;gDgDgDgD;ff;b5;g2;r6;r9;g9r8b7g6;g3bDg7r7b6g5;bD;bDb8g4;;g8;b9

Move Red 7 from pile 3 to pile 7
rDrDrDrD;bD;gDgDgDgD;ff;b5;g2;r6;r9;g9r8b7g6;g3bDg7;bD;bDb8g4;;g8r7b6g5;b9

Move Green 4 from pile 5 to pile 6
rDrDrDrD;bD;gDgDgDgD;ff;b5;g2;r6;r9;g9r8b7g6;g3bDg7;bD;bDb8;g4;g8r7b6g5;b9

Move Green 7 from pile 3 to pile 5
rDrDrDrD;bD;gDgDgDgD;ff;b5;g2;r6;r9;g9r8b7g6;g3bD;bD;bDb8g7;g4;g8r7b6g5;b9

Move Black 8 from pile 5 to pile 1
rDrDrDrD;bD;gDgDgDgD;ff;b5;g2;r6;r9b8g7;g9r8b7g6;g3bD;bD;bD;g4;g8r7b6g5;b9

Stack Black Dragons
rDrDrDrD;bDbDbDbD;gDgDgDgD;ff;b5;g2;r6;r9b8g7;g9r8b7g6;g3;;;g4;g8r7b6g5;b9

Place Green 3
rDrDrDrD;bDbDbDbD;gDgDgDgD;ff;b5;g3;r6;r9b8g7;g9r8b7g6;;;;g4;g8r7b6g5;b9

Place Green 4
rDrDrDrD;bDbDbDbD;gDgDgDgD;ff;b5;g4;r6;r9b8g7;g9r8b7g6;;;;;g8r7b6g5;b9

Place Green 5
rDrDrDrD;bDbDbDbD;gDgDgDgD;ff;b5;g5;r6;r9b8g7;g9r8b7g6;;;;;g8r7b6;b9

Place Black 6
rDrDrDrD;bDbDbDbD;gDgDgDgD;ff;b6;g5;r6;r9b8g7;g9r8b7g6;;;;;g8r7;b9

Place Green 6
rDrDrDrD;bDbDbDbD;gDgDgDgD;ff;b6;g6;r6;r9b8g7;g9r8b7;;;;;g8r7;b9

Place Black 7
rDrDrDrD;bDbDbDbD;gDgDgDgD;ff;b7;g6;r6;r9b8g7;g9r8;;;;;g8r7;b9

Place Green 7
rDrDrDrD;bDbDbDbD;gDgDgDgD;ff;b7;g7;r6;r9b8;g9r8;;;;;g8r7;b9

Place Black 8
rDrDrDrD;bDbDbDbD;gDgDgDgD;ff;b8;g7;r6;r9;g9r8;;;;;g8r7;b9

Place Black 9
rDrDrDrD;bDbDbDbD;gDgDgDgD;ff;b9;g7;r6;r9;g9r8;;;;;g8r7;

Place Red 7
rDrDrDrD;bDbDbDbD;gDgDgDgD;ff;b9;g7;r7;r9;g9r8;;;;;g8;

Place Green 8
rDrDrDrD;bDbDbDbD;gDgDgDgD;ff;b9;g8;r7;r9;g9r8;;;;;;

Place Red 8
rDrDrDrD;bDbDbDbD;gDgDgDgD;ff;b9;g8;r8;r9;g9;;;;;;

Place Green 9
rDrDrDrD;bDbDbDbD;gDgDgDgD;ff;b9;g9;r8;r9;;;;;;;

Place Red 9
rDrDrDrD;bDbDbDbD;gDgDgDgD;ff;b9;g9;r9;;;;;;;;
```