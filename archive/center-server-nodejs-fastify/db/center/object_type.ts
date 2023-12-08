import { Dayjs } from "dayjs";
import { Category } from "../../types/object_type";

class ObjectType {
    internal_id: number = 0;
    id: string = '';
    name: string = '';
    category: Category = Category.Invalid;
    create_time: string = '';
    update_time: string = '';
}

