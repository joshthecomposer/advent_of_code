let fs = require('fs');
let path = require("path");
let input = fs.readFileSync(path.resolve(__dirname, 'input.txt')).toString('utf-8').split('\r\n');

let groups = {};

let cursor = 0;
let groupNumber = 0;
while (cursor < input.length) {
    groups[groupNumber] = [input[cursor], input[cursor + 1], input[cursor + 2]];
    cursor += 3;
    groupNumber++;
}

let letters = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ';

let score = 0;

function findCommonChars(string1, string2, string3) {

    for (let i = 0; i < string1.length; i++) {
        if (string2.includes(string1[i]) && string3.includes(string1[i])) {
            score += (letters.indexOf(string1[i]) + 1)
            return;
        }
    }
}

for (g in groups) {
    findCommonChars(groups[g][0], groups[g][1], groups[g][2])
}

console.log(score)


