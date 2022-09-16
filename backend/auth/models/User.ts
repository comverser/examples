import mongoose from "mongoose";
import validator from "validator";
import bcrypt from "bcrypt";

const userSchema = new mongoose.Schema({
    email: {
        type: String,
        required: [true, "Please enter an email"],
        unique: true,
        lowercase: true,
        validate: [validator.isEmail, "Please enter a valid email"],
    },
    password: {
        type: String,
        required: [true, "Please enter an password"],
        minlength: [6, "Minimum password length is 6 characters"],
    },
});

/* fire a function after doc saved to db */
userSchema.post("save", function (doc, next) {
    console.log("new user was created & saved", doc);
    next();
});

/* fire a function before doc saved to db */
userSchema.pre("save", async function (next) {
    const salt = await bcrypt.genSalt();
    this.password = await bcrypt.hash(this.password, salt);
    next();
});

/* static method to login user */
userSchema.statics.login = async function (email, password) {
    const user = await this.findOne({ email });
    if (user) {
        const auth = await bcrypt.compare(password, user.password);
        if (auth) {
            return user;
        }
        throw Error("incorrect password");
    }
    throw Error("incorrect email");
};

/* manual type */
interface UserDocument {
    _id: string;
    email: string;
    password: string;
}

interface UserModel extends mongoose.Model<UserDocument> {
    login(email: string, password: string): UserDocument;
}

/* name should be singuler so that it is automatically connected to collection of plural name */
// const User = mongoose.model("user", userSchema);
const User: UserModel = mongoose.model<UserDocument, UserModel>(
    "user",
    userSchema
);

export default User;
