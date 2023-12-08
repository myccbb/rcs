
import * as css from 'csstype';
import * as utils from '../../../utils';
import * as objtype from '../../../services/objectType';
import { CategoryItem } from '../../../services/category';


import { useRef, useState } from 'react';

function CreateObjectType({ categoryList, toggleClose }: {
    categoryList: CategoryItem[],
    toggleClose: any,
}) {
    let nameref = useRef<HTMLInputElement>();
    let idref = useRef<HTMLInputElement>();
    const categoryref = useRef<HTMLSelectElement>();
    const [createObjtypeRes, setCreateObjtypeRes] = useState("");
    function createObjtype() {
        let id = idref.current.value;
        let name = nameref.current.value;
        let category = categoryref.current.value;
        console.log('create object ', id, name, category);
        objtype.create(id, name, category).catch(function (error) {
            setCreateObjtypeRes(error.message);
            setTimeout(() => { setCreateObjtypeRes("") }, 3000);
        });
    }

    function closeOnESC(event: any) {
        if (event.code === 'Escape') {
            console.log('into close on esc');
            toggleClose();
        }
    }

    function preventRefresh(event: any) {
        event.preventDefault();
    }

    return (
        <>
            <div style={styles.fade} id={ids.fade} onKeyDown={closeOnESC}>
                <div style={{ ...styles.form, ...utils.styles.global_center }}>
                    <form onSubmit={preventRefresh}>
                        <div style={styles.form_item}>
                            <label style={styles.form_label} htmlFor="category">Category</label>
                            <div className="select">
                                <select id={ids.category} ref={categoryref}>
                                    {categoryList.map(item => {
                                        return <option key={item.name} value={item.name} >{item.name}</option>
                                    })}
                                </select>
                            </div>
                        </div>
                        <div style={styles.form_item}>
                            <label style={styles.form_label} htmlFor="name">Name</label>
                            {/* <input id={ids.create_objtype_form_name_text} type="text" onChange={detectName}></input> */}
                            <input id={ids.name_text} ref={nameref} type="text" ></input>
                        </div>
                        <div style={styles.form_item}>
                            <label style={styles.form_label} htmlFor="id">ID</label>
                            <input id={ids.id_text} ref={idref} type="text" ></input>
                        </div>
                        <div style={styles.form_item}>
                            <input type="submit" className='button' value='Submit'
                                onClick={createObjtype} ></input>
                            <input type="reset" className='button' value='Cancel'
                                onClick={toggleClose}></input>
                        </div>
                        <div style={styles.form_item}>
                            <p>{createObjtypeRes}</p>
                        </div>
                    </form>
                </div>
            </div>

        </>
    );
}

const ids = {
    fade: 'fade',
    category: 'category',
    name_text: 'name-text',
    id_text: 'id-text',
}



const styles = {
    fade: {
        display: 'block',
        position: 'fixed',
        width: '100%',
        height: '100%',
        top: '0%',
        left: '0%',
        backgroundColor: 'rgba(0, 0, 0, 0.5)',
        zIndex: '8',
    } as css.Properties,

    form: {
        width: '50%',
        minWidth: '500px',
        height: '80%',
        //position: 'fixed',
        //top: '50%',
        //left: '50%',
        //transform: 'translate(-50%, -50%)',
        zIndex: '9',
        border: '3px solid #f1f1f1',
        borderRadius: '3px',
        backgroundColor: 'white',
    } as css.Properties,

    form_label: {
        display: 'block',
        width: '20%',
        minWidth: '800px',
        textAlign: 'left',
    } as css.Properties,

    form_item: {
        display: 'block',
        width: '80%',
        minWidth: '400px',
        margin: 'auto',
    } as css.Properties,

}

export default CreateObjectType;
