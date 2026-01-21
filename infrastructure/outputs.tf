output "iam_access_key_id" {
  description = "The access key ID for the ECR CI/CD user"
  value       = aws_iam_access_key.ecr_user_key.id
}

output "iam_secret_access_key" {
  description = "The secret access key for the ECR CI/CD user"
  value       = aws_iam_access_key.ecr_user_key.secret
  sensitive   = true
}
