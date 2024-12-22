-- Your SQL goes here
CREATE TABLE `todos`(
	`id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	`description` TEXT NOT NULL,
	`is_done` BOOL NOT NULL,
	`created_at` TIMESTAMP NOT NULL,
	`modified_at` TIMESTAMP NOT NULL
);

