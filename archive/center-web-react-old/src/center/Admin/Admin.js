import { NavLink, Outlet } from "react-router-dom";

import './Admin.css';



function Admin() {
    document.title = 'Center Admin';
    return (
        <>
            <div className="admin-layout">
                <div className="sider-container">
                    <aside className="menu">
                        <ul className="menu-list">
                            <li><NavLink to="objtype">object type</NavLink></li>
                            <li><NavLink to="label">label</NavLink></li>
                        </ul>
                    </aside>
                </div>
                <div className="content-container">
                    <Outlet />
                </div>
            </div>
        </>
    );
}

export default Admin;
