import { Request, Response } from 'express';
const settings = require('../settings.json');

const getLineIndex = (lines: number) => {
    return [8, 12, 16].indexOf(lines);
}

const random = () => {
    return Math.floor(Math.random() * 10000);
}

export const getPlayStatus = (req: Request, res: Response) => {
    const { wallet, lines, risk, betValue, ballCount } = req.body;
    // Get Balance of Wallet from DB
    let balance = 100;

    let lineIndex = getLineIndex(lines);
    let maxBallCount = balance / betValue;
    if (maxBallCount < ballCount) {
        maxBallCount = ballCount;
    }

    balance -= maxBallCount * betValue;

    const result = [];
    const chance = settings.chance[risk][lineIndex];
    const multiplier = settings.multiplier[risk][lineIndex];
    
    chance.forEach((percent: number, index: number) => {
        if (index) {
            chance[index] = chance[index - 1] + percent;
        }
    });

    for (let i = 0; i < maxBallCount; i++) {
        let target = 0;
        for (let j = 0; j < lines; j++) {            
            let rand = random();
            if (chance[j] > rand && rand > (j ? chance[j] : 0)) {
                target = j;
                break;
            }
        }
        result[i] = target;
        balance = balance + multiplier[target] * betValue;
    }

    // Update balance in the DB
    
    return res.json(result);
}