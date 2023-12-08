import * as react from 'react';


function Divider({className, type}: {
    className?: string,
    type: 'vertical' | 'horizontal',
}) {
    if (type === 'vertical') {
        return <div
            style={{borderWidth: "0.01rem"}}
            className={'w-0 border-solid border-gray-200 ' + ' ' + className ?? ''}
        ></div>
    }
    return <hr></hr>
}

export {
    Divider,
}