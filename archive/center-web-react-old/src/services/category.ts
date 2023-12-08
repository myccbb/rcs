import * as center from './center';
import { ListData } from './center';

interface CategoryItem {
    name: string,
}

async function list_all(): Promise<ListData<CategoryItem>> {
    console.log('async list all category');
    let url = '//' + window.location.hostname + '/center-server/api/v1/category/list'
    const response = await fetch(url);
    if (!response.ok) {
        throw new center.HttpError(response.status, response.statusText);
    }
    let raw_data = await response.json();
    let data = center.handle_response(raw_data) as ListData<CategoryItem>;
    if (data === null || typeof data !== "object") {
        throw new Error("invalid data");
    }
    return data
}

export type {
    CategoryItem,
}

export {
    list_all,
}
