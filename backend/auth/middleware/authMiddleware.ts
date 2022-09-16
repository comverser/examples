import { Request, Response, NextFunction } from "express";
import jwt from "jsonwebtoken";
import User from "../models/User";

const requireAuth = (req: Request, res: Response, next: NextFunction) => {
    const token = req.cookies.jwt;

    /* check json web token exists & is verified */
    if (token) {
        jwt.verify(token, "HShin secret", (err: any, decodedToken: any) => {
            if (err) {
                console.log(err.message);
                res.redirect("/login");
            } else {
                console.log(decodedToken);
                next();
            }
        });
    } else {
        res.redirect("/login");
    }
};

const checkUser = (req: Request, res: Response, next: NextFunction) => {
    const token = req.cookies.jwt;

    if (token) {
        jwt.verify(
            token,
            "HShin secret",
            async (err: any, decodedToken: any) => {
                if (err) {
                    console.log(err.message);
                    res.locals.user = null;
                    next();
                } else {
                    console.log(decodedToken);
                    let user = await User.findById(decodedToken.id);
                    res.locals.user = user;
                    next();
                }
            }
        );
    } else {
        res.locals.user = null;
        next();
    }
};

export default { requireAuth, checkUser };
