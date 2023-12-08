import React from 'react'
import ReactDOM from 'react-dom/client'


import { BrowserRouter, Routes, Route } from 'react-router-dom';

import './main.css';


import Center from './center/Center';
import Admin from './center/Admin/Admin';
import ListPieceType from './center/Admin/PieceType/ListPieceType';
import Label from './center/Admin/Label/Label';
import Collection from './center/Admin/Collection';
import PieceDetail from './center/Admin/Piece';

import Todo from './center/Todo/Todo';

import SimpleNote from './center/SimpleNote/SimpleNote';
import Collector from "./center/Collector/Collector";
import Daily from "./center/Daily/Daily";
import Everything from "@/src/center/Everything";


ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
    <React.StrictMode>
        <BrowserRouter basename='center'>
            <Routes>
                <Route path='/' element={<Center />}></Route>
                <Route path='/simple-note' element={<SimpleNote />}></Route>
                <Route path='/daily' element={<Daily />}></Route>
                <Route path='/admin' element={<Admin />}>
                    <Route path='label' element={<Label />}></Route>
                    <Route path='piece-type' element={<ListPieceType />}></Route>
                    <Route path='collection' element={<Collection />}></Route>
                    <Route path='piece' element={<PieceDetail />}></Route>
                    <Route path='piece/detail' element={<PieceDetail />}></Route>
                </Route>
                <Route path='/collector' element={<Collector />}>
                </Route>
                <Route path='/todo' element={<Todo />}>
                </Route>
                <Route path='/everything' element={<Everything />}>
                </Route>
            </Routes>
        </BrowserRouter>
    </React.StrictMode>,
)
