
docker exec -it postgres psql -U postgres -d postgres -c "
    CREATE TYPE todo_status_enum AS ENUM (
     'open',
     'close'
    );
    CREATE TABLE todo (
    id BIGSERIAL PRIMARY KEY,
    cid bigint NOT NULL, 
	ctime timestamp with time zone DEFAULT now(),
    mid BIGSERIAL, 
	mtime timestamp with time zone,   
    title text NOT NULL,
    status todo_status_enum NOT NULL DEFAULT 'open'
);
"
# docker exec -it postgres psql -U postgres -d postgres -c "
#     SELECT * FROM todo;
# "
