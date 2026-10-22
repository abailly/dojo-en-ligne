variable "private_key_file" {
  type        = string
  description = "The private key file used to log into the machine and configure it"
}

variable "access_key" {
  type = string
  sensitive = true
}

variable "secret_key" {
  type = string
  sensitive = true
}

variable "organization_id" {
  type = string
  sensitive = true
}

variable "project_id" {
  type = string
  sensitive = true
}

