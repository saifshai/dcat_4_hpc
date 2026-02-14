# Data Registry CLI (MVP)

Simple in-memory data registry CLI for a data mesh capstone project.

The CLI is exposed as a `mesh` command with a few subcommands for listing and
inspecting mock data products.

## Installation (Windows / PowerShell)

1. Make sure you have Python 3.9+ installed and on your `PATH`.
2. Open PowerShell in the project root (the folder that contains `pyproject.toml`).
3. Create and activate a virtual environment:

```powershell
python -m venv .venv
.venv\Scripts\Activate.ps1
```

## Installation (macOS / Unix / Linux)

1. Make sure you have Python 3.9+ installed:
2. Open a terminal in the project root (the folder that contains pyproject.toml).
3. Create and activate a virtual environment:

```bash
python3 -m venv .venv
source .venv/bin/activate
```

4. Install the package in editable mode so the `mesh` command is created:

```powershell
pip install -e .
```

After this, `mesh` should be available in the activated PowerShell session.

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
mesh <command> [options]
```

Available commands:

- `mesh teams` – list all teams in the mock registry.
- `mesh products` – list all data products.
- `mesh product <id>` – show full details for a single data product by ID.
- `mesh search <term>` – search data products by name containing `<term>`.
- `mesh add-product` – interactively add a new data product via prompts.

## Example usage (PowerShell)

```powershell
# List teams
mesh teams

# List products
mesh products

# Show details for product with ID 1
mesh product 1

# Search for products whose name contains "climate"
mesh search climate

# Interactively add a new data product
mesh add-product
```

