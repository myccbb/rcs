import { ListData } from './center';
import * as center from './center';

interface ObjectTypeItem  {
    id: string,
    name: string,
}

async function list_all(): Promise<ListData<ObjectTypeItem>> {
    console.log('async list all object type');
    let url = '//' + window.location.hostname + '/center-server/api/v1/objtype/list'
    const response = await fetch(url);
    if (!response.ok) {
        throw new center.HttpError(response.status, response.statusText);
    }
    let raw_data = await response.json();
    let data = center.handle_response(raw_data) as ListData<ObjectTypeItem>;
    if (data === null || typeof data !== "object") {
        throw new Error("invalid data");
    }
    return data
}

async function create(id='', name='', category='') {
    console.log('async create object type');
    let url = '//' + window.location.hostname + '/center-server/api/v1/objtype'
    const response = await fetch(url, {
        method: 'POST',
        cache: 'no-cache',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({
            id: id,
            name: name,
            category: category,
        }),
    });
    let raw_data = await response.json();
    let data = center.handle_response(raw_data);
    return data
}

export type {
    ObjectTypeItem,
}

export {
    list_all,
    create,
}
