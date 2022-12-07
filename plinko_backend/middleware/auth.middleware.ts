import { PublicKey } from '@solana/web3.js';
import { Request, Response, NextFunction } from 'express';
import nacl from 'tweetnacl';
import { authorizedAdmins } from '../constants';
import jwt from 'jsonwebtoken';

export function authorizedPlayer(req: Request, res: Response, next: NextFunction) {
    const authorizationHeader = req.headers['authorization'];
    
    let token;
    if (authorizationHeader) {
        token = authorizationHeader.split(' ')[1];
    }
    if (token) {
        const { payload } = jwt.verify(token, process.env.TOKEN_SECRET_KEY || '', { complete: true });
        if (payload) {
            // @ts-ignore
            const { message, wallet, signature } = payload;
            const verified = nacl.sign.detached.verify(
                new Uint8Array(Buffer.from(message)), 
                signature, 
                new PublicKey(wallet).toBytes()
            );
            if (verified) {
                next();
                return;
            }
        }
        
    }
    return res.status(402).json("Unauthorized Wallet");
}

export function authorizedAdmin(req: Request, res: Response, next: NextFunction) {
    const authorizationHeader = req.headers['authorization'];
    
    let token;
    if (authorizationHeader) {
        token = authorizationHeader.split(' ')[1];
    }
    if (token) {
        const { payload } = jwt.verify(token, process.env.TOKEN_SECRET_KEY || '', { complete: true });
        if (payload) {
            // @ts-ignore
            const { message, wallet, signature } = payload;
            const verified = nacl.sign.detached.verify(
                new Uint8Array(Buffer.from(message)), 
                new Uint8Array(Buffer.from(signature)), 
                new PublicKey(wallet).toBytes()
            );
            if (verified && authorizedAdmins.includes(wallet)) {
                next();
                return;
            }
        }
        
    }
    return res.status(402).json("Unauthorized Wallet");
}