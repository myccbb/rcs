import { Metadata } from 'next'

import ClientDaily from './client_daily';

export const metadata: Metadata = {
    title: 'Daily',
}

export default function Daily() {

    return (
        <>
            <ClientDaily></ClientDaily>
        </>
    )
}



