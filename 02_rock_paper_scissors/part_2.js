let fs = require('fs');
let input = fs.readFileSync('input.txt').toString('utf-8').split('\r\n');

// x = lose, y = tie, z = win

class Player1 {
    constructor(choice) {
        this.choice = choice
    }
}

class Player2 {
    constructor(choice, score) {
        this.choice = choice
        this.score = score
    }
}

let p = {
    'A': new Player1('rock'),
    'B': new Player1('paper'),
    'C': new Player1('scissors'),
    'X': new Player2('lose', 0),
    'Y': new Player2('tie', 3),
    'Z': new Player2('win', 6)
}

for (let i = 0; i < input.length; i++) { 
    input[i] = input[i].split(' ')
}

for (let i = 0; i < input.length; i++) { 
    for (let j = 0; j < input[i].length; j++) { 
        switch (input[i][j]) { 
            case 'A':
                input[i][j] = p['A']
                break;
            case 'B':
                input[i][j] = p['B']
                break;
            case 'C':
                input[i][j] = p['C']
                break;
            case 'X':
                input[i][j] = p['X']
                break;
            case 'Y':
                input[i][j] = p['Y']
                break;
            case 'Z':
                input[i][j] = p['Z']
                break;
            default:
                break;
        }
    }
}

let rock = 1
let paper = 2
let scissors = 3
console.log(input)
let score = 0;

for (let i = 0; i < input.length; i++) { 
    let enemy = input[i][0]
    let you = input[i][1]
    switch (enemy.choice) {
        case 'rock':
            switch (you.choice) {
                case 'lose':
                    score += scissors + you.score
                    break;
                case 'tie':
                    score += rock + you.score
                    break;
                case 'win':
                    score += paper + you.score
                    break;
            }
            break;
        case 'paper':
            switch (you.choice) {
                case 'lose':
                    score += rock + you.score
                    break;
                case 'tie':
                    score += paper + you.score
                    break;
                case 'win':
                    score += scissors + you.score
                    break;
            }
            break;
        case 'scissors':
            switch (you.choice) {
                case 'lose':
                    score += paper + you.score
                    break;
                case 'win':
                    score += rock + you.score
                    break;
                case 'tie':
                    score += scissors + you.score
                    break;
            }
            break;
    }
}

console.log(score)