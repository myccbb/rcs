import {useNavigate} from "react-router-dom";
import {getMatches} from "@tauri-apps/api/cli";

function Launcher() {
    let navigate = useNavigate();
    getMatches().then((result) => {
        switch (result.args.app.value) {
            case "daily":
                navigate("/daily");
                break;
        }
    }).catch((err) => {
        console.log(err);
    });

    return (
        <>
            <div>
                <button>launch</button>
            </div>
            <p>hello launcher</p>
        </>
    )
}


export default Launcher;