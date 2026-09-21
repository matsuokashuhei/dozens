resource "aws_cognito_user_pool" "user" {
  for_each = toset([var.environment])
  name     = join("-", [var.project_name, each.key, "user"])

  username_attributes      = ["email"]
  auto_verified_attributes = ["email"]
  user_pool_tier           = "ESSENTIALS"

  user_attribute_update_settings {
    attributes_require_verification_before_update = ["email"]
  }

  schema {
    name                = "name"
    attribute_data_type = "String"
    mutable             = true
    required            = false
    string_attribute_constraints {
      min_length = 0
      max_length = 64
    }
  }

  schema {
    name                = "email"
    attribute_data_type = "String"
    mutable             = true
    required            = true
    string_attribute_constraints {
      min_length = 0
      max_length = 2048
    }
  }

  mfa_configuration = "OFF"

  sign_in_policy {
    allowed_first_auth_factors = ["EMAIL_OTP", "PASSWORD"]
  }

  email_configuration {
    email_sending_account = "DEVELOPER"
    from_email_address    = join("@", ["no-reply", aws_ses_domain_identity.cognito[each.key].domain])
    source_arn            = aws_ses_domain_identity.cognito[each.key].arn
  }

  verification_message_template {
    default_email_option = "CONFIRM_WITH_CODE"
  }

  admin_create_user_config {
    allow_admin_create_user_only = false
  }

  lambda_config {
    kms_key_id = aws_kms_key.cognito_email_sender[each.key].arn
    custom_email_sender {
      lambda_arn     = aws_lambda_function.custom_email_sender[each.key].arn
      lambda_version = "V1_0"
    }
  }

  tags = {
    Environment = each.key
    Project     = var.project_name
  }
}

resource "aws_cognito_user_pool_client" "api" {
  for_each     = toset([var.environment])
  name         = join("-", [var.project_name, each.key, "api"])
  user_pool_id = aws_cognito_user_pool.user[each.key].id

  generate_secret = false

  explicit_auth_flows = [
    "ALLOW_USER_AUTH",
  ]

  refresh_token_rotation {
    feature                    = "ENABLED"
    retry_grace_period_seconds = 10
  }

  access_token_validity  = 1
  id_token_validity      = 1
  refresh_token_validity = 3650

  token_validity_units {
    access_token  = "days"
    id_token      = "days"
    refresh_token = "days"
  }
}
