import dayjs, { Dayjs } from "dayjs";

import utc from 'dayjs/plugin/utc'
import timezone from 'dayjs/plugin/timezone'

dayjs.extend(utc);
dayjs.extend(timezone);

function now_china(): dayjs.Dayjs {
    return dayjs().tz("Asia/Shanghai");
}

const DB_TIME_FORMAT = 'YYYY-MM-DD HH:mm:ss'

export {
    DB_TIME_FORMAT,
    now_china,
}
