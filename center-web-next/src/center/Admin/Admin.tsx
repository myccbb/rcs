import {NavLink, Outlet} from 'react-router-dom';

import * as css from 'csstype';

import './Admin.css';

import {Notification} from '../../web-components/Notification';


function Admin() {


    document.title = 'Center Admin';
    return (
        <>
            <Notification/>
            <div style={styles.adminLayout}>
                <div style={styles.sidebarContainer}>
                    <aside className='my-menu'>
                        <ul>
                            <li className='my-menu-item'>
                                <NavLink to='piece-type'>piece type</NavLink>
                            </li>
                            <li className='my-menu-item'>
                                <NavLink to='label'>label</NavLink>
                            </li>
                            <li className='my-menu-item'>
                                <NavLink to='collection'>collection</NavLink>
                            </li>
                            <li className='my-menu-item'>
                                <NavLink to='piece'>piece</NavLink>
                            </li>
                        </ul>
                    </aside>
                </div>
                <div style={styles.contentContainer}>
                    <Outlet/>
                </div>
            </div>
        </>
    );
}

const styles = {
    adminLayout: {
        display: 'flex',
        flexDirection: 'row',
        height: '100vh',
        width: '100vw',
    } as css.Properties,

    sidebarContainer: {
        width: '20%',
        height: '100%',
        maxWidth: '260px',
        minWidth: '130px',
        // backgroundColor: 'antiquewhite',
    } as css.Properties,

    contentContainer: {
        // padding: '10px 30px 10px 10px',
        width: '100%',
        height: '100%',
        backgroundColor: 'white',
    } as css.Properties,
}

export default Admin;
