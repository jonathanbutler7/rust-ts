const fs = require("fs");

String(fs.readFileSync("lines"))
  .split("\n")
  .filter((_, index) => index % 2 === 0)
  .filter((_, index) => index > 1 && index < 4)
  .map((line) => console.log(line));
