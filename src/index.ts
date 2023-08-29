const fs = require("fs");

// String(fs.readFileSync("lines"))
//   .split("\n")
//   .filter((_, index) => index % 2 === 0)
//   .filter((_, index) => index > 1 && index < 4)
//   .map((line) => console.log(line));

// enum Color {
//   Red,
//   Green,
//   Blue,
//   Yellow,
// }

// function printColor(color: Color) {
//   switch (color) {
//     case Color.Red:
//       console.log("red");
//       break;
//     case Color.Green:
//       console.log("green");
//       break;
//     case Color.Blue:
//       console.log("blue");
//   }
// }

// printColor(Color.Red);

// type Custom = {
//   age: number;
//   name: string;
// };

// type Item = number | string | Custom;

// function append(items: Item[]) {
//   items.push("Hello Fem");
// }

// const items: Item[] = [];
// const numbers: number[] = [1, 2, 3];

// append(items);
// append(numbers);

// console.log(numbers);

// function thing(value: number | undefined): number | undefined {
//   return value === undefined ? undefined : value * 5;
// }

// function practice(nums: number[], index: number) {
//   return (nums[index] ?? index) * 5;
// }

const fileName = process.argv[2];

fs.readFileSync(fileName)
  .toString()
  .split("\n")
  .forEach((line: string) => {
    const print = parseInt(line);
    if (isNaN(print)) console.log("Line not a number");
    console.log(line);
  });
