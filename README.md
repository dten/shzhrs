#shzhrs

[![Travis CI Status](https://travis-ci.org/dten/shzhrs.svg?branch=master)](https://travis-ci.org/dten/shzhrs)

Usage:

`cargo run --release --example solve`

```
Enter board:
;;;;;;;rDg5bDg2b6;r8ffg1b4r6;g3bDg7r7r4;r5b1gDr2b2;bDb8g4gDr9;r3g9r1b7b5;g8gDrDb3rD;b9rDgDg6bD
Solved in Duration { secs: 0, nanos: 50147163 }

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

Move Black 5 from pile 2 to pile 6
b7;g9;gD;;b2;;r4;rDg5bDg2b6;r8ffg1b4r6;g3bDg7r7;r5;bDb8g4gDr9;b5;g8gDrDb3rD;b9rDgDg6bD

Place Red 5
b7;g9;gD;;b2;;r5;rDg5bDg2b6;r8ffg1b4r6;g3bDg7r7;;bDb8g4gDr9;b5;g8gDrDb3rD;b9rDgDg6bD

Place Red 6
b7;g9;gD;;b2;;r6;rDg5bDg2b6;r8ffg1b4;g3bDg7r7;;bDb8g4gDr9;b5;g8gDrDb3rD;b9rDgDg6bD

Move Black 4 from pile 2 to pile 4
b7;g9;gD;;b2;;r6;rDg5bDg2b6;r8ffg1;g3bDg7r7;b4;bDb8g4gDr9;b5;g8gDrDb3rD;b9rDgDg6bD

Place Green 1
b7;g9;gD;;b2;g1;r6;rDg5bDg2b6;r8ff;g3bDg7r7;b4;bDb8g4gDr9;b5;g8gDrDb3rD;b9rDgDg6bD

Flower power!
b7;g9;gD;ff;b2;g1;r6;rDg5bDg2b6;r8;g3bDg7r7;b4;bDb8g4gDr9;b5;g8gDrDb3rD;b9rDgDg6bD

Move Black 6 from pile 1 to pile 3
b7;g9;gD;ff;b2;g1;r6;rDg5bDg2;r8;g3bDg7r7b6;b4;bDb8g4gDr9;b5;g8gDrDb3rD;b9rDgDg6bD

Place Green 2
b7;g9;gD;ff;b2;g2;r6;rDg5bD;r8;g3bDg7r7b6;b4;bDb8g4gDr9;b5;g8gDrDb3rD;b9rDgDg6bD

Move Black 7 from spares to pile 2
;g9;gD;ff;b2;g2;r6;rDg5bD;r8b7;g3bDg7r7b6;b4;bDb8g4gDr9;b5;g8gDrDb3rD;b9rDgDg6bD

Move Red Dragon from pile 7 to spares
rD;g9;gD;ff;b2;g2;r6;rDg5bD;r8b7;g3bDg7r7b6;b4;bDb8g4gDr9;b5;g8gDrDb3;b9rDgDg6bD

Place Black 3
rD;g9;gD;ff;b3;g2;r6;rDg5bD;r8b7;g3bDg7r7b6;b4;bDb8g4gDr9;b5;g8gDrD;b9rDgDg6bD

Place Black 4
rD;g9;gD;ff;b4;g2;r6;rDg5bD;r8b7;g3bDg7r7b6;;bDb8g4gDr9;b5;g8gDrD;b9rDgDg6bD

Place Black 5
rD;g9;gD;ff;b5;g2;r6;rDg5bD;r8b7;g3bDg7r7b6;;bDb8g4gDr9;;g8gDrD;b9rDgDg6bD

Move Green Dragon from spares to pile 4
rD;g9;;ff;b5;g2;r6;rDg5bD;r8b7;g3bDg7r7b6;gD;bDb8g4gDr9;;g8gDrD;b9rDgDg6bD

Move Red 9 from pile 5 to pile 6
rD;g9;;ff;b5;g2;r6;rDg5bD;r8b7;g3bDg7r7b6;gD;bDb8g4gD;r9;g8gDrD;b9rDgDg6bD

Move Green Dragon from pile 5 to spares
rD;g9;gD;ff;b5;g2;r6;rDg5bD;r8b7;g3bDg7r7b6;gD;bDb8g4;r9;g8gDrD;b9rDgDg6bD

Place Black 6
rD;g9;gD;ff;b6;g2;r6;rDg5bD;r8b7;g3bDg7r7;gD;bDb8g4;r9;g8gDrD;b9rDgDg6bD

Place Black 7
rD;g9;gD;ff;b7;g2;r6;rDg5bD;r8;g3bDg7r7;gD;bDb8g4;r9;g8gDrD;b9rDgDg6bD

Place Red 7
rD;g9;gD;ff;b7;g2;r7;rDg5bD;r8;g3bDg7;gD;bDb8g4;r9;g8gDrD;b9rDgDg6bD

Place Red 8
rD;g9;gD;ff;b7;g2;r8;rDg5bD;;g3bDg7;gD;bDb8g4;r9;g8gDrD;b9rDgDg6bD

Move Green 7 from pile 3 to pile 2
rD;g9;gD;ff;b7;g2;r8;rDg5bD;g7;g3bD;gD;bDb8g4;r9;g8gDrD;b9rDgDg6bD

Place Red 9
rD;g9;gD;ff;b7;g2;r9;rDg5bD;g7;g3bD;gD;bDb8g4;;g8gDrD;b9rDgDg6bD

Move Green Dragon from spares to pile 6
rD;g9;;ff;b7;g2;r9;rDg5bD;g7;g3bD;gD;bDb8g4;gD;g8gDrD;b9rDgDg6bD

Move Black Dragon from pile 3 to spares
rD;g9;bD;ff;b7;g2;r9;rDg5bD;g7;g3;gD;bDb8g4;gD;g8gDrD;b9rDgDg6bD

Place Green 3
rD;g9;bD;ff;b7;g3;r9;rDg5bD;g7;;gD;bDb8g4;gD;g8gDrD;b9rDgDg6bD

Place Green 4
rD;g9;bD;ff;b7;g4;r9;rDg5bD;g7;;gD;bDb8;gD;g8gDrD;b9rDgDg6bD

Place Black 8
rD;g9;bD;ff;b8;g4;r9;rDg5bD;g7;;gD;bD;gD;g8gDrD;b9rDgDg6bD

Stack Black Dragons
rD;g9;bDbDbDbD;ff;b8;g4;r9;rDg5;g7;;gD;;gD;g8gDrD;b9rDgDg6

Place Green 5
rD;g9;bDbDbDbD;ff;b8;g5;r9;rD;g7;;gD;;gD;g8gDrD;b9rDgDg6

Place Green 6
rD;g9;bDbDbDbD;ff;b8;g6;r9;rD;g7;;gD;;gD;g8gDrD;b9rDgD

Place Green 7
rD;g9;bDbDbDbD;ff;b8;g7;r9;rD;;;gD;;gD;g8gDrD;b9rDgD

Move Green 9 from spares to pile 2
rD;;bDbDbDbD;ff;b8;g7;r9;rD;g9;;gD;;gD;g8gDrD;b9rDgD

Move Green Dragon from pile 8 to spares
rD;gD;bDbDbDbD;ff;b8;g7;r9;rD;g9;;gD;;gD;g8gDrD;b9rD

Stack Red Dragons
rDrDrDrD;gD;bDbDbDbD;ff;b8;g7;r9;;g9;;gD;;gD;g8gD;b9

Place Black 9
rDrDrDrD;gD;bDbDbDbD;ff;b9;g7;r9;;g9;;gD;;gD;g8gD;

Stack Green Dragons
rDrDrDrD;gDgDgDgD;bDbDbDbD;ff;b9;g7;r9;;g9;;;;;g8;

Place Green 8
rDrDrDrD;gDgDgDgD;bDbDbDbD;ff;b9;g8;r9;;g9;;;;;;

Place Green 9
rDrDrDrD;gDgDgDgD;bDbDbDbD;ff;b9;g9;r9;;;;;;;;
```