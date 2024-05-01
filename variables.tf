variable "user_ocid" {
  description = "User OCID for Oracle Cloud Infrastructure"
  type        = string
}

variable "fingerprint" {
  description = "Public Key Fingerprint for Oracle Cloud Infrastructure"
  type        = string
}

variable "private_key_path" {
  description = "Path to the Private Key used for Oracle Cloud Infrastructure"
  type        = string
}

variable "tenancy_ocid" {
  description = "Tenancy OCID for Oracle Cloud Infrastructure"
  type        = string
}

variable "region" {
  description = "Region for Oracle Cloud Infrastructure"
  type        = string
}

variable "availability_domain" {
  description = "Availability Domain for Oracle Cloud Infrastructure"
  type        = string
}

variable "compartment_id" {
  description = "Compartment OCID for Oracle Cloud Infrastructure"
  type        = string
}

variable "mysql_user" {
  description = "User for the MySQL Database"
  type        = string
}

variable "mysql_password" {
  description = "Password for the MySQL Database"
  type        = string
}

variable "source_id" {
  description = "Image OCID for the Oracle Cloud Infrastructure"
  type        = string
}

variable "mode" {
  description = "Mode for the Application"
  type        = string
}

variable "rust_log" {
  description = "Rust Log Level"
  type        = string
}

variable "host" {
  description = "Host for the Application"
  type        = string
}

variable "port" {
  description = "Port for the Application"
  type        = number
}

variable "user_secret_key" {
  description = "User Secret Key for the Application"
  type        = string
}

variable "admin_secret_key" {
  description = "Admin Secret Key for the Application"
  type        = string
}

variable "ipinfo_token" {
  description = "IPInfo Token for the Application"
  type        = string
}

variable "admin_default_email" {
  description = "Admin Default Email for the Application"
  type        = string
}

variable "admin_default_password" {
  description = "Admin Default Password for the Application"
  type        = string
}

variable "smtp_host" {
  description = "SMTP Host for the Application"
  type        = string
}

variable "smtp_username" {
  description = "SMTP Username for the Application"
  type        = string
}

variable "smtp_password" {
  description = "SMTP Password for the Application"
  type        = string
}