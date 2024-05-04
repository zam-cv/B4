terraform {
  required_providers {
    oci = {
      source  = "hashicorp/oci"
      version = "= 5.40.0"
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
    source      = "10.0.0.0/24"
    description = "access to container instance port 3306 from home"

    tcp_options {
      min = 3306
      max = 3306
    }
  }

  egress_security_rules {
    protocol         = 6
    destination_type = "CIDR_BLOCK"
    destination      = "10.0.0.0/24"
    description      = "access to container instance port 3306"

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
    description = "access to container instance port 1420 from anywhere"

    tcp_options {
      min = 1420
      max = 1420
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
    description      = "access to container registries via HTTP"

    tcp_options {
      min = 80
      max = 80
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
    memory_in_gbs = 8
  }

  vnics {
    subnet_id      = oci_core_subnet.qrops_subnet.id
  }

  containers {
    image_url    = "mysql:8.0"
    display_name = "mysql-server"
    environment_variables = {
      MYSQL_ROOT_PASSWORD = var.mysql_root_password
      MYSQL_DATABASE      = var.mysql_db_name
      MYSQL_USER          = var.mysql_user
      MYSQL_PASSWORD      = var.mysql_password
      MYSQL_BIND_ADDRESS  = "0.0.0.0"
    }
  }
}

resource "oci_container_instances_container_instance" "app" {
  availability_domain      = data.oci_identity_availability_domains.local_ads.availability_domains.0.name
  compartment_id           = var.compartment_id
  display_name             = "app"
  freeform_tags            = { "project-name" = "crops" }
  container_restart_policy = "ALWAYS"
  shape                    = "CI.Standard.E4.Flex"

  shape_config {
    ocpus         = 4
    memory_in_gbs = 8
  }

  vnics {
    subnet_id = oci_core_subnet.qrops_subnet.id
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
      DATABASE_HOST          = oci_container_instances_container_instance.database.vnics.0.private_ip
      DATABASE_URL           = "mysql://${var.mysql_user}:${var.mysql_password}@${oci_container_instances_container_instance.database.vnics.0.private_ip}:3306/${var.mysql_db_name}"
      ADMIN_DEFAULT_EMAIL    = var.admin_default_email
      ADMIN_DEFAULT_PASSWORD = var.admin_default_password
      SMTP_HOST              = var.smtp_host
      SMTP_USERNAME          = var.smtp_username
      SMTP_PASSWORD          = var.smtp_password
    }
  }
}

resource "oci_container_instances_container_instance" "platform" {
  availability_domain      = data.oci_identity_availability_domains.local_ads.availability_domains.0.name
  compartment_id           = var.compartment_id
  display_name             = "platform"
  freeform_tags            = { "project-name" = "crops" }
  container_restart_policy = "ALWAYS"
  shape                    = "CI.Standard.E4.Flex"

  shape_config {
    ocpus         = 4
    memory_in_gbs = 16
  }

  vnics {
    subnet_id = oci_core_subnet.qrops_subnet.id
  }

  depends_on = [
    oci_container_instances_container_instance.app
  ]

  containers {
    image_url    = "zamcv/platform"
    display_name = "platform-server"
  }
}

data "oci_core_vnic" "app_vnic" {
  vnic_id = oci_container_instances_container_instance.app.vnics[0].vnic_id
}

data "oci_core_vnic" "platform_vnic" {
  vnic_id = oci_container_instances_container_instance.platform.vnics[0].vnic_id
}

output "app_public_ip" {
  value = data.oci_core_vnic.app_vnic.public_ip_address
}

output "platform_public_ip" {
  value = data.oci_core_vnic.platform_vnic.public_ip_address
}