import { useEffect, useState } from 'react';
import * as css from 'csstype';
import * as react from 'react';

import * as pieceType from '../../../services/center/piece_type';

import CreatePieceType from './CreatePieceType';
import { Button } from "@/src/web-components/Button";
import EditPieceType from "./EditPieceType";


function ListPieceType() {
    const [pieceTypeList, setPieceTypeList] = react.useState<pieceType.PieceTypeItem[]>([]);

    function listPieceType() {
        console.log('current host: %s', window.location.hostname);
        pieceType.listAll().then(function (response) {
            console.log('response is ' + JSON.stringify(response))
            if (response.results !== null) {
                setPieceTypeList(response.results)
            }
        }).catch(function (error) {
            console.log('error is ' + JSON.stringify(error))
        })
    }

    useEffect(() => {
        listPieceType();
    }, [])


    const [showHideCreateForm, setShowHideCreateForm] = useState<boolean>(false)


    function toggleCreatePieceTypeForm() {
        if (showHideCreateForm) {
            setShowHideCreateForm(false);
        } else {
            setShowHideCreateForm(true);
        }
        listPieceType();
    }

    const opIdPrefix = 'th-op-';
    const editIdPrefix = 'th-edit-';

    function printEvent(e: any) {
        // console.log(e.target.id);
        // console.log(e.target.cellIndex);
        // console.log(e.target.closest('tr').rowIndex);
        // console.log(document.getElementById(e.target.id)?.textContent);
        // let id = (e.target.id as String).replace(opIdPrefix, '');
        // console.log(id);
    }


    const [showHideEditForm, setShowHideEditForm] = useState<boolean>(false)

    function toggleEditPieceTypeForm() {
        if (showHideEditForm) {
            console.log('set showHide to false');
            setShowHideEditForm(false);
        } else {
            console.log('set showHide to true');
            setShowHideEditForm(true);
        }
        listPieceType();
    }

    const [editPieceTypeId, setEditPieceTypeId] = useState<String>("");

    function editPieceType(e: any) {
        let id = (e.target.id as String).replace(editIdPrefix, '');
        console.log(id);
        setEditPieceTypeId(id);
        setShowHideEditForm(true);
    }


    return (
        <>
            <div style={styles.pieceTypeLayout}>
                <div style={styles.searchConditionContainer}>
                    <Button onClick={toggleCreatePieceTypeForm}>new</Button>
                    <div className='px-3' />
                    <Button onClick={listPieceType}>search</Button>
                </div>
                <div className="p-3"></div>
                <div style={styles.searchResultContainer}>
                    <table className={'table-auto w-full'}>
                        <thead className={'h-10 text-left border-b'}>
                            <tr>
                                <th>id</th>
                                <th>name</th>
                                <th>desc</th>
                                <th>operation</th>
                            </tr>
                        </thead>
                        <tbody className={'border-t'} onClick={printEvent}>
                            {pieceTypeList.map((item) => {
                                return <tr className={'h-10'} id={"tr-" + item.id} key={item.id}>
                                    <td>{item.id}</td>
                                    <td>{item.name}</td>
                                    <td>desc</td>
                                    <td id={opIdPrefix + item.id}>
                                        <p id={editIdPrefix + item.id}
                                            onClick={editPieceType}
                                            className={'text-blue-500'}>edit</p>
                                    </td>
                                </tr>
                            })}
                        </tbody>
                        {showHideEditForm &&
                            <EditPieceType
                                toggleClose={toggleEditPieceTypeForm} />}
                    </table>
                </div>
                {showHideCreateForm &&
                    <CreatePieceType
                        toggleClose={toggleCreatePieceTypeForm} />}
            </div>
        </>
    );
}


const styles = {
    pieceTypeLayout: {
        display: 'flex',
        flexDirection: 'column',
        width: '100%',
        height: '100%',
        padding: '10px',
    } as css.Properties,

    searchConditionContainer: {
        display: 'flex',
        flexDirection: 'row',
    } as css.Properties,

    searchResultContainer: {
        width: '100%',
        height: '100%',
    } as css.Properties,

}


export default ListPieceType;
