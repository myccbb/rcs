import * as css from "csstype";
import * as react from "react";
import * as utils from "../../../utils";
import * as pieceType from "../../../services/center/piece_type";

import { useRef, useState } from "react";
import { NotificationContext } from "../../../web-components/Notification";
import { InputButton } from "../../../web-components/Button";

function EditPieceType({
    toggleClose,
}: {
    toggleClose: any;
}) {
    let notice = react.useContext(NotificationContext);
    let nameRef = useRef<HTMLInputElement>(null);
    let idRef = useRef<HTMLInputElement>(null);
    let descriptionRef = useRef<HTMLSelectElement>(null);

    function updatePieceType() {
        let id = idRef.current?.value ?? "";
        let name = nameRef.current?.value ?? "";
        let description = "";
        console.log("update piece ", id, name);
        pieceType
            .update(id, name, description)
            .then(function (_) {
                notice.info("success");
            })
            .catch(function (error) {
                notice.info(error.message);
            });
    }

    function closeOnESC(event: any) {
        if (event.code === "Escape") {
            toggleClose();
        }
    }

    function preventRefresh(event: any) {
        event.preventDefault();
    }

    return (
        <>
            <div style={styles.fade} id={ids.fade} onKeyDown={closeOnESC}>
                <div style={{ ...styles.form, ...utils.styles.globalCenter }}>
                    <form onSubmit={preventRefresh}>
                        <div style={styles.formItem}>
                            <label
                                className={"h-10 flex items-center"}
                                htmlFor="category"
                            >
                                Category
                            </label>
                        </div>
                        <div style={styles.formItem}>
                            <label
                                className={"h-10 flex items-center"}
                                htmlFor="id"
                            >
                                ID
                            </label>
                            <input
                                className={"border h-8"}
                                id={ids.idText}
                                ref={idRef}
                                type="text"
                            ></input>
                        </div>
                        <div style={styles.formItem}>
                            <label
                                className={"h-10 flex items-center"}
                                htmlFor="name"
                            >
                                Name
                            </label>
                            <input
                                className={"border h-8"}
                                id={ids.nameText}
                                ref={nameRef}
                                type="text"
                            ></input>
                        </div>
                        <div className={"py-2"}></div>
                        <div style={styles.formItem}>
                            <div className={"flex flex-row"}>
                                <InputButton
                                    type="submit"
                                    className="button"
                                    value="Submit"
                                    onClick={updatePieceType}
                                ></InputButton>
                                <div className={"px-3"}></div>
                                <InputButton
                                    type="reset"
                                    className="button"
                                    value="Cancel"
                                    onClick={toggleClose}
                                ></InputButton>
                            </div>
                        </div>
                    </form>
                </div>
            </div>
        </>
    );
}

const ids = {
    fade: "fade",
    category: "category",
    nameText: "name-text",
    idText: "id-text",
};

const styles = {
    fade: {
        display: "block",
        position: "fixed",
        width: "100%",
        height: "100%",
        top: "0%",
        left: "0%",
        backgroundColor: "rgba(0, 0, 0, 0.5)",
        zIndex: "8",
    } as css.Properties,

    form: {
        width: "50%",
        minWidth: "500px",
        height: "80%",
        //position: 'fixed',
        //top: '50%',
        //left: '50%',
        //transform: 'translate(-50%, -50%)',
        zIndex: "9",
        border: "3px solid #f1f1f1",
        borderRadius: "3px",
        backgroundColor: "white",
    } as css.Properties,

    formLabel: {
        display: "block",
        width: "20%",
        minWidth: "800px",
        textAlign: "left",
    } as css.Properties,

    formItem: {
        display: "block",
        width: "80%",
        minWidth: "400px",
        margin: "auto",
    } as css.Properties,
};

export default EditPieceType;
