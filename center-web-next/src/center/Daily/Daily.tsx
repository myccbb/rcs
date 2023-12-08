import {Divider} from "@/src/web-components/Divider";

import * as css from 'csstype';

import {ChevronDownIcon, MenuIcon, MoreHorizontal, PlusIcon} from "lucide-react";

import * as react from 'react';
import {root} from "postcss";
import {Notification, NotificationContext} from "@/src/web-components/Notification";


interface DndInfo {
    selectedItemId: string,
    srcTreeId: string,
    srcPos?: number,
    srcTree?: Tree,
    prevTreeId: string,
    prevPos?: number,
    prevTree?: Tree,
    currentTreeId?: string,
    currentPos?: number,
    currentTree?: Tree,
    destTreeId: string,
    destPos?: number,
    destTree?: Tree,
}


let dndInfo: DndInfo = {
    selectedItemId: '',
    srcTreeId: '',
    prevTreeId: '',
    destTreeId: '',
};

let DndContext = react.createContext(dndInfo);


function Daily() {
    let rootTree: Tree = {
        id: 'root',
        title: 'root',
        subTree: [
            {
                title: 'a',
                id: 'a',
            },
            {
                title: 'b',
                id: 'b',
                subTree: [
                    {
                        title: 'b-a',
                        id: 'b-a',
                    },
                    {
                        title: 'b-b',
                        id: 'b-b',
                    },
                    {
                        title: 'b-c',
                        id: 'b-c',
                    },
                ],
            },
            {
                title: 'c',
                id: 'c',
                subTree: [
                    {
                        title: 'c-a',
                        id: 'c-a',
                    },
                    {
                        title: 'c-b',
                        id: 'c-b',
                    },
                    {
                        title: 'c-c',
                        id: 'c-c',
                    },
                ],
            },
        ],
    }

    // function handleKeyDown(e: react.KeyboardEvent<HTMLDivElement>) {
    //     console.log('daily ' + e.key);
    // }

    return (
        <>
            <Notification/>
            <div
                className={'flex flex-row h-screen'}
                // onKeyDown={handleKeyDown}
            >
                <DndContext.Provider value={dndInfo}>
                    <CollectionTree className={'w-1/5 p-1'} tree={rootTree}/>
                </DndContext.Provider>
                <Divider type={'vertical'} className={'h-full'}/>
                <div className={'p-1'}>
                    <p>right</p>
                </div>
            </div>
        </>
    )
}


interface Tree {
    id: string,
    title: string,
    subTree?: Tree[],
}

function moveTree(
    tree: Tree,
    srcSubTreeId: string,
    srcSubTreePos: number | undefined,
    destSubTreeId: string,
    destSubTreePos: number,
) {
    if (srcSubTreePos === undefined) {
        return
    }
    if (srcSubTreeId === destSubTreeId && srcSubTreePos === destSubTreePos) {
        return
    }
    // console.log('moveTree', srcSubTreeId, srcSubTreePos, destSubTreeId, destSubTreePos)
    let srcSubTree = findSubTreeById(tree, srcSubTreeId)
    if (srcSubTree === null || srcSubTree.subTree === undefined) {
        return
    }
    let targetTree = srcSubTree.subTree[srcSubTreePos]
    srcSubTree.subTree.splice(srcSubTreePos, 1)
    let destSubTree = findSubTreeById(tree, destSubTreeId)
    if (destSubTree === null || destSubTree.subTree === undefined) {
        return
    }
    destSubTree.subTree.splice(destSubTreePos, 0, targetTree)
}

function findSubTreeById(tree: Tree, idStr: string): Tree | null {
    if (tree.id === idStr) {
        return tree
    }
    if (tree.subTree === undefined) {
        return null
    }
    for (const subTree of tree.subTree) {
        if (subTree.id == idStr) {
            return subTree
        }
        let result = findSubTreeById(subTree, idStr)
        if (result !== null) {
            return result
        }
    }
    return null
}

