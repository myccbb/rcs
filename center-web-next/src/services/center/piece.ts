import * as center from './center';
import {ListData} from "./center";

enum PieceTypeID {
    CollectorFolder = "collector_folder",
    Bookmark = "bookmark",
}

interface Piece {
    internal_id: number,
    id: string,
    parent_id: string,
    piece_type_id: PieceTypeID,
    title: string,
    content: object,
    create_time: Date,
    update_time: Date,
}


async function listAll(): Promise<ListData<Piece>> {
    console.log('async list all piece');
    let url = '/center-server/api/v1/piece/list';
    const response = await center.request('GET', url, null);
    return response as ListData<Piece>;
}

async function getById(id: string): Promise<Piece> {
    let url = '/center-server/api/v1/piece/detail';
    const response = await center.request('POST', url, {id: id});
    return response as Piece;
}


async function create(id: string, title: string, piece_type_id: string, content: any) {
    console.log('async create piece');
    let url = '/center-server/api/v1/piece/new'
    return await center.request('POST', url, {
        id: id,
        title: title,
        piece_type_id: piece_type_id,
        content: content,
    })
}

async function updateByInternalId(internalId: number, id: string, title: string, piece_type_id: string, content: any) {
    console.log('async update piece, ' + JSON.stringify(content));
    let url = '/center-server/api/v1/piece/update';
    return await center.request('PUT', url, {
        internal_id: internalId,
        id: id,
        title: title,
        piece_type_id: piece_type_id,
        content: content,
    })
}

async function update(id: string, title: string, piece_type_id: string, content: any) {
    console.log('async update piece, ' + JSON.stringify(content));
    let url = '/center-server/api/v1/piece/update';
    return await center.request('PUT', url, {
        id: id,
        title: title,
        piece_type_id: piece_type_id,
        content: content,
    })
}

export type {
    Piece,
}

export {
    PieceTypeID,
    getById,
    listAll,
    create,
    update,
    updateByInternalId,
}