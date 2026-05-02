# Services

The service layer exposes business logic to `mesh_cli`.

```
services/
├── registry_service.rs  # Registry workflows built on repositories
└── mod.rs               # Service module exports
```

Services coordinate repositories and hide persistence details from `mesh_cli`.

Keep CLI-facing workflow logic here, not in repositories or raw database code.
