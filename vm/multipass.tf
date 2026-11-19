provider "multipass" {}

resource "multipass_instance" "test" {
  name  = "dojo-vm"
  cpus  = 2
  image = "resolute"
}
