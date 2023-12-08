import { ListData } from "./center";
import * as center from "./center";
import { CollectionItem } from "./collection";

interface PieceTypeItem {
    internal_id: number;
    id: string;
    name: string;
    category: string;
}

async function listAll(
    name_like: string | undefined = undefined,
    page_index = 1,
    page_size = 10
): Promise<ListData<PieceTypeItem>> {
    const url = "/center-server/api/v1/piece-type/list";
    const response = await center.request("GET", url, {
        name_like: name_like ?? "",
        page_index: page_index.toString(),
        page_size: page_size.toString(),
    });
    return response as ListData<PieceTypeItem>;
}

async function getById(id: string): Promise<ListData<PieceTypeItem>> {
    let url = "/center-server/api/v1/piece-type/detail";
    const response = await center.request("GET", url, {
        id: id,
    });
    return response as ListData<PieceTypeItem>;
}

async function create(
    id: string,
    name: string,
    category: string,
    description: string
) {
    console.log("async create piece type");
    let url =
        "//" +
        window.location.hostname +
        "/center-server/api/v1/object-type/new";
    const response = await fetch(url, {
        method: "POST",
        cache: "no-cache",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({
            id: id,
            name: name,
            category: category,
            description: description,
        }),
    });
    if (!response.ok) {
        throw new center.HttpError(response.status, response.statusText);
    }
    let rawData = await response.json();
    return center.handleResponse(rawData);
}

async function update(
    id: string,
    name: string,
    description: string
) {
    console.log("async update piece type");
    let url =
        "//" +
        window.location.hostname +
        "/center-server/api/v1/collection/update";
    const response = await fetch(url, {
        method: "PUT",
        cache: "no-cache",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({
            id: id,
            name: name,
            description: description,
        }),
    });
    let rawData = await response.json();
    return center.handleResponse(rawData);
}

export type { PieceTypeItem };

export { listAll, create, update };
