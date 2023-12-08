import {PieceTypeID as PiecePieceType} from "./piece";


interface CollectorFolder {
    id: string,
    parent_id: string,
    child_list: CollectorFolderChildItem[],
    description: string,
}

interface CollectorFolderChildItem {
    id: string,
    piece_type_id: PiecePieceType,
}

interface CollectorCollection {
    id: string,
    title: string,
    description: string
    child_list: CollectorFolderChildItem[],
}
