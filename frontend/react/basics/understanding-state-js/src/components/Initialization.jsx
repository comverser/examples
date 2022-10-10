import React, { useState } from "react";

const Initialization = () => {
  const [user, setUser] = useState({
    username: "john",
    email: "john@gmail.com",
    images: ["profile.png", "cover.png"],
  });

  return (
    <div className="topic">
      <h2>Cannot read properties of undefined error</h2>
      {/* Solution 1 */}
      {user && <span>Username is: {user.name}</span>}
      {/* Solution 2 */}
      <span>Username is: {user?.name}</span>
      {/* Solution 3: give initial value */}
      <span>Username is: {user.name}</span>
      <span>Profile picture is: {user.images[1]}</span>
    </div>
  );
};

export default Initialization;
