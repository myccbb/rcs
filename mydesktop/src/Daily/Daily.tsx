import {invoke} from "@tauri-apps/api/tauri";
import {useEffect} from "react";
import {NavLink} from "react-router-dom";

function Daily() {
    // function insert() {
    //     invoke("test_insert")
    //         .then((result) => console.log(result))
    //         .catch((err) => console.log(err));
    // }
    // useEffect(() => {
    //     insert();
    // }, []);
    return (
        <>
            <button>simple note</button>
            <p>hello note</p>
        </>
    )
}


export default Daily;