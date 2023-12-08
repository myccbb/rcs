import dayjs, { Dayjs } from 'dayjs';

enum IdPrefix {
    PIECE = "PIECE",
    COLLECTION = "COLL",
    OBJECT_TYPE = "TYPE",
}

function piece_id(): string {
    return random_id(IdPrefix.PIECE, dayjs(), 6);
}
function collection_id(): string {
    return random_id(IdPrefix.COLLECTION, dayjs(), 6);
}
function objtype_id(): string {
    return random_id(IdPrefix.OBJECT_TYPE, dayjs(), 6);
}

function random_id(prefix: IdPrefix, now: Dayjs, random_length: number) {
    return prefix + '-' + now.format('YYMMDD-HHMMSS-') + random_string(6);
}

const LETTER_TABLE = "ABCDEFGHIJKLMNOPQRSTUVWXTZ";
function random_string(length: number): string {
    let result = "";
    for (let i = 0; i < length; ++i) {
        result += LETTER_TABLE[Math.floor(Math.random() * LETTER_TABLE.length)];
    }
    return result;
}

export {
    IdPrefix,
    piece_id,
    collection_id,
    objtype_id,
}