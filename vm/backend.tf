terraform {
  required_providers {
    scaleway = {
      source = "scaleway/scaleway"
    }
    multipass = {
      source  = "larstobi/multipass"
      version = "1.4.2"
    }
  }

  required_version = ">= 0.13"
}
