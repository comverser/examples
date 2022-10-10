import React, { useState } from "react";

const SingleUseState = () => {
  const [user, setUser] = useState({
    name: "",
    surname: "",
    username: "",
    email: "",
    password: "",
    country: "",
    city: "",
    address: "",
  });

  const handleChange = (e) => {
    setUser((prev) => ({ ...prev, [e.target.name]: e.target.value }));
  };

  console.log(user);

  return (
    <div className="topic">
      <form action="">
        <input
          type="text"
          onChange={handleChange}
          name="name"
          placeholder="name"
        />
        <input
          type="text"
          onChange={handleChange}
          name="surname"
          placeholder="surname"
        />
        <input
          type="text"
          onChange={handleChange}
          name="username"
          placeholder="username"
        />
        <input
          type="text"
          onChange={handleChange}
          name="email"
          placeholder="email"
        />
        <input
          type="text"
          onChange={handleChange}
          name="password"
          placeholder="password"
        />
        <input
          type="text"
          onChange={handleChange}
          name="country"
          placeholder="country"
        />
        <input
          type="text"
          onChange={handleChange}
          name="city"
          placeholder="city"
        />
        <input
          type="text"
          onChange={handleChange}
          name="address"
          placeholder="address"
        />
      </form>
    </div>
  );
};

export default SingleUseState;
