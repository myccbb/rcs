import * as react from 'react';

function Button({children, onClick, className}: {
    children: react.ReactNode,
    onClick?: () => void,
    className?: string,
}) {
    className = className ?? '';
    return (
        <>
            <button className={'h-10 px-3 border rounded hover:bg-gray-100 active:bg-gray-200 ' + className}
                    onClick={onClick}>
                {children}
            </button>
        </>
    );
}

function InputButton({children, onClick, className, type, value}: {
    children?: react.ReactNode,
    onClick?: () => void,
    className?: string,
    type?: string,
    value: string,
}) {
    className = className ?? '';
    type = type ?? 'button';
    return (
        <>
            <input type={type} value={value}
                   className={'h-10 px-3 border rounded hover:bg-gray-100 active:bg-gray-200 ' + className}
                   onClick={onClick}>
                {children}
            </input>
        </>
    );
}

export {
    Button,
    InputButton,
};