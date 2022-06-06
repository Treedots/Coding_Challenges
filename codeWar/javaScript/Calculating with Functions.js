let zero = (ops) => ops==undefined?0:ops(0);
let one = (ops) => ops==undefined?1:ops(1);
let two = (ops) => ops==undefined?2:ops(2);
let three = (ops) => ops==undefined?3:ops(3);
let four = (ops) => ops==undefined?4:ops(4);
let five = (ops) => ops==undefined?5:ops(5);
let six = (ops) => ops==undefined?6:ops(6);
let seven = (ops) => ops==undefined?7:ops(7);
let eight = (ops) => ops==undefined?8:ops(8);
let nine = (ops) => ops==undefined?9:ops(9);

let plus = (x) => function(y){return Math.floor(x+y)};
let minus = (x) => function(y){return Math.floor(y-x)};
let times = (x) => function(y){return Math.floor(x*y)}; 
let dividedBy = (x) => function(y){return Math.floor(y/x)};