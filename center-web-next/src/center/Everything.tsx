import * as react from 'react';

function Everything() {
    function handleKeyDown(e: react.KeyboardEvent<HTMLDivElement>) {
        if (e.ctrlKey && e.key == 'n') {
            console.log('ctrl-n pressed');
            e.preventDefault();
        }
    }

    return (
        <>
            <div className={'bg-gray-100'}
                 onKeyDown={handleKeyDown}
                 tabIndex={0}
            >
                <h1 onKeyDown={handleKeyDown}>Everything</h1>
            </div>
        </>
    );
}

export default Everything;