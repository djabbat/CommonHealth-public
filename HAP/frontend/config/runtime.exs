import Config

config :hap_frontend, HAPFrontendWeb.Endpoint,
  secret_key_base:
    System.get_env("SECRET_KEY_BASE") ||
      "5zE8SDvJ8CSZsj0C6o2zBQDDZ7Qj00r5RlN/xwK/GiXy6Lv++2vs2QyKv8oasO+l"

config :hap_frontend,
  backend_url: System.fetch_env!("BACKEND_URL")