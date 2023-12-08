import React from "react";
import ReactDOM from "react-dom/client";
// import App from "./App";
// import "./styles.css";

import {BrowserRouter, Route, Routes} from "react-router-dom";

import Daily from "./Daily/Daily";
import Launcher from "./Launcher";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
        {/*<App />*/}
        <BrowserRouter>
            <Routes>
                <Route path="/" element={<Launcher/>}/>
                <Route path="/daily" element={<Daily/>}/>
            </Routes>
        </BrowserRouter>
    </React.StrictMode>
);
