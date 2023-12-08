import {PieceTypeID as PiecePieceType} from "./piece";

// TODO implement bookmark type definition and service
interface Bookmark {
    id: string,
    folder_id: string,
    title: string,
    url: string,
    description: string,
}

interface BookmarkPieceConfig {
    title: string,
    url: string,
    description: string,
}

async function listByFolderId(folder_id: string): Promise<Bookmark[]> {
    return [];
}

export type {
    Bookmark,
}