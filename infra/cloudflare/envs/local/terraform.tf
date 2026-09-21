terraform {
  required_version = ">= 1.6"

  backend "s3" {
    bucket = "dozens-tfstate-967026628831"
    key    = "cloudflare/local/terraform.tfstate"
    region = "ap-northeast-1"
  }

  required_providers {
    cloudflare = {
      source  = "cloudflare/cloudflare"
      version = ">= 5.0"
    }
  }
}

provider "cloudflare" {
}
