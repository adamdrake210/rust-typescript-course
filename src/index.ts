// const list = [1, 2, 3];
// const newList = list.map((x) => x + 1);

// console.log('newList: ', newList);

// import fs from 'fs';

// const file = fs.readFileSync('lines', 'utf8');

// const lines = file.split('\n');

// lines.forEach((line: string) => {
//   console.log(line);
// });

// function isNumber(value: number | undefined) {
//   return value === undefined ? 0 : value * 5;
// }


// console.log(isNumber(5));
// console.log(isNumber(undefined));

import fs from "fs";

const fileName = process.argv[2];

// @ts-ignore
fs.readFileSync(fileName).
  toString().
  split('\n').
  forEach(line => console.log(line))