output "alb_dns_name" {
  description = "The DNS name of the load balancer"
  value       = join("", aws_lb.main[*].dns_name)
}

output "ecs_cluster_name" {
  description = "Name of the ECS cluster"
  value       = join("", aws_ecs_cluster.main[*].name)
}
