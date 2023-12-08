import * as react from 'react';
import {createContext, useContext, useState} from "react";

type NotificationItem = {
    content: string,
    id: number,
}

class NotificationList {
    name: string;
    notificationList: NotificationItem[];
    setNotificationList: (l: NotificationItem[]) => void;

    constructor(name: string) {
        this.name = name;
        this.setNotificationList = (l: NotificationItem[]) => {
        };
        this.notificationList = [];
    }

    init(f: (l: NotificationItem[]) => void) {
        this.setNotificationList = f;
    }

    shift() {
        if (this.notificationList.length <= 0) {
            return;
        }
        this.notificationList = this.notificationList.slice(1);
        this.setNotificationList(this.notificationList);
    }

    info(content: string) {
        const notificationContent = {
            content: content,
            id: Date.now(),
        }
        this.notificationList = [...this.notificationList, notificationContent];
        // console.log(this.name + ' push called, length ' + this.notificationList.length);
        this.setNotificationList(this.notificationList);
        setTimeout(() => {
            this.shift();
            // console.log(this.name + ' shift called, length ' + this.notificationList.length);
        }, 3000);
    }
}


const NotificationContext = createContext(new NotificationList('global'));


function Notification() {
    const [list, setList] = useState<NotificationItem[]>([]);
    let notificationList = useContext(NotificationContext);
    notificationList.init(setList);
    return (
        <>
            <NotificationContext.Provider value={notificationList}>
            </NotificationContext.Provider>
            <ul
                className={'z-40 fixed top-0 left-1/2 flex flex-col items-center'}
                style={{transform: "translate(-50%, 0)",}}
            >
                {list.map((notification: NotificationItem) => {
                    return <li
                        key={notification.id}
                        className={'h-10'}
                    >{notification.content}</li>
                })}
            </ul>
        </>
    );
}

export type {
    NotificationItem,
}

export {
    Notification,
    NotificationList,
    NotificationContext,
}
