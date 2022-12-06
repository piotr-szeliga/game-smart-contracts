import { Request, Response } from "express"
import fs from 'fs';

const settings = require("../settings.json");

export const getSettings = (req: Request, res: Response) => {
    return res.json({ multiplier: settings.multiplier });
}

export const getAdminSettings = (req: Request, res: Response) => {
    return res.json(settings);
}

export const setSettings = (req: Request, res: Response) => {
    const { multiplier, chance } = req.body;
    const settingsContent = JSON.stringify({ multiplier, chance });
    fs.writeFile("../settings.json", settingsContent, (err) => {
        if (err) {
            console.log(err);
            return res.status(500).json("Failed to save settings");
        }
        console.log("JSON file has been saved.");
    });
    res.json("Success");
}