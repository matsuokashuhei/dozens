resource "aws_ses_domain_identity" "cognito" {
  for_each = toset([var.environment])
  domain   = join(".", [each.key, var.domain])
}

resource "aws_ses_domain_dkim" "cognito" {
  for_each = toset([var.environment])
  domain   = aws_ses_domain_identity.cognito[each.key].domain
}

resource "aws_ses_domain_mail_from" "cognito" {
  for_each               = toset([var.environment])
  domain                 = aws_ses_domain_identity.cognito[each.key].domain
  mail_from_domain       = join(".", ["mail", aws_ses_domain_identity.cognito[each.key].domain])
  behavior_on_mx_failure = "RejectMessage"
}
