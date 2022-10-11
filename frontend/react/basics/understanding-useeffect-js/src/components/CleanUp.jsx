import React, { useEffect, useState } from "react";

const CleanUp = () => {
  const [number, setNumber] = useState(0);

  useEffect(() => {
    console.log("effect runs!");
    const interval = setInterval(() => {
      console.log("number :>> ", number);
      setNumber((prev) => prev + 1);
    }, 1000);

    // return a clean up function
    return () => {
      console.log("wait! befroe running the effect, I should clean here!");
      clearInterval(interval);
      console.log("okey done! you can run!");
    };
  }, []);

  return (
    <div>
      <div>{number}cleaning</div>
    </div>
  );
};

export default CleanUp;
