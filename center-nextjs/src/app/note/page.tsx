import { PrismaClient } from "@prisma/client";

import NoteCommand from "./note_command";

import NoteList from "./note_list";
import NoteDetail from './note_detail';
import Separator from '@/ui_components/separator';

const prisma = new PrismaClient();

async function list_piece() {
    const pieces = await prisma.piece.findMany();
    console.log(pieces);
    for (let piece in pieces) {
        console.log(piece);
    }
}

const Page = () => {
    // list_piece();
    return (
        <>
            <div className='flex flex-col h-screen p-1'>
                <NoteCommand />
                <Separator orientation="horizontal" />
                <div className='flex flex-row h-full'>
                    <NoteList />
                    <Separator orientation="vertical" />
                    <NoteDetail />
                </div>
            </div>
        </>
    )
}

export default Page;