-- Your SQL goes here
CREATE TABLE `admins`(
	`id` INTEGER AUTO_INCREMENT NOT NULL PRIMARY KEY,
	`email` VARCHAR(50) NOT NULL,
	`password` VARCHAR(150) NOT NULL
);

CREATE TABLE `players`(
	`id` INTEGER AUTO_INCREMENT NOT NULL PRIMARY KEY,
	`current_cycle` INTEGER NOT NULL,
	`current_score` FLOAT4(30),
	`balance_cash` INTEGER NOT NULL,
	`balance_verqor` INTEGER NOT NULL,
	`balance_coyote` INTEGER NOT NULL,
	`max_plots` INTEGER NOT NULL
);

CREATE TABLE `users`(
	`id` INTEGER AUTO_INCREMENT NOT NULL PRIMARY KEY,
	`user_type` ENUM(
		'cliente',
		'agricultor',
		'fabricante_o_distribuidor_de_agroinsumos',
		'proverdor_de_seguros',
		'financiera',
		'empresa_cfg',
		'acopiador',
		'inversionista',
		'publico_en_general'
	) NOT NULL,
	`username` VARCHAR(50) NOT NULL,
	`email` VARCHAR(255) NOT NULL,
	`password` VARCHAR(150) NOT NULL,
	`gender` ENUM('m', 'f', 'x') NOT NULL,
	`os` VARCHAR(50),
	`player_id` INTEGER NOT NULL,
	`longitude` FLOAT4(30),
	`latitude` FLOAT4(30),
	`year_of_birth` INTEGER NOT NULL,
	FOREIGN KEY (`player_id`) REFERENCES `players`(`id`)
);

CREATE TABLE `loans`(
	`id` INTEGER AUTO_INCREMENT NOT NULL PRIMARY KEY,
	`status` VARCHAR(10) NOT NULL,
	`cycle` INTEGER NOT NULL,
	`amount` INTEGER NOT NULL,
	`creditor` VARCHAR(10) NOT NULL,
	`player_id` INTEGER NOT NULL,
	FOREIGN KEY (`player_id`) REFERENCES `players`(`id`)
);

CREATE TABLE `insurance`(
	`id` INTEGER AUTO_INCREMENT NOT NULL PRIMARY KEY,
	`insurance_type` VARCHAR(10) NOT NULL,
	`sum_assured` INTEGER NOT NULL,
	`loan_id` INTEGER NOT NULL,
	FOREIGN KEY (`loan_id`) REFERENCES `loans`(`id`)
);

CREATE TABLE `statistics`(
	`id` INTEGER AUTO_INCREMENT NOT NULL PRIMARY KEY,
	`cycle` INTEGER NOT NULL,
	`score` INTEGER NOT NULL,
	`player_id` INTEGER NOT NULL,
	FOREIGN KEY (`player_id`) REFERENCES `players`(`id`)
);

CREATE TABLE `crop_types`(
	`name` VARCHAR(50) NOT NULL PRIMARY KEY,
	`price` INTEGER NOT NULL,
	`duration` INTEGER NOT NULL
);

CREATE TABLE `plots`(
	`id` INTEGER AUTO_INCREMENT NOT NULL PRIMARY KEY,
	`crop_type_id` VARCHAR(50),
	`player_id` INTEGER NOT NULL,
	FOREIGN KEY (`crop_type_id`) REFERENCES `crop_types`(`name`),
	FOREIGN KEY (`player_id`) REFERENCES `players`(`id`)
);

CREATE TABLE `roles`(
	`id` INTEGER AUTO_INCREMENT NOT NULL PRIMARY KEY,
	`name` ENUM('admin', 'user') NOT NULL
);

CREATE TABLE `permissions`(
	`id` INTEGER AUTO_INCREMENT NOT NULL PRIMARY KEY,
	`name` ENUM(
		'view_documents',
		'view_dashboard',
		'view_distribution',
		'add_accounts',
		'edit_accounts',
		'send_emails'
	) NOT NULL
);

CREATE TABLE `role_permissions`(
	`id` INTEGER AUTO_INCREMENT NOT NULL PRIMARY KEY,
	`role_id` INTEGER NOT NULL,
	`permission_id` INTEGER NOT NULL,
	FOREIGN KEY (`role_id`) REFERENCES `roles`(`id`),
	FOREIGN KEY (`permission_id`) REFERENCES `permissions`(`id`)
);

CREATE TABLE `user_roles`(
	`id` INTEGER AUTO_INCREMENT NOT NULL PRIMARY KEY,
	`user_id` INTEGER NOT NULL,
	`role_id` INTEGER NOT NULL,
	FOREIGN KEY (`user_id`) REFERENCES `users`(`id`),
	FOREIGN KEY (`role_id`) REFERENCES `roles`(`id`)
);

CREATE TABLE `admin_roles`(
	`id` INTEGER AUTO_INCREMENT NOT NULL PRIMARY KEY,
	`admin_id` INTEGER NOT NULL,
	`role_id` INTEGER NOT NULL,
	FOREIGN KEY (`admin_id`) REFERENCES `admins`(`id`),
	FOREIGN KEY (`role_id`) REFERENCES `roles`(`id`)
);