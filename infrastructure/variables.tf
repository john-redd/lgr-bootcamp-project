variable "aws_region" {
  description = "AWS region"
  type        = string
  default     = "us-east-1"
}

variable "ecr_auth_service_repo_name" {
  description = "Name of the ECR repository for the Auth Service"
  type        = string
  default     = "auth-service"
}

variable "ecr_app_service_repo_name" {
  description = "Name of the ECR repository for the App Service"
  type        = string
  default     = "app-service"
}
