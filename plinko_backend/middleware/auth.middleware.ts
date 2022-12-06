import { PublicKey } from '@solana/web3.js';
import { Request, Response, NextFunction } from 'express';
import nacl from 'tweetnacl';
import { authorizedAdmins } from '../constants';

export function authorized(req: Request, res: Response, next: NextFunction) {
    const { message, wallet, signature } = req.body;

    if (message && wallet && signature) {
        const verified = nacl.sign.detached.verify(
            new Uint8Array(Buffer.from(message)), 
            signature, 
            new PublicKey(wallet).toBytes()
        );
        if (verified) {
            next();
        }
    }

    res.status(402).json("Unauthorized Wallet");
}

export function isAdmin(req: Request, res: Response, next: NextFunction) {
    const { wallet } = req.body;

    if (authorizedAdmins.includes(wallet)) {
        next();
    }

    res.status(402).json("Unauthorized Admin");
}