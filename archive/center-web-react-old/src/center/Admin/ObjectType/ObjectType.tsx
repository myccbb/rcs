import { useState } from 'react';
import * as css from 'csstype';

import * as objtype from '../../../services/objectType';
import * as category from '../../../services/category';

import CreateObjectType from './CreateObjectType';
import ListObjectType from './ListObjectType';



function ObjectType() {
    const [objtypeList, setObjtypeList] = useState<objtype.ObjectTypeItem[]>([]);
    function listObjectType() {
        console.log('current host: %s', window.location.hostname);
        objtype.list_all().then(function (response) {
            console.log('response is ' + JSON.stringify(response))
            setObjtypeList(response.results)
        }).catch(function (error) {
            console.log('error is ' + JSON.stringify(error))
        })
    }
    const [categoryList, setCategoryList] = useState<category.CategoryItem[]>([])
    function listCategory() {
        category.list_all().then(function (response) {
            console.log('response is ' + JSON.stringify(response));
            setCategoryList(response.results)
        }).catch(function (error) {
            console.log('error is ' + JSON.stringify(error))
        })
    }
    const [showHide, setShowHide] = useState<boolean>(false)
    function toggleCreateObjtypeForm() {
        if (showHide === true) {
            console.log('set showHide to false');
            setShowHide(false);
        } else {
            console.log('set showHide to true');
            listCategory();
            setShowHide(true);
        }
    }
    return (
        <>
            <div style={styles.objtype_layout}>
                <div style={styles.search_condition_container}>
                    <div className='px-3'>
                        <button className='button' onClick={toggleCreateObjtypeForm}>new</button>
                    </div>
                    <div className='px-3'>
                        <button className='button' onClick={listObjectType}>search</button>
                    </div>
                </div>
                <div className="p-3"></div>
                <div style={styles.search_result_container}>
                    <div className='search-result'>
                        <ListObjectType objtypeList={objtypeList}></ListObjectType>
                    </div>
                    {showHide && <CreateObjectType categoryList={categoryList} toggleClose={toggleCreateObjtypeForm} />}
                </div>
            </div>
        </>
    );
}



const styles = {
    objtype_layout: {
        display: 'flex',
        flexDirection: 'column',
        width: '100%',
        height: '100%',
    } as css.Properties,

    search_condition_container: {
        display: 'flex',
        flexDirection: 'row',
    } as css.Properties,

    search_result_container: {
        width: '100%',
        height: '100%',
        display: 'flex',
        flexDirection: 'column',
    } as css.Properties,

    search_result: {
        width: '100px',
        height: '100px',
        background: 'yellowgreen',
    } as css.Properties,

}



export default ObjectType;
