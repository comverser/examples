import React from "react";
import ReactDOM from "react-dom";

import "./index.scss";

import Footer from "home/Footer";
import Header from "home/Header";

const App = () => (

  <div className="text-3xl mx-auto max-w-6xl">
  <Header />
  <div className="my-10">
    PDP Page Content
  </div>
  <Footer />
  </div>


);
ReactDOM.render(<App />, document.getElementById("app"));
