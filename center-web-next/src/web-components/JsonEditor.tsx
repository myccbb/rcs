import * as react from 'react';
import {useEffect} from "react";

import {Content, JSONEditor, Mode, OnChange, OnChangeStatus} from 'vanilla-jsoneditor';

import './JsonEditor.css';

function JsonEditor({className, onChange, content, readOnly}: {
    className: string,
    onChange?: any,
    content?: Content,
    readOnly?: boolean,
}) {

    const containerRef = react.useRef<HTMLDivElement>(null);
    let editorRef = react.useRef<JSONEditor>();

    useEffect(() => {
        if (containerRef.current) {
            editorRef.current = new JSONEditor({
                target: containerRef.current,
                props: {},
            });
        }
        return () => {
            if (editorRef.current) {
                editorRef.current.destroy();
                editorRef.current = undefined;
            }
        }
    }, []);
    useEffect(() => {
        if (editorRef.current) {
            editorRef.current?.updateProps({
                mode: Mode.text,
                content: content ?? {json: {}},
                onChange: onChange,
                readOnly: readOnly ?? false,
            });
        }
    }, [content, onChange, readOnly]);

    return (
        <>
            <div id={'jsoneditor'} ref={containerRef} className={className}></div>
        </>
    )
}

export {
    JsonEditor,
}