function CollectionTree({className, tree}: {
    className?: string,
    tree: Tree,
}) {
    const [rootTree, setRootTree] = react.useState(tree)
    const [selectedItemId, setSelectedItemId] = react.useState('')

    let notice = react.useContext(NotificationContext);

    let defaultPrompt = 'Create Collection';
    let [collectionNamePrompt, setCollectionNamePrompt] = react.useState(defaultPrompt)
    let collectionNameRef = react.useRef('');

    function handleFocus(e: react.FocusEvent<HTMLDivElement>) {
        setCollectionNamePrompt('')
    }

    function handleBlur(e: react.FocusEvent<HTMLDivElement>) {
        const content = e.currentTarget.textContent ?? '';
        if (content === '') {
            setCollectionNamePrompt(defaultPrompt);
        }
    }

    function handleKeyDown(e: react.KeyboardEvent<HTMLDivElement>) {
        const content = e.currentTarget.textContent ?? '';
        if (e.key === 'Enter') {
            e.preventDefault()
            if (content.length == 0) {
                notice.info('collection name cannot be empty')
                return
            }
            const newTreeId = 'newCollection';
            let newTree: Tree = {
                id: newTreeId,
                title: content,
            }
            let existTree = findSubTreeById(rootTree, newTreeId);
            if (existTree === null) {
                if (rootTree.subTree === undefined) {
                    rootTree.subTree = [newTree]
                } else {
                    rootTree.subTree.splice(rootTree.subTree.length, 0, newTree)
                }
                setRootTree({...rootTree})
            }
        }
        // console.log(e.key);
        const MaxCollectionNameLength = 30
        if (content.length > MaxCollectionNameLength && e.key !== 'Backspace') {
            e.preventDefault()
            notice.info("collection name too long")
        }
    }

    return (
        <div className={'flex flex-col ' + className ?? ''}>

            <CollectionList
                depth={0}
                curTree={rootTree}
                rootTree={rootTree}
                setRootTree={setRootTree}
                setSelectedItemId={setSelectedItemId}
            />
            <div
                className={'h-10 w-full mt-auto flex flex-row items-center hover:bg-gray-200'}
                // onClick={handleClickCreate}
            >
                <div className={'px-2'}></div>
                <PlusIcon>hello</PlusIcon>
                <div className={'px-1'}></div>
                <div
                    contentEditable={true}
                    className={'h-max w-full outline-0 text-gray-500'}
                    onFocus={handleFocus}
                    onBlur={handleBlur}
                    suppressContentEditableWarning={true}
                    onKeyDown={handleKeyDown}
                >{collectionNamePrompt}</div>
                <div className={'px-2'}></div>
            </div>
        </div>
    )
}


