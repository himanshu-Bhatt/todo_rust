# docker exec -it postgres psql -U postgres -d postgres -c "
#     insert into todo (cid, ctime, mid, mtime, title, status) values (787, '2003-11-10 09:37:35', 2093, '2008-09-21 13:59:48', 'sit', 'open');
# insert into todo (cid, ctime, mid, mtime, title, status) values (175, '2016-09-04 19:44:10', 241, '2007-05-26 03:37:12', 'consectetur', 'open');
# insert into todo (cid, ctime, mid, mtime, title, status) values (707, '2011-10-08 20:16:01', 7297, '2003-03-16 05:26:29', 'Lorem', 'close');
# insert into todo (cid, ctime, mid, mtime, title, status) values (971, '2005-11-07 14:48:38', 6109, '2003-10-20 00:11:01', 'adipiscing', 'close');
# insert into todo (cid, ctime, mid, mtime, title, status) values (855, '2009-10-27 07:37:42', 9555, '2004-02-28 04:01:19', 'sit', 'open');
# insert into todo (cid, ctime, mid, mtime, title, status) values (234, '2014-09-01 19:37:19', 9503, '2012-05-30 14:30:08', 'consectetur', 'close');
# "
docker exec -it postgres psql -U postgres -d postgres -c "
    CREATE TYPE todo_status_enum AS ENUM (
     'open',
     'close'
    );
    CREATE TABLE todo (
    id BIGSERIAL PRIMARY KEY,
    cid bigint NOT NULL, 
	ctime timestamp with time zone DEFAULT now(),
    mid bigint, 
	mtime timestamp with time zone,   
    title text NOT NULL,
    status todo_status_enum NOT NULL DEFAULT 'open'
);
"
# docker exec -it postgres psql -U postgres -d postgres -c "
#     SELECT * FROM todo;
# "
