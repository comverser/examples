import React, { useState } from "react";

const FunctionalUpdate = () => {
  const [number, setNumber] = useState(0);

  const increase = () => {
    setNumber(number + 1);
  };

  const increaseAsync = () => {
    setTimeout(() => {
      setNumber((prev) => prev + 1);
    }, 2000);
  };

  return (
    <div className="App">
      <button onClick={increase}>increase</button>
      <button onClick={increaseAsync}>increaseAsync</button>
      <h1>{number}</h1>
    </div>
  );
};

export default FunctionalUpdate;
