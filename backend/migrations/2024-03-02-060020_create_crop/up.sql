-- Your SQL goes here

CREATE TABLE `crop_types`(
	`id` INTEGER AUTO_INCREMENT NOT NULL PRIMARY KEY,
	`name` VARCHAR(50) NOT NULL,
	`price` INTEGER NOT NULL
);

CREATE TABLE `crop_sections`(
	`id` INTEGER AUTO_INCREMENT NOT NULL PRIMARY KEY,
	`user_id` INTEGER NOT NULL,
	`crop_type_id` INTEGER NOT NULL,
	`units` INTEGER NOT NULL,
	FOREIGN KEY (`user_id`) REFERENCES `users`(`id`),
	FOREIGN KEY (`crop_type_id`) REFERENCES `crop_types`(`id`)
);