function CollectionList(
    {
        depth, curTree,
        parentTree,
        rootTree, setRootTree,
        setSelectedItemId,
    }: {
        depth: number,
        curTree: Tree,
        parentTree?: Tree,
        rootTree: Tree,
        setRootTree: react.Dispatch<react.SetStateAction<Tree>>,
        setSelectedItemId: react.Dispatch<react.SetStateAction<string>>,
    }) {
    if (curTree.subTree === undefined) {
        return <></>
    }
    let dnd = react.useContext(DndContext);

    function handleDragStart(e: react.DragEvent<HTMLLIElement>) {
        if (curTree.subTree === undefined) {
            return
        }
        dnd.srcPos = Number(e.currentTarget.dataset.position);
        dnd.srcTreeId = e.currentTarget.dataset.treeid ?? '';
        dnd.srcTree = curTree.subTree[dnd.srcPos];
        dnd.prevPos = Number(e.currentTarget.dataset.position);
        dnd.prevTreeId = e.currentTarget.dataset.treeid ?? '';
        dnd.prevTree = curTree.subTree[dnd.prevPos];
    }

    function handleDragOver(e: react.DragEvent<HTMLLIElement>) {
        e.preventDefault()
        if (curTree.subTree === undefined) {
            return
        }
        let currentPos = Number(e.currentTarget.dataset.position);
        let currentTreeId = e.currentTarget.dataset.treeid ?? '';
        if (dnd.currentTreeId == currentTreeId && dnd.currentPos == currentPos) {
            return
        }
        // console.log(e.currentTarget.dataset.treeId);
        dnd.currentPos = currentPos;
        dnd.currentTreeId = currentTreeId;
        dnd.currentTree = curTree.subTree[currentPos];
        moveTree(rootTree,
            dnd.prevTreeId, dnd.prevPos,
            dnd.currentTreeId, dnd.currentPos)
        setRootTree({...rootTree})
        dnd.prevPos = dnd.currentPos;
        dnd.prevTree = dnd.currentTree;
        dnd.prevTreeId = dnd.currentTreeId;
    }

    function handleDrop(e: react.DragEvent<HTMLLIElement>) {
        e.preventDefault()
        if (curTree.subTree === undefined) {
            return
        }
        let currentPos = Number(e.currentTarget.dataset.position);
        let currentTreeId = e.currentTarget.dataset.treeid ?? '';
        if (dnd.srcTreeId == currentTreeId && dnd.srcPos == currentPos) {
            return
        }
        console.log("move from", dnd.srcTreeId, dnd.srcPos,
            "to", dnd.currentTreeId, dnd.currentPos)
    }

    function handleClickListItem(e: react.MouseEvent<HTMLLIElement>) {
        let itemId = e.currentTarget.dataset.itemid ?? '';
        dnd.selectedItemId = itemId;
        setSelectedItemId(itemId);
    }


    let scrollIfRoot = depth == 0 ? 'overflow-hidden overflow-y-auto' : '';
    return (
        <ul className={scrollIfRoot}>
            {curTree.subTree.map((item, index) => {
                let haveSubTree = item.subTree !== undefined && item.subTree.length > 0;
                return (
                    <div key={item.id + '-div'}>
                        <li
                            draggable={true}
                            data-position={index}
                            data-treeid={curTree === undefined ? '' : curTree.id}
                            data-itemid={item.id}
                            onDragStart={handleDragStart}
                            onDragOver={handleDragOver}
                            onDrop={handleDrop}
                            onClick={handleClickListItem}
                        >
                            <CollectionItem
                                itemId={item.id}
                                depth={depth}
                                haveSubTree={haveSubTree}
                            >
                                {item.title}
                            </CollectionItem>
                        </li>
                        <div className={'py-0.5'}></div>
                        {haveSubTree
                            && <CollectionList
                                depth={depth + 1}
                                curTree={item}
                                parentTree={curTree}
                                rootTree={rootTree}
                                setRootTree={setRootTree}
                                setSelectedItemId={setSelectedItemId}
                            />}
                    </div>
                )
            })}
        </ul>
    )
}


const ListItemStyle: css.Properties = {
    height: '2.5rem',
    userSelect: 'none',
    display: 'flex',
    flexDirection: 'row',
    alignItems: 'center',
    marginTop: '0.1rem',
    marginBottom: "0.1rem",
    // border: '0.1rem solid #E36C52',
    // borderRadius: '0.25rem',
}

const iconColor = '#E36C52'


function CollectionItem({itemId, depth, haveSubTree, children}: {
    itemId: string,
    depth: number
    haveSubTree: boolean
    children: react.ReactNode,
}) {
    let dnd = react.useContext(DndContext);
    let bgClassName = dnd.selectedItemId == itemId ? 'bg-gray-200' : '';
    return (
        <div style={ListItemStyle} className={'hover:bg-gray-200' + ' ' + bgClassName}>
            <div className={'px-2'}></div>
            {[...Array(depth)].map((_, index) => {
                return (
                    <div key={index} className={'px-2'}></div>
                )
            })}
            {haveSubTree
                ? <ChevronDownIcon color={iconColor}/>
                : <MenuIcon color={iconColor}></MenuIcon>}
            <div className={'px-1'}/>
            {children}
            <MoreHorizontal className={'ml-auto'}/>
        </div>
    )
}

export default Daily;
