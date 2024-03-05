-- This file should undo anything in `up.sql`

ALTER TABLE `crop_sections` DROP COLUMN `crop_type_id`;
ALTER TABLE `crop_sections` ADD COLUMN `crop_type_id` INTEGER NOT NULL;
ALTER TABLE crop_sections ADD CONSTRAINT crop_sections_ibfk_2 FOREIGN KEY (crop_type_id) REFERENCES crop_types(id);


ALTER TABLE `users` DROP COLUMN `max_sections`;

