variable "aws_region" {
  type    = string
  default = "ap-northeast-1"
}

variable "project_name" {
  type    = string
  default = "dozens"
}

variable "environment" {
  type    = string
  default = "local"
}

variable "domain" {
  type    = string
  default = "dozens.cacheandbuffer.com"
}

variable "zone_id" {
  type = string
}

variable "dkim_tokens" {
  type = list(string)
}

variable "verification_token" {
  type = string
}
