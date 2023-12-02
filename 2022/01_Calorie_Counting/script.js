let fs = require('fs');
let input = fs.readFileSync('input.txt').toString('utf-8').split('\r\n');


function toNumbers(input) {
    for (let i = 0; i < input.length; i++) {
        if (input[i] !== '') {
            input[i] = Number(input[i]);
        }
    }
    return input
}

input = toNumbers(input)

function calories(input) {
    let calories = 0;
    let elves = []
    let result = 0
    for (let i = 0; i < input.length; i++) {
        if (input[i] === '') { 
            elves.push(calories)
            calories = 0
            continue
        }
        calories += input[i];
    }
    for (let i = 0; i < elves.length; i++) { 
        if (elves[i] > result) {
            result = elves[i];
        }
    }
    return elves
}


const selectionSort = (arr) =>{
    let counter = 0;
    while (counter < arr.length) { 
        shortest = arr[counter]
        position = counter
        for (let i = counter; i < arr.length; i++) { 
            if (arr[i] < shortest) {
                position = i;
                shortest = arr[i];
            }
        }
        let temp = arr[counter]
        arr[counter] = arr[position];
        arr[position] = temp;
        counter++;
    }
    return arr;
}



let elves = calories(input);
let arr = selectionSort(elves);

// let topThree = [arr[arr.length - 1], arr[arr.length - 2], arr[arr.length - 3]]

console.log(arr)