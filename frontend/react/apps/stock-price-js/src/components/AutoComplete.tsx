import React, { useContext, useEffect, useState } from "react";

import { WatchListContext } from "../context/watchListContext";
import finnHub from "../apis/finnHub";

export const AutoComplete = () => {
  const [search, setSearch] = useState("");
  const [results, setResults] = useState([]);
  const { addStock } = useContext(WatchListContext);

  const renderDropdown = () => {
    const dropDownClass = search ? "show" : null;

    return (
      <ul
        style={{
          height: "500px",
          overflowY: "scroll",
          overflowX: "hidden",
          cursor: "pointer",
        }}
        className={`dropdown-menu ${dropDownClass}`}
      >
        {results.map((result: { description: string; symbol: string }) => {
          return (
            <li
              key={result.symbol}
              className="dropdown-item"
              onClick={() => {
                addStock(result.symbol);
                setSearch("");
              }}
            >
              {result.description} ({result.symbol})
            </li>
          );
        })}
      </ul>
    );
  };

  useEffect(() => {
    const fetchData = async () => {
      try {
        const response = await finnHub.get("search", {
          params: {
            q: search,
          },
        });

        setResults(response.data.result);
      } catch (err) {}
    };

    if (search.length > 0) {
      fetchData();
    } else {
      setResults([]);
    }
  }, [search]);

  return (
    <div className="w-50 p-5 rounded mx-auto">
      <div className="form-floating dropdown">
        <input
          style={{ backgroundColor: "rgba(145, 158, 171, 0.04" }}
          id="search"
          className="form-control"
          type="text"
          placeholder="Search"
          autoComplete="off"
          value={search}
          onChange={(e) => setSearch(e.target.value)}
        />
        <label htmlFor="search">Search</label>
        {renderDropdown()}
      </div>
    </div>
  );
};
