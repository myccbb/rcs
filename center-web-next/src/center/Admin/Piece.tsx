import * as react from 'react';
import {useEffect, useRef, useState} from 'react';

import * as css from 'csstype';

import {Button} from '../../web-components/Button';

import * as pieceType from '../../services/center/piece_type';
import * as piece from '../../services/center/piece';

import {NotificationContext} from '../../web-components/Notification';
import {PieceTypeItem} from "../../services/center/piece_type";

import {JsonEditor} from "../../web-components/JsonEditor";


function PieceDetail() {
    let notice = react.useContext(NotificationContext);

    let pieceIdRef = useRef<HTMLInputElement | null>(null);
    let pieceTitleRef = useRef<HTMLInputElement | null>(null);
    let piecePieceTypeIdRef = useRef<HTMLSelectElement | null>(null);
    let pieceInternalId = useRef<number | null>(null);

    const [pieceConfigContent, setPieceConfigContent] = useState<any>();
    const [pieceTypeList, setPieceTypeList] = useState<PieceTypeItem[]>([]);

    function listPiecePieceType() {
        pieceType.listAll('', 'piece').then(function (response) {
            setPieceTypeList(response.results ?? []);
        }).catch(function (error) {
            notice.info(error.message);
        })
    }

    function getPieceById() {
        let pieceId = pieceIdRef.current?.value ?? '';
        if (pieceId === '') {
            notice.info('Piece ID is empty');
            return;
        }
        piece.getById(pieceId).then(function (response) {
            console.log(response);
            if (pieceTitleRef.current) {
                pieceTitleRef.current.value = response.title;
            }
            if (piecePieceTypeIdRef.current) {
                piecePieceTypeIdRef.current.id = response.piece_type_id;
            }
            setPieceConfigContent({json: response.content})
            pieceInternalId.current = response.internal_id;
            console.log('piece internal id is ' + response.internal_id);
        }).catch(function (error) {
            console.log(error);
            notice.info(error.msg);
        })
    }

    function createPiece() {
        let pieceId = pieceIdRef.current?.value ?? '';
        let pieceTitle = pieceTitleRef.current?.value ?? '';
        let piecePieceTypeId = piecePieceTypeIdRef.current?.value ?? '';
        if (pieceId === '') {
            notice.info('Piece ID is empty');
            return;
        }
        let pieceConfig = {};
        if (pieceConfigContent != null) {
            pieceConfig = pieceConfigContent.json
            if (pieceConfig == null && pieceConfigContent.text != null) {
                pieceConfig = JSON.parse(pieceConfigContent.text ?? '{}');
            }
        }

        piece.create(
            pieceId,
            pieceTitle,
            piecePieceTypeId,
            pieceConfig,
        ).then(function (response) {
            notice.info('Piece created');
        }).catch(function (error) {
            notice.info(error.msg);
        });
    }

    function updatePieceById() {
        let pieceId = pieceIdRef.current?.value ?? '';
        let pieceTitle = pieceTitleRef.current?.value ?? '';
        let piecePieceTypeId = piecePieceTypeIdRef.current?.value ?? '';
        if (pieceId === '') {
            notice.info('Piece ID is empty');
            return;
        }
        console.log(pieceInternalId);
        if (pieceInternalId.current == null) {
            notice.info('piece not found');
            return;
        }
        let pieceConfig = pieceConfigContent.json;
        if (pieceConfigContent.json === undefined) {
            pieceConfig = {};
            if (pieceConfigContent.text !== undefined) {
                pieceConfig = JSON.parse(pieceConfigContent.text);
            }
        }
        piece.updateByInternalId(
            pieceInternalId.current,
            pieceId,
            pieceTitle,
            piecePieceTypeId,
            pieceConfig,
        ).then(function (response) {
            notice.info('Piece updated');
        }).catch(function (error) {
            notice.info(error.msg);
        });
    }

    useEffect(() => {
        listPiecePieceType();
    }, [])

    return (
        <>
            <div className={'flex flex-col px-3 w-full h-full divide-y-2 divide-gray-300'}>
                {/*<Table th_list={['1', '2', '3', 'operation']} data_list={[*/}
                {/*    {a: 11, b: 12, c: 13},*/}
                {/*    {a: 21, b: 22, c: 23},*/}
                {/*]} operation={<p>hello</p>}/>*/}
                <div className={'flex flex-row py-3'}>
                    <div style={styles.operationContainer}>
                        <Button onClick={getPieceById}>Get</Button>
                        <div className={'w-3'}></div>
                        <Button onClick={createPiece}>New</Button>
                        <div className={'w-3'}></div>
                        <Button onClick={updatePieceById}>Update</Button>
                        <div className={'w-3'}></div>
                    </div>
                </div>
                <div className={'flex flex-col py-3 h-full'}>
                    <label className={'h-10'}>Piece ID:</label>
                    <input type='text' className={'border border-gray-200 h-10'}
                           ref={pieceIdRef}>
                    </input>

                    <div className={'h-3'}></div>
                    <label className={'h-10'}>Piece Title:</label>
                    <input type='text' className={'border border-gray-200 h-10'}
                           ref={pieceTitleRef}>
                    </input>

                    <div className={'h-3'}></div>
                    <label className={'h-10'}>Piece PieceTypeId:</label>
                    <select className={'border border-gray-200 h-10'} ref={piecePieceTypeIdRef}>
                        {pieceTypeList.map(item => {
                            return <option key={item.id} value={item.name}>{item.name}</option>
                        })}
                    </select>

                    <div className={'h-3'}></div>
                    <label className={'h-10'}>Piece Config:</label>
                    <JsonEditor
                        className={'h-full font-mono'}
                        onChange={setPieceConfigContent}
                        content={pieceConfigContent}/>
                </div>
            </div>
        </>
    );
}


const styles = {
    layout: {
        display: 'flex',
        flexDirection: 'column',
        padding: '10px 30px 10px 10px',
        width: '100%',
        height: '100%',
    } as css.Properties,

    operationContainer: {
        display: 'flex',
        flexDirection: 'row',
    } as css.Properties,

}


export default PieceDetail;
