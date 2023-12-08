import { config } from '../../config';
import mysql from 'mysql2';

const conn = mysql.createPool({
    host: config.db.host,
    port: config.db.port,
    database: config.db.db_name,
    user: config.db.username,
    password: config.db.password,
});

export { conn };