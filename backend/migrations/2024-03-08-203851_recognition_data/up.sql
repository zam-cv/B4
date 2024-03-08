-- Your SQL goes here
CREATE TABLE `users`(
	`id` INTEGER AUTO_INCREMENT NOT NULL PRIMARY KEY,
	`username` VARCHAR(50) NOT NULL,
	`password` VARCHAR(150) NOT NULL,
	`balance_cash` INTEGER NOT NULL,
	`balance_verqor` INTEGER NOT NULL,
	`balance_coyote` INTEGER NOT NULL,
	`current_day` TIMESTAMP NOT NULL,
	`max_sections` INTEGER NOT NULL,
	`ip` VARCHAR(30),
	`os` VARCHAR(50)
);

CREATE TABLE `crop_types`(
	`name` VARCHAR(50) NOT NULL PRIMARY KEY,
	`price` INTEGER NOT NULL
);

CREATE TABLE `crop_sections`(
	`id` INTEGER AUTO_INCREMENT NOT NULL PRIMARY KEY,
	`user_id` INTEGER NOT NULL,
	`crop_type_id` VARCHAR(50),
	`units` INTEGER NOT NULL,
	FOREIGN KEY (`user_id`) REFERENCES `users`(`id`),
	FOREIGN KEY (`crop_type_id`) REFERENCES `crop_types`(`name`)
);

CREATE TABLE `admins`(
	`id` INTEGER AUTO_INCREMENT NOT NULL PRIMARY KEY,
	`username` VARCHAR(50) NOT NULL,
	`password` VARCHAR(150) NOT NULL
);

CREATE TABLE `statistics`(
	`user_id` INTEGER NOT NULL,
	`id` INTEGER AUTO_INCREMENT NOT NULL PRIMARY KEY,
	`date` TIMESTAMP NOT NULL,
	`punctuation` INTEGER NOT NULL,
	FOREIGN KEY (`user_id`) REFERENCES `users`(`id`)
);

