variable "images" {
  type = "list"
  default = [
    "stretch",
    "wheezy"
  ]
}

provider "docker" {}

resource "docker_container" "build" {
  count               = "${length(var.images)}"

  
  image               = "debian:${var.images[count.index]}"
  name                = "build-debian-${var.images[count.index]}"
  rm                  = true

  command             = [
    "bash",
    "-c",
    <<END
      cd /app
      apt-get update
      apt-get install -y curl gcc
      
      curl https://sh.rustup.rs -sSf | sh -s -- \
        -y \
        --default-host x86_64-unknown-linux-gnu \
        --default-toolchain nightly-x86_64-unknown-linux-gnu 
      source /root/.cargo/env
      cargo install cargo-deb
      cargo deb
      mkdir -p target/debian-${var.images[count.index]}
      cp target/debian/* target/debian-${var.images[count.index]}
END
  ]

  volumes {
    container_path    = "/app"
    host_path         = "${path.cwd}"
  }

  volumes {
    container_path    = "/app/target/debian"
  }

  provisioner "local-exec" {
    command = "docker logs --tail=100 -f ${self.id}"
  }
}
