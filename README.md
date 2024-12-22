# Todo App
This is a simple todo app written in rust and its using sqlite as a underlying database


# Project Setup
This project is using the diesel library to work with databases.

For simpler work with diesel you need to install the CLI seperately. Please follow the instructions on their website (see [diesel.rs](https://diesel.rs/guides/getting-started)).

To initialize a dummy database for local testing, add the database url in the .env file on project root level with following content ```DATABASE_URL=database/todo-list.db``` and execute the command ```diesel setup``` 

# Important commands for migrations
- ```diesel migration generate --diff-schema <name of migration>``` (to create migration in migration folder based on the schema.rs)
- ```diesel migration run``` (to apply all pending migrations)