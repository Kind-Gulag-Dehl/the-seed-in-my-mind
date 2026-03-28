import React from "react";
import ReactDOM from "react-dom/client";
import { ReferenceApp } from "./referenceapp";

const root = document.getElementById("root");
if (!root) {
  throw new Error("Missing #root element");
}

ReactDOM.createRoot(root).render(
  <React.StrictMode>
    <ReferenceApp />
  </React.StrictMode>
);
