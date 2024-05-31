resource "aws_iam_user" "tegmen" {
  name = "tegmen"
}

data "aws_iam_policy_document" "tegmen_code_deploy_policy_statement" {
  statement {
    effect    = "Allow"
    actions   = ["codedeploy:*"]
    resources = ["*"]
  }
}

resource "aws_iam_policy" "tegmen_code_deploy_policy" {
  name   = "tegmen_code_deploy_policy"
  policy = data.aws_iam_policy_document.tegmen_code_deploy_policy_statement.json
}

resource "aws_iam_user_policy_attachment" "tegmen_code_deploy_policy" {
  user       = aws_iam_user.tegmen.name
  policy_arn = aws_iam_policy.tegmen_code_deploy_policy.arn
}

data "aws_iam_policy_document" "deploy_service_role_assume_policy" {
  statement {
    effect = "Allow"

    principals {
      type        = "AWS"
      identifiers = [aws_iam_user.tegmen.arn]
    }

    actions = ["sts:AssumeRole"]
  }
}

resource "aws_iam_role" "deploy_service_role" {
  name                = "deploy_service_role"
  assume_role_policy  = data.aws_iam_policy_document.deploy_service_role_assume_policy.json
  managed_policy_arns = ["arn:aws:iam::aws:policy/AmazonAPIGatewayInvokeFullAccess"]
}

data "aws_iam_policy_document" "tegmen_policy_assume_role" {
  statement {
    effect    = "Allow"
    actions   = ["sts:AssumeRole"]
    resources = [aws_iam_role.deploy_service_role.arn]
  }
}

resource "aws_iam_user_policy" "tegmen_user_policy" {
  name   = "tegmen_user_policy"
  user   = aws_iam_user.tegmen.name
  policy = data.aws_iam_policy_document.tegmen_policy_assume_role.json
}

data "aws_iam_policy_document" "deploy_application_role_assume_policy" {
  statement {
    effect = "Allow"

    principals {
      type        = "Service"
      identifiers = ["codedeploy.amazonaws.com"]
    }

    actions = ["sts:AssumeRole"]
  }
}

resource "aws_iam_role" "deploy_application_role" {
  name                = "deploy_application_role"
  assume_role_policy  = data.aws_iam_policy_document.deploy_application_role_assume_policy.json
  managed_policy_arns = ["arn:aws:iam::aws:policy/service-role/AWSCodeDeployRole"]
}

resource "aws_codedeploy_app" "tegmen" {
  compute_platform = "Server"
  name             = "tegmen"
}

resource "aws_codedeploy_deployment_group" "tegmen_deployment_group" {
  app_name              = aws_codedeploy_app.tegmen.name
  deployment_group_name = "tegmen"
  service_role_arn      = aws_iam_role.deploy_application_role.arn

  on_premises_instance_tag_filter {
    key   = "tegmen"
    type  = "KEY_AND_VALUE"
    value = "tegmen"
  }
}

resource "aws_iam_user" "master_bedroom" {
  name = "master_bedroom"
}

resource "aws_iam_user_policy_attachment" "master_bedroom_user_policy" {
  user       = aws_iam_user.master_bedroom.name
  policy_arn = "arn:aws:iam::aws:policy/AWSCodeDeployFullAccess"
}

resource "aws_iam_user" "guest_bedroom" {
  name = "guest_bedroom"
}

resource "aws_iam_user_policy_attachment" "guest_bedroom_user_policy" {
  user       = aws_iam_user.guest_bedroom.name
  policy_arn = "arn:aws:iam::aws:policy/AWSCodeDeployFullAccess"
}
