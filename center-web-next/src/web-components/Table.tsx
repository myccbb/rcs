import * as react from 'react';
import EditPieceType from "../center/Admin/PieceType/EditPieceType";


function SimpleTable({th_list, data_list, operation}: {
    th_list: string[],
    data_list: any[],
    operation: null | react.ReactNode,
}) {
    return (
        <>
            <table className={'table-auto w-full'}>
                <thead className={'h-10 text-left border-b'}>
                <tr>
                    {th_list.map((item) => {
                        return <th key={item}>{item}</th>
                    })}
                </tr>
                </thead>
                <tbody className={'border-t'}>
                {data_list.map((row) => {
                    let values = Object.values(row)
                    if (operation != null) {
                        values.push(operation)
                    }
                    return <tr className={'h-10'} id={"tr-" + values[0]} key={values.toString()}>
                        {values.map((value: any) => {
                            if (react.isValidElement(value)) {
                                return <td key={value.toString()}>{value}</td>
                            }
                            return <td key={value.toString()}>{value.toString()}</td>
                        })}
                    </tr>
                })}
                </tbody>
            </table>
        </>
    )
}

export {
    SimpleTable,
}