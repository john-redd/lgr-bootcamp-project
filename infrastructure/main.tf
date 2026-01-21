provider "aws" {
  region = var.aws_region
}

resource "aws_ecr_repository" "auth_service_repo" {
  name                 = var.ecr_auth_service_repo_name
  image_tag_mutability = "MUTABLE"

  image_scanning_configuration {
    scan_on_push = true
  }

  tags = {
    Name = var.ecr_auth_service_repo_name
  }
}

resource "aws_ecr_repository" "app_service_repo" {
  name                 = var.ecr_app_service_repo_name
  image_tag_mutability = "MUTABLE"

  image_scanning_configuration {
    scan_on_push = true
  }

  tags = {
    Name = var.ecr_app_service_repo_name
  }
}

resource "aws_iam_user" "ecr_user" {
  name = var.iam_user_name
}

resource "aws_iam_policy" "ecr_push_pull_policy" {
  name        = "${var.iam_user_name}-policy"
  description = "Policy to allow push and pull images from ECR repositories"

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Sid    = "GetAuthorizationToken"
        Effect = "Allow"
        Action = [
          "ecr:GetAuthorizationToken"
        ]
        Resource = "*"
      },
      {
        Sid    = "AllowPushPull"
        Effect = "Allow"
        Action = [
          "ecr:BatchCheckLayerAvailability",
          "ecr:GetDownloadUrlForLayer",
          "ecr:BatchGetImage",
          "ecr:PutImage",
          "ecr:InitiateLayerUpload",
          "ecr:UploadLayerPart",
          "ecr:CompleteLayerUpload"
        ]
        Resource = [
          aws_ecr_repository.auth_service_repo.arn,
          aws_ecr_repository.app_service_repo.arn
        ]
      },
      {
        Sid    = "AllowUpdateService"
        Effect = "Allow"
        Action = [
          "ecs:UpdateService"
        ]
        Resource = [
          aws_ecs_service.auth_service.id,
          aws_ecs_service.app_service.id
        ]
      }
    ]
  })
}

resource "aws_iam_user_policy_attachment" "ecr_user_policy_attachment" {
  user       = aws_iam_user.ecr_user.name
  policy_arn = aws_iam_policy.ecr_push_pull_policy.arn
}

resource "aws_iam_access_key" "ecr_user_key" {
  user = aws_iam_user.ecr_user.name
}
