import * as center from './center';
import { ListData } from './center';

interface CategoryItem {
    name: string,
}

async function listAll(): Promise<ListData<CategoryItem>> {
    console.log('async list all category');
    let url = '//' + window.location.hostname + '/center-server/api/v1/category/list'
    const response = await fetch(url);
    if (!response.ok) {
        throw new center.HttpError(response.status, response.statusText);
    }
    let rawData = await response.json();
    let data = center.handleResponse(rawData) as ListData<CategoryItem>;
    if (data === null || typeof data !== "object") {
        throw new Error("invalid data");
    }
    return data
}

export type {
    CategoryItem,
}

export {
    listAll,
}
