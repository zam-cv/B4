-- Your SQL goes here

ALTER TABLE crop_sections DROP FOREIGN KEY crop_sections_ibfk_2;
ALTER TABLE `crop_sections` DROP COLUMN `crop_type_id`;
ALTER TABLE `crop_sections` ADD COLUMN `crop_type_id` INTEGER;



ALTER TABLE `users` ADD COLUMN `max_sections` INTEGER NOT NULL;

