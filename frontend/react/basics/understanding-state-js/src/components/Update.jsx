import React, { useState } from "react";

const UpdateObject = () => {
  const [user, setUser] = useState({
    username: "john",
    email: "john@gmail.com",
    images: ["profile.png", "cover.png"],
  });
  const [input, setInput] = useState("");

  const changeUser = () => {
    setUser((prev) => ({ ...prev, name: input }));
  };

  console.log(user, input);

  return (
    <div className="topic">
      <h2>Update a specific object property</h2>
      <input
        onChange={(e) => setInput(e.target.value)}
        type="text"
        placeholder="enter a new name..."
      />
      <button onClick={changeUser}>Change username</button>
      <span>Username is: {user.name}</span>
    </div>
  );
};

export default UpdateObject;
