import express from "express";
import mongoose from "mongoose";
import cookieParser from "cookie-parser";
import authRoutes from "./routes/authRoutes";
import authMiddleware from "./middleware/authMiddleware";

const app = express();

/* middleware */
app.use(express.static("public"));
app.use(express.json());
app.use(cookieParser());

/* view engine */
app.set("view engine", "ejs");

/* database connection */
const dbURI = "mongodb+srv://s:3434@cluster0.ebgu5.mongodb.net/node-auth";
mongoose
    .connect(dbURI)
    .then((result) => app.listen(3000))
    .catch((err) => console.log(err));

/* routes */
app.get("*", authMiddleware.checkUser);
app.get("/", (req, res) => res.render("home"));
app.get("/smoothies", authMiddleware.requireAuth, (req, res) =>
    res.render("smoothies")
);
app.use(authRoutes);

/* cookies */
// app.get("/set-cookies", (req, res) => {
//     res.cookie("newUser", false); // res.setHeader("Set-Cookie", "newUser=false");
//     res.cookie("isEmployee", true, {
//         maxAge: 1000 * 60 * 60 * 24,
//         httpOnly: true,
//         secure: true,
//     });
//     res.send("you got the cookies!");
// });

// app.get("/read-cookies", (req, res) => {
//     const cookies = req.cookies;
//     console.log(cookies.newUser);

//     res.json(cookies);
// });
