import React from 'react';
import ReactDOM from 'react-dom/client';

import './index.css';
import reportWebVitals from './reportWebVitals';

import { BrowserRouter, Routes, Route } from "react-router-dom";

// import App from './App';

import Label from './center/Admin/Label/Label';
import Center from './center/Center';
import ObjectType from './center/Admin/ObjectType/ObjectType';

import SimpleNote from "./center/SimpleNote/SimpleNote";
import Admin from './center/Admin/Admin';


const root = ReactDOM.createRoot(
    document.getElementById('root') as HTMLElement
);

root.render(
    <React.StrictMode>
        <BrowserRouter basename='center'>
            <Routes>
                <Route path="/" element={<Center />} />
                <Route path="/simplenote" element={<SimpleNote />} />
                <Route path="/admin" element={<Admin />} >
                    <Route path="label" element={<Label />} />
                    <Route path="objtype" element={<ObjectType />} />
                </Route>
            </Routes>
        </BrowserRouter>
    </React.StrictMode>
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
