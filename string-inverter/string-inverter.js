const rl = require('../console-input');

console.log("------- INVERSOR DE STRING ------- ")

rl.question("Digite o texto: ", (text) => {
    let invertedText = "";
    for (const char of text) {
        invertedText = char + invertedText
    }
    
    console.log(invertedText);
    rl.close();
});

