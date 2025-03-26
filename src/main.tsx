import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { AntdProvider } from "~/provider/antd";
import { BrowserRouter } from "react-router-dom";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <BrowserRouter>
      <AntdProvider>
        <App />
      </AntdProvider>
    </BrowserRouter>
  </React.StrictMode>,
);
