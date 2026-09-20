data "cloudflare_zone" "cacheandbuffercom" {
  zone_id = var.zone_id
}

resource "cloudflare_dns_record" "verification" {
  for_each = toset([var.environment])
  zone_id  = data.cloudflare_zone.cacheandbuffercom.id
  name     = join(".", ["_amazonses", each.key, var.domain])
  type     = "TXT"
  content  = "\"${var.verification_token}\""
  ttl      = 1
}

locals {
  dkims = flatten([
    for env in toset([var.environment]) : [
      for dkim_token in toset(var.dkim_tokens) : {
        env   = env
        token = dkim_token
      }
    ]
  ])
}

resource "cloudflare_dns_record" "dkim" {
  for_each = { for dkim in local.dkims : join("-", [dkim.env, dkim.token]) => dkim }
  zone_id  = data.cloudflare_zone.cacheandbuffercom.id
  name     = join(".", [each.value.token, "_domainkey", each.value.env, var.domain])
  type     = "CNAME"
  content  = join(".", [each.value.token, "dkim.amazonses.com"])
  proxied  = false
  ttl      = 1
}

resource "cloudflare_dns_record" "mx" {
  for_each = toset([var.environment])
  zone_id  = data.cloudflare_zone.cacheandbuffercom.id
  name     = join(".", ["mail", each.key, var.domain])
  type     = "MX"
  content  = "feedback-smtp.${var.aws_region}.amazonses.com"
  priority = 10
  ttl      = 1
}

resource "cloudflare_dns_record" "spf" {
  for_each = toset([var.environment])
  zone_id  = data.cloudflare_zone.cacheandbuffercom.id
  name     = join(".", ["mail", each.key, var.domain])
  type     = "TXT"
  content  = "\"v=spf1 include:amazonses.com ~all\""
  ttl      = 1
}

resource "cloudflare_dns_record" "dmarc" {
  for_each = toset([var.environment])
  zone_id  = data.cloudflare_zone.cacheandbuffercom.id
  name     = join(".", ["_dmarc", each.key, var.domain])
  type     = "TXT"
  content  = "\"v=DMARC1; p=none;\""
  ttl      = 1
}

# resource "cloudflare_dns_record" "vps" {
#   for_each = toset([var.environment])
#   zone_id  = data.cloudflare_zone.cacheandbuffercom.id
#   name     = join(".", [each.key, var.domain])
#   type     = "A"
#   content  = "133.167.103.109"
#   proxied  = false
#   ttl      = 1
# }
