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
