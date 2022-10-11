import { useEffect, useMemo, useState } from "react";

function UseMemo() {
  const [name, setName] = useState("");
  const [state, setState] = useState({
    name: "",
    selected: false,
  });

  // const user = useMemo(
  //   () => ({
  //     name: state.name,
  //     selected: state.selected,
  //   }),
  //   [state.name, state.selected]
  // );

  // useEffect(() => {
  //   console.log("the state has changed, useEffect runs!");
  // }, [user]);

  useEffect(() => {
    console.log("the state has changed, useEffect runs!");
  }, [state.name, state.selected]);

  const handleAdd = () => {
    setState((prev) => ({ ...prev, name }));
  };

  const handleSelect = () => {
    setState((prev) => ({ ...prev, selected: true }));
  };

  console.count("component rendered!");

  return (
    <div>
      <input onChange={(e) => setName(e.target.value)} type="text" />
      <button onClick={handleAdd}>Add Name</button>
      <button onClick={handleSelect}>Select</button>
      {`{
        name: ${state.name},
        selected: ${state.selected.toString()}
      }`}
    </div>
  );
}

export default UseMemo;
