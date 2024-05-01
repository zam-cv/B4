terraform {
  required_providers {
    oci = {
      source  = "hashicorp/oci"
      version = "= 5.39.0"
    }
  }
}

provider "oci" {
  config_file_profile = "DEFAULT"
  user_ocid           = var.user_ocid
  fingerprint         = var.fingerprint
  private_key_path    = var.private_key_path
  tenancy_ocid        = var.tenancy_ocid
  region              = var.region
}

resource "oci_core_vcn" "qrops_vcn" {
  compartment_id = var.compartment_id
  cidr_block     = "10.0.0.0/16"
  display_name   = "qrops_vcn"
  dns_label      = "vcndns"

  freeform_tags = {
    "project-name" = "crops"
  }
}

resource "oci_core_security_list" "public_sn_sl" {
  compartment_id = var.compartment_id
  vcn_id         = oci_core_vcn.qrops_vcn.id
  display_name   = "security list for the public subnet"

  ingress_security_rules {
    protocol    = 6
    source_type = "CIDR_BLOCK"
    source      = "0.0.0.0/0"
    description = "access to container instance port 3306 from home"

    tcp_options {
      min = 3306
      max = 3306
    }
  }

  ingress_security_rules {
    protocol    = 6
    source_type = "CIDR_BLOCK"
    source      = "0.0.0.0/0"
    description = "access to container instance port 80 from anywhere"

    tcp_options {
      min = 80
      max = 80
    }
  }

  ingress_security_rules {
    protocol    = 6
    source_type = "CIDR_BLOCK"
    source      = "0.0.0.0/0"
    description = "access to container instance port 8080 from anywhere"

    tcp_options {
      min = 8080
      max = 8080
    }
  }

  ingress_security_rules {
    protocol    = 6
    source_type = "CIDR_BLOCK"
    source      = "0.0.0.0/0"
    description = "access to container instance port 22 from anywhere"

    tcp_options {
      min = 22
      max = 22
    }
  }

  egress_security_rules {
    protocol         = 6
    destination_type = "CIDR_BLOCK"
    destination      = "0.0.0.0/0"
    description      = "access to container registries via HTTPS"

    tcp_options {
      min = 443
      max = 443
    }
  }

  freeform_tags = {
    "project-name" = "crops"
  }
}

resource "oci_core_subnet" "qrops_subnet" {
  compartment_id = var.compartment_id
  vcn_id         = oci_core_vcn.qrops_vcn.id
  cidr_block     = "10.0.0.0/24"
  display_name   = "qrops_subnet"
  route_table_id = oci_core_route_table.igw_rt.id
  dns_label      = "subnetdns"

  security_list_ids = [
    oci_core_security_list.public_sn_sl.id
  ]

  freeform_tags = {
    "project-name" = "crops"
  }
}

resource "oci_core_internet_gateway" "internet_gateway" {
  compartment_id = var.compartment_id
  vcn_id         = oci_core_vcn.qrops_vcn.id
  display_name   = "internet_gateway"
  enabled        = true
}

resource "oci_core_route_table" "igw_rt" {
  compartment_id = var.compartment_id
  vcn_id         = oci_core_vcn.qrops_vcn.id
  display_name   = "Internet gateway route table"

  route_rules {
    network_entity_id = oci_core_internet_gateway.internet_gateway.id
    destination       = "0.0.0.0/0"
  }

  freeform_tags = {
    "project-name" = "crops"
  }
}

data "oci_identity_availability_domains" "local_ads" {
  compartment_id = var.compartment_id
}

resource "oci_container_instances_container_instance" "database" {
  availability_domain      = data.oci_identity_availability_domains.local_ads.availability_domains.0.name
  compartment_id           = var.compartment_id
  display_name             = "database"
  freeform_tags            = { "project-name" = "crops" }
  container_restart_policy = "ALWAYS"
  shape                    = "CI.Standard.E4.Flex"

  shape_config {
    ocpus         = 2
    memory_in_gbs = 16
  }

  vnics {
    subnet_id      = oci_core_subnet.qrops_subnet.id
    hostname_label = "db-server"
  }

  containers {
    image_url    = "mysql:8.0"
    display_name = "mysql-server"
    command = [
      "--default-authentication-plugin=caching_sha2_password",
      "--character-set-server=utf8mb4",
      "--collation-server=utf8mb4_unicode_ci"
    ]
    environment_variables = {
      MYSQL_USER     = var.mysql_user
      MYSQL_PASSWORD = var.mysql_password
    }
  }
}

resource "oci_container_instances_container_instance" "app" {
  availability_domain      = data.oci_identity_availability_domains.local_ads.availability_domains.0.name
  compartment_id           = var.compartment_id
  display_name             = "app"
  shape                    = "CI.Standard.E4.Flex"
  freeform_tags            = { "project-name" = "crops" }
  container_restart_policy = "ALWAYS"

  shape_config {
    ocpus         = 2
    memory_in_gbs = 16
  }

  vnics {
    subnet_id             = oci_core_subnet.qrops_subnet.id
    is_public_ip_assigned = true
  }

  depends_on = [
    oci_container_instances_container_instance.database
  ]

  containers {
    image_url    = "zamcv/qrops"
    display_name = "app-server"
    environment_variables = {
      MODE                   = var.mode
      RUST_LOG               = var.rust_log
      HOST                   = var.host
      PORT                   = var.port
      USER_SECRET_KEY        = var.user_secret_key
      ADMIN_SECRET_KEY       = var.admin_secret_key
      IPINFO_TOKEN           = var.ipinfo_token
      DATABASE_URL           = "mysql://${var.mysql_user}:${var.mysql_password}@db-server:3306/game"
      ADMIN_DEFAULT_EMAIL    = var.admin_default_email
      ADMIN_DEFAULT_PASSWORD = var.admin_default_password
      SMTP_HOST              = var.smtp_host
      SMTP_USERNAME          = var.smtp_username
      SMTP_PASSWORD          = var.smtp_password
    }
  }
}

resource "oci_core_public_ip" "app_public_ip" {
  provider       = oci
  compartment_id = var.compartment_id
  lifetime       = "RESERVED"
  depends_on     = [oci_container_instances_container_instance.app]
  display_name   = "app_public_ip"
}

output "app_public_ip" {
  value = oci_core_public_ip.app_public_ip.ip_address
}