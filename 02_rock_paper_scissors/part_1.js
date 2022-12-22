let fs = require('fs');
let input = fs.readFileSync('input.txt').toString('utf-8').split('\r\n');

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
    'X': new Player2('rock', 1),
    'Y': new Player2('paper', 2),
    'Z': new Player2('scissors', 3)
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
let loss = 0
let tie = 3
let victory = 6
console.log(input)
let score = 0;

for (let i = 0; i < input.length; i++) {
    let enemy = input[i][0]
    let you = input[i][1]
    switch (enemy.choice) {
        case 'rock':
            switch (you.choice) {
                case 'rock':
                    score += tie + you.score
                    break;
                case 'paper':
                    score += victory + you.score
                    break;
                case 'scissors':
                    score += loss + you.score
                    break;
            }
            break;
        case 'paper':
            switch (you.choice) {
                case 'rock':
                    score += loss + you.score
                    break;
                case 'paper':
                    score += tie + you.score
                    break;
                case 'scissors':
                    score += victory + you.score
                    break;
            }
            break;
        case 'scissors':
            switch (you.choice) {
                case 'rock':
                    score += victory + you.score
                    break;
                case 'paper':
                    score += loss + you.score
                    break;
                case 'scissors':
                    score += tie + you.score
                    break;
            }
            break;
    }
}

console.log(score)