import * as react from 'react';
import {useEffect, useRef, useState} from 'react';

import * as css from 'csstype';

import {Button} from '../../web-components/Button';

import * as pieceType from '../../services/center/piece_type';
import * as collection from '../../services/center/collection';

import {NotificationContext} from '../../web-components/Notification';
import {PieceTypeItem} from "../../services/center/piece_type";

import {JsonEditor} from "../../web-components/JsonEditor";
import {Content, JSONContent} from "vanilla-jsoneditor";


function Collection() {
    let notice = react.useContext(NotificationContext);

    let collectionIdRef = useRef<HTMLInputElement | null>(null);
    let collectionTitleRef = useRef<HTMLInputElement | null>(null);
    let collectionPieceTypeIdRef = useRef<HTMLSelectElement | null>(null);
    let collectionInternalId = useRef<number | null>(null);

    const [collectionConfigContent, setCollectionConfigContent] = useState<any>();
    const [pieceTypeList, setPieceTypeList] = useState<PieceTypeItem[]>([]);
    function listCollectionPieceType() {
        pieceType.listAll('', 'collection').then(function (response) {
            setPieceTypeList(response.results ?? []);
        }).catch(function (error) {
            notice.info(error.message);
        })
    }

    function getCollectionById() {
        let collectionId = collectionIdRef.current?.value ?? '';
        if (collectionId === '') {
            notice.info('Collection ID is empty');
            return;
        }
        collection.getById(collectionId).then(function (response) {
            if (collectionTitleRef.current) {
                collectionTitleRef.current.value = response.title;
            }
            if (collectionPieceTypeIdRef.current) {
                collectionPieceTypeIdRef.current.id = response.piece_type_id;
            }
            setCollectionConfigContent({json: response.content})
            collectionInternalId.current = response.internal_id;
            console.log('collection internal id is ' + response.internal_id);
        }).catch(function (error) {
            notice.info(error.msg);
        })
    }

    function createCollection() {
        let collectionId = collectionIdRef.current?.value ?? '';
        let collectionTitle = collectionTitleRef.current?.value ?? '';
        let collectionPieceTypeId = collectionPieceTypeIdRef.current?.value ?? '';
        if (collectionId === '') {
            notice.info('Collection ID is empty');
            return;
        }
        let collectionConfig = {};
        if (collectionConfigContent != null) {
            collectionConfig = collectionConfigContent.json
            if (collectionConfig == null && collectionConfigContent.text != null) {
                collectionConfig = JSON.parse(collectionConfigContent.text ?? '{}');
            }
        }
        collection.create(
            collectionId,
            collectionTitle,
            collectionPieceTypeId,
            collectionConfig,
        ).then(function (response) {
            notice.info('Collection created');
        }).catch(function (error) {
            notice.info(error.msg);
        });
    }

    function updateCollectionById() {
        let collectionId = collectionIdRef.current?.value ?? '';
        let collectionTitle = collectionTitleRef.current?.value ?? '';
        let collectionPieceTypeId = collectionPieceTypeIdRef.current?.value ?? '';
        if (collectionId === '') {
            notice.info('Collection ID is empty');
            return;
        }
        console.log(collectionInternalId);
        if (collectionInternalId.current == null) {
            notice.info('collection not found');
            return;
        }
        let collectionConfig = collectionConfigContent.json;
        if (collectionConfigContent.json === undefined) {
            collectionConfig = {};
            if (collectionConfigContent.text !== undefined) {
                collectionConfig = JSON.parse(collectionConfigContent.text);
            }
        }
        collection.updateByInternalId(
            collectionInternalId.current,
            collectionId,
            collectionTitle,
            collectionPieceTypeId,
            collectionConfig,
        ).then(function (response) {
            notice.info('Collection updated');
        }).catch(function (error) {
            notice.info(error.msg);
        });
    }

    useEffect(() => {
        listCollectionPieceType();
    }, [])

    return (
        <>
            <div className={'flex flex-col px-3 w-full h-full divide-y-2 divide-gray-300'}>
                <div className={'flex flex-row py-3'}>
                    <div style={styles.operationContainer}>
                        <Button onClick={getCollectionById}>Get</Button>
                        <div className={'w-3'}></div>
                        <Button onClick={createCollection}>New</Button>
                        <div className={'w-3'}></div>
                        <Button onClick={updateCollectionById}>Update</Button>
                        <div className={'w-3'}></div>
                    </div>
                </div>
                <div className={'flex flex-col py-3 h-full'}>
                    <label className={'h-10'}>Collection ID:</label>
                    <input type='text' className={'border border-gray-200 h-10'}
                           ref={collectionIdRef}>
                    </input>

                    <div className={'h-3'}></div>
                    <label className={'h-10'}>Collection Title:</label>
                    <input type='text' className={'border border-gray-200 h-10'}
                           ref={collectionTitleRef}>
                    </input>

                    <div className={'h-3'}></div>
                    <label className={'h-10'}>Collection PieceTypeId:</label>
                    <select className={'border border-gray-200 h-10'} ref={collectionPieceTypeIdRef}>
                        {pieceTypeList.map(item => {
                            return <option key={item.id} value={item.name}>{item.name}</option>
                        })}
                    </select>

                    <div className={'h-3'}></div>
                    <label className={'h-10'}>Collection Config:</label>
                    <JsonEditor
                        className={'h-full font-mono'}
                        onChange={setCollectionConfigContent}
                        content={collectionConfigContent}/>
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


export default Collection;
