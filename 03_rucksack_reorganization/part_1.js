let fs = require('fs');
let input = fs.readFileSync('input.txt').toString('utf-8').split('\r\n');

let letters = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ';

let priorities = {}

for (let i = 0; i < letters.length; i++) { 
    priorities[letters[i]] = i + 1
}

//Build a rucksack class which splits each string into two compartments
class Rucksack {
    constructor(input) {
        this.c1 = input.slice(0, input.length/2)
        this.c2 = input.slice(input.length/2, input.length)
    }
}

function checkRucksack(c1, c2) {
    let i = 0;
    while (i < c1.length) { 
        for (let j = 0; j < c2.length; j++) { 
            if (c1[i] === c2[j]) {
                return c1[i]
            }
        }
        i++;
    }
}

for (let i = 0; i < input.length; i++) {
    input[i] = new Rucksack(input[i])
    input[i] = checkRucksack(input[i].c1, input[i].c2)
}

let result = 0;

for (let i = 0; i < input.length; i++) { 
    result += priorities[input[i]]
}

console.log(result)