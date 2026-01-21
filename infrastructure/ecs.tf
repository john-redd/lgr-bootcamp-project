resource "aws_ecs_cluster" "main" {
  count = var.enable_ecs ? 1 : 0
  name  = "lgr-bootcamp-cluster"
}

resource "aws_cloudwatch_log_group" "ecs_logs" {
  count             = var.enable_ecs ? 1 : 0
  name              = "/ecs/lgr-bootcamp"
  retention_in_days = 7
}

resource "aws_service_discovery_private_dns_namespace" "main" {
  count       = var.enable_ecs ? 1 : 0
  name        = "local"
  description = "Service discovery namespace for local services"
  vpc         = module.vpc.vpc_id
}

resource "aws_service_discovery_service" "auth_service" {
  count = var.enable_ecs ? 1 : 0
  name  = "auth-service"

  dns_config {
    namespace_id = aws_service_discovery_private_dns_namespace.main[0].id

    dns_records {
      ttl  = 10
      type = "A"
    }
  }

  health_check_custom_config {
    failure_threshold = 1
  }
}

resource "aws_service_discovery_service" "app_service" {
  count = var.enable_ecs ? 1 : 0
  name  = "app-service"

  dns_config {
    namespace_id = aws_service_discovery_private_dns_namespace.main[0].id

    dns_records {
      ttl  = 10
      type = "A"
    }
  }

  health_check_custom_config {
    failure_threshold = 1
  }
}

# --- IAM Roles ---

resource "aws_iam_role" "ecs_task_execution_role" {
  count = var.enable_ecs ? 1 : 0
  name  = "ecs-task-execution-role"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action = "sts:AssumeRole"
        Effect = "Allow"
        Principal = {
          Service = "ecs-tasks.amazonaws.com"
        }
      }
    ]
  })
}

resource "aws_iam_role_policy_attachment" "ecs_task_execution_role_policy" {
  count      = var.enable_ecs ? 1 : 0
  role       = aws_iam_role.ecs_task_execution_role[0].name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AmazonECSTaskExecutionRolePolicy"
}

resource "aws_iam_role" "ecs_task_role" {
  count = var.enable_ecs ? 1 : 0
  name  = "ecs-task-role"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action = "sts:AssumeRole"
        Effect = "Allow"
        Principal = {
          Service = "ecs-tasks.amazonaws.com"
        }
      }
    ]
  })
}

# --- Task Definitions ---

resource "aws_ecs_task_definition" "auth_service" {
  count                    = var.enable_ecs ? 1 : 0
  family                   = "auth-service"
  network_mode             = "awsvpc"
  requires_compatibilities = ["FARGATE"]
  cpu                      = 256
  memory                   = 512
  execution_role_arn       = aws_iam_role.ecs_task_execution_role[0].arn
  task_role_arn            = aws_iam_role.ecs_task_role[0].arn

  container_definitions = jsonencode([
    {
      name      = "auth-service"
      image     = aws_ecr_repository.auth_service_repo.repository_url
      essential = true
      portMappings = [
        {
          containerPort = 3000
          hostPort      = 3000
          protocol      = "tcp"
        }
      ]
      logConfiguration = {
        logDriver = "awslogs"
        options = {
          "awslogs-group"         = aws_cloudwatch_log_group.ecs_logs[0].name
          "awslogs-region"        = var.aws_region
          "awslogs-stream-prefix" = "auth-service"
        }
      }
    }
  ])
}

resource "aws_ecs_task_definition" "app_service" {
  count                    = var.enable_ecs ? 1 : 0
  family                   = "app-service"
  network_mode             = "awsvpc"
  requires_compatibilities = ["FARGATE"]
  cpu                      = 256
  memory                   = 512
  execution_role_arn       = aws_iam_role.ecs_task_execution_role[0].arn
  task_role_arn            = aws_iam_role.ecs_task_role[0].arn

  container_definitions = jsonencode([
    {
      name      = "app-service"
      image     = aws_ecr_repository.app_service_repo.repository_url
      essential = true
      portMappings = [
        {
          containerPort = 8000
          hostPort      = 8000
          protocol      = "tcp"
        }
      ]
      environment = [
        {
          name  = "AUTH_SERVICE_IP"
          value = aws_lb.main[0].dns_name
        },
        {
          name  = "AUTH_SERVICE_HOST_NAME"
          value = "auth-service.local"
        }
      ]
      logConfiguration = {
        logDriver = "awslogs"
        options = {
          "awslogs-group"         = aws_cloudwatch_log_group.ecs_logs[0].name
          "awslogs-region"        = var.aws_region
          "awslogs-stream-prefix" = "app-service"
        }
      }
    }
  ])
}

# --- Services ---

resource "aws_ecs_service" "auth_service" {
  count           = var.enable_ecs ? 1 : 0
  name            = "auth-service"
  cluster         = aws_ecs_cluster.main[0].id
  task_definition = aws_ecs_task_definition.auth_service[0].arn
  desired_count   = 1
  launch_type     = "FARGATE"

  network_configuration {
    subnets          = module.vpc.public_subnets
    security_groups  = [aws_security_group.ecs_tasks_sg[0].id]
    assign_public_ip = true
  }

  load_balancer {
    target_group_arn = aws_lb_target_group.auth_service_tg[0].arn
    container_name   = "auth-service"
    container_port   = 3000
  }

  service_registries {
    registry_arn = aws_service_discovery_service.auth_service[0].arn
  }
}

resource "aws_ecs_service" "app_service" {
  count           = var.enable_ecs ? 1 : 0
  name            = "app-service"
  cluster         = aws_ecs_cluster.main[0].id
  task_definition = aws_ecs_task_definition.app_service[0].arn
  desired_count   = 1
  launch_type     = "FARGATE"

  network_configuration {
    subnets          = module.vpc.public_subnets
    security_groups  = [aws_security_group.ecs_tasks_sg[0].id]
    assign_public_ip = true
  }

  load_balancer {
    target_group_arn = aws_lb_target_group.app_service_tg[0].arn
    container_name   = "app-service"
    container_port   = 8000
  }

  service_registries {
    registry_arn = aws_service_discovery_service.app_service[0].arn
  }
}
