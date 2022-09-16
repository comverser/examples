import React, { useState } from "react";
import { useGlobalContext } from "../context";

const Search = () => {
  const [text, setText] = useState("");

  const { setSearchTerm, fetchRandomMeal } = useGlobalContext()!;

  const handleRandomMeal = () => {
    setSearchTerm("");
    setText("");
    fetchRandomMeal();
  };

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setText(e.target.value);
  };
  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    if (text) {
      setSearchTerm(text);
    }
  };

  return (
    <header className="search-container">
      <form onSubmit={handleSubmit}>
        <input
          type="text"
          onChange={handleChange}
          value={text}
          placeholder="type favorite meal"
          className="form-input"
        />
        <button type="submit" className="btn">
          search
        </button>
        <button
          type="btn"
          className="btn btn-hipster"
          onClick={handleRandomMeal}
        >
          suprise me !
        </button>
      </form>
    </header>
  );
};

export default Search;
