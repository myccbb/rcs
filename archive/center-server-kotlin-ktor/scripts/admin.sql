-- set root password
alter user 'root'@'localhost' identified with mysql_native_password by 'xxx';

-- create user center
create user 'center'@'localhost' identified by 'xxx';

-- give center db access to center user
grant all privileges on center.* to 'center'@'localhost';