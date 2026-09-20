data "archive_file" "custom_email_sender" {
  type        = "zip"
  source_dir  = "${path.module}/lambda/custom_email_sender"
  output_path = "${path.module}/lambda/custom_email_sender.zip"
  excludes    = ["package-lock.json"]
}

resource "aws_kms_key" "cognito_email_sender" {
  for_each                = toset([var.environment])
  description             = "Cognito custom email sender secrets (${var.project_name}-${each.key})"
  deletion_window_in_days = 7
  enable_key_rotation     = true

  tags = {
    Environment = each.key
    Project     = var.project_name
  }
}

resource "aws_kms_alias" "cognito_email_sender" {
  for_each      = toset([var.environment])
  name          = "alias/${var.project_name}-${each.key}-cognito-email-sender"
  target_key_id = aws_kms_key.cognito_email_sender[each.key].key_id
}

resource "aws_iam_role" "custom_email_sender" {
  for_each = toset([var.environment])
  name     = join("-", [var.project_name, each.key, "custom-email-sender"])

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Effect    = "Allow"
      Principal = { Service = "lambda.amazonaws.com" }
      Action    = "sts:AssumeRole"
    }]
  })
}

resource "aws_iam_role_policy_attachment" "custom_email_sender" {
  for_each   = toset([var.environment])
  role       = aws_iam_role.custom_email_sender[each.key].name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
}

resource "aws_iam_role_policy" "custom_email_sender" {
  for_each = toset([var.environment])
  name     = "custom-email-sender"
  role     = aws_iam_role.custom_email_sender[each.key].id

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect   = "Allow"
        Action   = ["kms:Decrypt", "kms:DescribeKey"]
        Resource = [aws_kms_key.cognito_email_sender[each.key].arn]
      },
      {
        Effect = "Allow"
        Action = ["ses:SendEmail", "ses:SendRawEmail"]
        Resource = [
          aws_ses_domain_identity.cognito[each.key].arn,
          "arn:aws:ses:${data.aws_region.current.region}:${data.aws_caller_identity.current.account_id}:identity/*",
        ]
      },
    ]
  })
}

resource "aws_cloudwatch_log_group" "custom_email_sender" {
  for_each          = toset([var.environment])
  name              = "/aws/lambda/${join("-", [var.project_name, each.key, "custom-email-sender"])}"
  retention_in_days = 14
}

resource "aws_lambda_function" "custom_email_sender" {
  for_each         = toset([var.environment])
  function_name    = join("-", [var.project_name, each.key, "custom-email-sender"])
  role             = aws_iam_role.custom_email_sender[each.key].arn
  handler          = "index.handler"
  runtime          = "nodejs20.x"
  filename         = data.archive_file.custom_email_sender.output_path
  source_code_hash = data.archive_file.custom_email_sender.output_base64sha256
  timeout          = 15
  memory_size      = 256

  environment {
    variables = {
      KMS_KEY_ARN = aws_kms_key.cognito_email_sender[each.key].arn
      FROM_EMAIL  = join("@", ["no-reply", aws_ses_domain_identity.cognito[each.key].domain])
    }
  }

  depends_on = [
    aws_cloudwatch_log_group.custom_email_sender,
    aws_iam_role_policy_attachment.custom_email_sender,
    aws_iam_role_policy.custom_email_sender,
  ]

  tags = {
    Environment = each.key
    Project     = var.project_name
  }
}

resource "aws_lambda_permission" "custom_email_sender" {
  for_each      = toset([var.environment])
  statement_id  = "AllowCognitoInvoke"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.custom_email_sender[each.key].function_name
  principal     = "cognito-idp.amazonaws.com"
  source_arn    = aws_cognito_user_pool.user[each.key].arn
}
