docker run -p 5432:5432 \
	-e POSTGRES_ROOT_USER=postgres \
	-e POSTGRES_ROOT_PASSWORD=postgres \
	-e POSTGRES_PASSWORD=postgres \
	-e POSTGRES_DATABASE=postgres \
	-v /home/himanshu/rust/todo/src/db/sql:/sql \
	--name postgres postgres:14