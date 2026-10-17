import React from "react";
import ReactDOM from "react-dom/client";
import { AlarmWindow } from "./AlarmWindow";
import "../styles.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <AlarmWindow />
  </React.StrictMode>,
);
