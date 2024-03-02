-- Your SQL goes here

CREATE TABLE `users`(
	`id` INTEGER AUTO_INCREMENT NOT NULL PRIMARY KEY,
	`username` VARCHAR(50) NOT NULL,
	`password` VARCHAR(150) NOT NULL,
	`balance_cash` INTEGER NOT NULL,
	`balance_verqor` INTEGER NOT NULL,
	`balance_coyote` INTEGER NOT NULL,
	`current_day` TIMESTAMP NOT NULL
);

CREATE TABLE `statistics`(
	`user_id` INTEGER NOT NULL,
	`id` INTEGER AUTO_INCREMENT NOT NULL PRIMARY KEY,
	`date` TIMESTAMP NOT NULL,
	`punctuation` INTEGER NOT NULL,
	FOREIGN KEY (`user_id`) REFERENCES `users`(`id`)
);

