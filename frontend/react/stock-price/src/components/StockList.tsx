import React, { useContext } from "react";
import PropTypes from "prop-types";
import { useState, useEffect } from "react";

import { WatchListContext } from "../context/watchListContext";
import { BsFillCaretUpFill, BsFillCaretDownFill } from "react-icons/bs";
import finnHub from "../apis/finnHub";

type StockType = {
  data: string;
  symbol: string;
};

export const StockList = () => {
  const [stock, setStock] = useState<StockType[]>();
  const { watchList } = useContext(WatchListContext);

  const changeColor = (change: number) => {
    return change > 0 ? "success" : "danger";
  };
  const renderIcon = (change: number) => {
    return change > 0 ? <BsFillCaretUpFill /> : <BsFillCaretDownFill />;
  };

  useEffect(() => {
    let isMounted = true;
    const fetchData = async () => {
      try {
        const responses = await Promise.all(
          watchList.map((stock) => {
            return finnHub.get("/quote", {
              params: { symbol: stock },
            });
          })
        );

        const data = responses.map((response): StockType => {
          return {
            data: response.data,
            symbol: response.config.params.symbol,
          };
        });

        // setStock(data);
        if (isMounted) {
          console.debug("StockList.tsx is mounted");
          setStock(data);
        } else {
          console.debug("Does such a case exist?");
        }
      } catch (err) {}
    };

    fetchData();

    return () => {
      isMounted = false;
    };
  }, [watchList]);

  return (
    <div>
      <table className="table hover mt-5">
        <thead style={{ color: "rgb(79,89,102)" }}>
          <tr>
            <th scope="col">Name</th>
            <th scope="col">Last</th>
            <th scope="col">Chg</th>
            <th scope="col">Chg%</th>
            <th scope="col">High</th>
            <th scope="col">Low</th>
            <th scope="col">Open</th>
            <th scope="col">Pclose</th>
          </tr>
        </thead>
        <tbody>
          {stock?.map((stockData) => {
            return (
              <tr className="table-row" key={stockData.symbol}>
                <th scope="row">{stockData.symbol}</th>
                <td>{stockData.data.c}</td>
                <td className={`text-${changeColor(stockData.data.d)}`}>
                  {stockData.data.d}
                  {renderIcon(stockData.data.d)}
                </td>
                <td className={`text-${changeColor(stockData.data.dp)}`}>
                  {stockData.data.dp}

                  {renderIcon(stockData.data.dp)}
                </td>
                <td>{stockData.data.h}</td>
                <td>{stockData.data.l}</td>
                <td>{stockData.data.o}</td>
                <td>{stockData.data.pc}</td>
              </tr>
            );
          })}
        </tbody>
      </table>
    </div>
  );
};

StockList.propTypes = {};
