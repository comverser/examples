import React from "react";
import PropTypes from "prop-types";

import { AutoComplete } from "../components/AutoComplete";
import { StockList } from "../components/StockList";

export const StockOverviewPage = () => {
  return (
    <div>
      StockOverviewPage
      <AutoComplete />
      <StockList />
    </div>
  );
};

StockOverviewPage.propTypes = {};
