
import * as objtype from '../../../services/objectType';

function ListObjectType({ objtypeList }: {
    objtypeList: objtype.ObjectTypeItem[],
}) {
    return (
        <>
            <div>
                <ul>
                    {objtypeList.map((item) => {
                        return <li key={item.id}>{item.name}</li>
                    })}
                </ul>
            </div>
        </>
    )
}

export default ListObjectType;
