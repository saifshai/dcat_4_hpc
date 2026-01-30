# Data Registry CLI (MVP)

Simple in-memory data registry CLI for a data mesh capstone project.

The CLI is exposed as a `dcat` command with a few subcommands for listing and
inspecting mock data products.

## Installation (Windows / PowerShell)

1. Make sure you have Python 3.9+ installed and on your `PATH`.
2. Open PowerShell in the project root (the folder that contains `pyproject.toml`).
3. Create and activate a virtual environment:

```powershell
python -m venv .venv
.venv\Scripts\Activate.ps1
```

4. Install the package in editable mode so the `dcat` command is created:

```powershell
pip install -e .
```

After this, `dcat` should be available in the activated PowerShell session.

> If PowerShell blocks running the activation script, you can temporarily
> relax the execution policy for the current session:
>
> ```powershell
> Set-ExecutionPolicy -Scope Process -ExecutionPolicy Bypass
> .venv\Scripts\Activate.ps1
> ```

## CLI commands

Once installed, the main entry point is:

```powershell
dcat <command> [options]
```

Available commands:

- `dcat teams` – list all teams in the mock registry.
- `dcat products` – list all data products.
- `dcat product <id>` – show full details for a single data product by ID.
- `dcat search <term>` – search data products by name containing `<term>`.
- `dcat add-product` – interactively add a new data product via prompts.

## Example usage (PowerShell)

```powershell
# List teams
dcat teams

# List products
dcat products

# Show details for product with ID 1
dcat product 1

# Search for products whose name contains "climate"
dcat search climate

# Interactively add a new data product
dcat add-product
```

