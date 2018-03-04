#shzhrs

[![Travis CI Status](https://travis-ci.org/dten/shzhrs.svg?branch=master)](https://travis-ci.org/dten/shzhrs)

Usage:

`cargo run --release --example solve`

```
Enter board:
;;;;;;;rDg5bDg2b6;r8ffg1b4r6;g3bDg7r7r4;r5b1gDr2b2;bDb8g4gDr9;r3g9r1b7b5;g8gDrDb3rD;b9rDgDg6bD
Solved in Duration { secs: 0, nanos: 5964942 }

Move Black 5 from pile 6 to pile 2
;;;;;;;rDg5bDg2b6;r8ffg1b4r6b5;g3bDg7r7r4;r5b1gDr2b2;bDb8g4gDr9;r3g9r1b7;g8gDrDb3rD;b9rDgDg6bD

Move Black 7 from pile 6 to spares
b7;;;;;;;rDg5bDg2b6;r8ffg1b4r6b5;g3bDg7r7r4;r5b1gDr2b2;bDb8g4gDr9;r3g9r1;g8gDrDb3rD;b9rDgDg6bD

Place Red 1
b7;;;;;;r1;rDg5bDg2b6;r8ffg1b4r6b5;g3bDg7r7r4;r5b1gDr2b2;bDb8g4gDr9;r3g9;g8gDrDb3rD;b9rDgDg6bD

Move Black 2 from pile 4 to spares
b7;b2;;;;;r1;rDg5bDg2b6;r8ffg1b4r6b5;g3bDg7r7r4;r5b1gDr2;bDb8g4gDr9;r3g9;g8gDrDb3rD;b9rDgDg6bD

Place Red 2
b7;b2;;;;;r2;rDg5bDg2b6;r8ffg1b4r6b5;g3bDg7r7r4;r5b1gD;bDb8g4gDr9;r3g9;g8gDrDb3rD;b9rDgDg6bD

Move Green Dragon from pile 4 to spares
b7;b2;gD;;;;r2;rDg5bDg2b6;r8ffg1b4r6b5;g3bDg7r7r4;r5b1;bDb8g4gDr9;r3g9;g8gDrDb3rD;b9rDgDg6bD

Place Black 1
b7;b2;gD;;b1;;r2;rDg5bDg2b6;r8ffg1b4r6b5;g3bDg7r7r4;r5;bDb8g4gDr9;r3g9;g8gDrDb3rD;b9rDgDg6bD

Place Black 2
b7;;gD;;b2;;r2;rDg5bDg2b6;r8ffg1b4r6b5;g3bDg7r7r4;r5;bDb8g4gDr9;r3g9;g8gDrDb3rD;b9rDgDg6bD

Move Green 9 from pile 6 to spares
b7;g9;gD;;b2;;r2;rDg5bDg2b6;r8ffg1b4r6b5;g3bDg7r7r4;r5;bDb8g4gDr9;r3;g8gDrDb3rD;b9rDgDg6bD

Place Red 3
b7;g9;gD;;b2;;r3;rDg5bDg2b6;r8ffg1b4r6b5;g3bDg7r7r4;r5;bDb8g4gDr9;;g8gDrDb3rD;b9rDgDg6bD

Place Red 4
b7;g9;gD;;b2;;r4;rDg5bDg2b6;r8ffg1b4r6b5;g3bDg7r7;r5;bDb8g4gDr9;;g8gDrDb3rD;b9rDgDg6bD

Place Red 5
b7;g9;gD;;b2;;r5;rDg5bDg2b6;r8ffg1b4r6b5;g3bDg7r7;;bDb8g4gDr9;;g8gDrDb3rD;b9rDgDg6bD

Move Red Dragon from pile 7 to pile 4
b7;g9;gD;;b2;;r5;rDg5bDg2b6;r8ffg1b4r6b5;g3bDg7r7;rD;bDb8g4gDr9;;g8gDrDb3;b9rDgDg6bD

Place Black 3
b7;g9;gD;;b3;;r5;rDg5bDg2b6;r8ffg1b4r6b5;g3bDg7r7;rD;bDb8g4gDr9;;g8gDrD;b9rDgDg6bD

Move Black 5 from pile 2 to pile 6
b7;g9;gD;;b3;;r5;rDg5bDg2b6;r8ffg1b4r6;g3bDg7r7;rD;bDb8g4gDr9;b5;g8gDrD;b9rDgDg6bD

Place Red 6
b7;g9;gD;;b3;;r6;rDg5bDg2b6;r8ffg1b4;g3bDg7r7;rD;bDb8g4gDr9;b5;g8gDrD;b9rDgDg6bD

Place Black 4
b7;g9;gD;;b4;;r6;rDg5bDg2b6;r8ffg1;g3bDg7r7;rD;bDb8g4gDr9;b5;g8gDrD;b9rDgDg6bD

Place Green 1
b7;g9;gD;;b4;g1;r6;rDg5bDg2b6;r8ff;g3bDg7r7;rD;bDb8g4gDr9;b5;g8gDrD;b9rDgDg6bD

Flower power!
b7;g9;gD;ff;b4;g1;r6;rDg5bDg2b6;r8;g3bDg7r7;rD;bDb8g4gDr9;b5;g8gDrD;b9rDgDg6bD

Place Black 5
b7;g9;gD;ff;b5;g1;r6;rDg5bDg2b6;r8;g3bDg7r7;rD;bDb8g4gDr9;;g8gDrD;b9rDgDg6bD

Place Black 6
b7;g9;gD;ff;b6;g1;r6;rDg5bDg2;r8;g3bDg7r7;rD;bDb8g4gDr9;;g8gDrD;b9rDgDg6bD

Place Green 2
b7;g9;gD;ff;b6;g2;r6;rDg5bD;r8;g3bDg7r7;rD;bDb8g4gDr9;;g8gDrD;b9rDgDg6bD

Place Black 7
;g9;gD;ff;b7;g2;r6;rDg5bD;r8;g3bDg7r7;rD;bDb8g4gDr9;;g8gDrD;b9rDgDg6bD

Place Red 7
;g9;gD;ff;b7;g2;r7;rDg5bD;r8;g3bDg7;rD;bDb8g4gDr9;;g8gDrD;b9rDgDg6bD

Place Red 8
;g9;gD;ff;b7;g2;r8;rDg5bD;;g3bDg7;rD;bDb8g4gDr9;;g8gDrD;b9rDgDg6bD

Place Red 9
;g9;gD;ff;b7;g2;r9;rDg5bD;;g3bDg7;rD;bDb8g4gD;;g8gDrD;b9rDgDg6bD

Move Green 7 from pile 3 to spares
g7;g9;gD;ff;b7;g2;r9;rDg5bD;;g3bD;rD;bDb8g4gD;;g8gDrD;b9rDgDg6bD

Move Black Dragon from pile 3 to pile 2
g7;g9;gD;ff;b7;g2;r9;rDg5bD;bD;g3;rD;bDb8g4gD;;g8gDrD;b9rDgDg6bD

Place Green 3
g7;g9;gD;ff;b7;g3;r9;rDg5bD;bD;;rD;bDb8g4gD;;g8gDrD;b9rDgDg6bD

Move Green Dragon from pile 5 to pile 3
g7;g9;gD;ff;b7;g3;r9;rDg5bD;bD;gD;rD;bDb8g4;;g8gDrD;b9rDgDg6bD

Place Green 4
g7;g9;gD;ff;b7;g4;r9;rDg5bD;bD;gD;rD;bDb8;;g8gDrD;b9rDgDg6bD

Place Black 8
g7;g9;gD;ff;b8;g4;r9;rDg5bD;bD;gD;rD;bD;;g8gDrD;b9rDgDg6bD

Move Green 7 from spares to pile 6
;g9;gD;ff;b8;g4;r9;rDg5bD;bD;gD;rD;bD;g7;g8gDrD;b9rDgDg6bD

Stack Black Dragons
bDbDbDbD;g9;gD;ff;b8;g4;r9;rDg5;;gD;rD;;g7;g8gDrD;b9rDgDg6

Place Green 5
bDbDbDbD;g9;gD;ff;b8;g5;r9;rD;;gD;rD;;g7;g8gDrD;b9rDgDg6

Place Green 6
bDbDbDbD;g9;gD;ff;b8;g6;r9;rD;;gD;rD;;g7;g8gDrD;b9rDgD

Place Green 7
bDbDbDbD;g9;gD;ff;b8;g7;r9;rD;;gD;rD;;;g8gDrD;b9rDgD

Move Red Dragon from pile 7 to pile 2
bDbDbDbD;g9;gD;ff;b8;g7;r9;rD;rD;gD;rD;;;g8gD;b9rDgD

Stack Green Dragons
bDbDbDbD;g9;gDgDgDgD;ff;b8;g7;r9;rD;rD;;rD;;;g8;b9rD

Place Green 8
bDbDbDbD;g9;gDgDgDgD;ff;b8;g8;r9;rD;rD;;rD;;;;b9rD

Place Green 9
bDbDbDbD;;gDgDgDgD;ff;b8;g9;r9;rD;rD;;rD;;;;b9rD

Stack Red Dragons
bDbDbDbD;rDrDrDrD;gDgDgDgD;ff;b8;g9;r9;;;;;;;;b9

Place Black 9
bDbDbDbD;rDrDrDrD;gDgDgDgD;ff;b9;g9;r9;;;;;;;;
```