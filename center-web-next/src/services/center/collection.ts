import {ListData} from './center';
import * as center from './center';

interface CollectionItem {
    internal_id: number,
    id: string,
    piece_type_id: string,
    title: string,
    content: any,
}

async function listAll(): Promise<ListData<CollectionItem>> {
    console.log('async list all collection');
    let url = '//' + window.location.hostname + '/center-server/api/v1/collection/list'
    const response = await fetch(url);
    if (!response.ok) {
        throw new center.HttpError(response.status, response.statusText);
    }
    let rawData = await response.json();
    let data = center.handleResponse(rawData) as ListData<CollectionItem>;
    if (data === null || typeof data !== "object") {
        throw new Error("invalid data");
    }
    return data
}

async function getById(id: string): Promise<CollectionItem> {
    let url = '//' + window.location.hostname + '/center-server/api/v1/collection/detail';
    const response = await fetch(url, {
        method: 'POST',
        cache: 'no-cache',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({
            id: id,
        })
    });
    if (!response.ok) {
        throw new center.HttpError(response.status, response.statusText);
    }
    let rawData = await response.json();
    let data = center.handleResponse(rawData) as CollectionItem;
    if (data === null || typeof data !== "object") {
        throw new Error("invalid data");
    }
    return data
}


async function create(id: string, title: string, piece_type_id: string, content: any) {
    console.log('async create collection');
    let url = '//' + window.location.hostname + '/center-server/api/v1/collection/new'
    const response = await fetch(url, {
        method: 'POST',
        cache: 'no-cache',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({
            id: id,
            title: title,
            piece_type_id: piece_type_id,
            content: content,
        }),
    });
    if (!response.ok) {
        throw new center.HttpError(response.status, response.statusText);
    }
    let rawData = await response.json();
    return center.handleResponse(rawData);
}

async function updateByInternalId(
    internal_id: number, id: string, title: string, piece_type_id: string, content: any,
) {
    console.log('async update collection, ' + JSON.stringify(content));
    let url = '//' + window.location.hostname + '/center-server/api/v1/collection/update'
    const response = await fetch(url, {
        method: 'PUT',
        cache: 'no-cache',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({
            internal_id: internal_id,
            id: id,
            title: title,
            piece_type_id: piece_type_id,
            content: content,
        }),
    });
    let rawData = await response.json();
    return center.handleResponse(rawData);
}

async function update(id: string, title: string, piece_type_id: string, content: any) {
    console.log('async update collection, ' + JSON.stringify(content));
    let url = '//' + window.location.hostname + '/center-server/api/v1/collection/update'
    const response = await fetch(url, {
        method: 'PUT',
        cache: 'no-cache',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({
            id: id,
            title: title,
            piece_type_id: piece_type_id,
            content: content,
        }),
    });
    let rawData = await response.json();
    return center.handleResponse(rawData);
}

export type {
    CollectionItem,
}

export {
    listAll,
    getById,
    create,
    update,
    updateByInternalId,
}
