'use client';

import type {AppType} from 'next/app';

import {trpc} from "@/utils/trpc";


const ClientDaily: AppType = () => {
    const hello = trpc.hello.useQuery({text: 'daily'});
    if (!hello.data) {
        return (
            <div>loading...</div>
        )
    }
    return (
        <>
            <h1>{hello.data.greeting}</h1>
        </>
    )
}


export default trpc.withTRPC(ClientDaily);
