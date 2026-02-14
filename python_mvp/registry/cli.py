from __future__ import annotations

import argparse
from typing import List, Optional

from registry.services import Registry
from registry.seed import seed_mock_data

import json
from pathlib import Path

DATA_FILE = Path("mesh_registry.json")


def print_header(title: str) -> None:
	print("\n" + "=" * 80)
	print(title.center(80))
	print("=" * 80)


def print_teams(registry: Registry) -> None:
	print_header("Teams")
	for t in registry.list_teams():
		print(f"[{t.teams_id}] {t.name} (created {t.created_at:%Y-%m-%d})")


def print_products(registry: Registry) -> None:
	print_header("Data products")
	for p in registry.list_products():
		team = registry.get_team(p.owner_team_id)
		owner = team.name if team else f"team:{p.owner_team_id}"
		print(f"[{p.product_id}] {p.name} (owner={owner})")
		# print(
		# 	f"[{p.product_id}] {p.name} | owner={owner} | format={p.data_format} | "
		# 	f"status={p.status} | class={p.classification}"
		# )


def print_product_details(registry: Registry, product_id: int) -> None:
	product = registry.get_product(product_id)
	if not product:
		print(f"No data product with id {product_id} found.")
		return

	team = registry.get_team(product.owner_team_id)
	owner = team.name if team else f"team:{product.owner_team_id}"

	print_header(f"Data product: {product.name}")
	print(f"ID          : {product.product_id}")
	print(f"Description : {product.description}")
	print(f"Owner team  : {owner}")
	print(f"Format      : {product.data_format}")
	print(f"Access URI  : {product.access_uri}")
	print(f"Status      : {product.status}")
	print(f"Classif.    : {product.classification}")
	print(f"Created     : {product.created_at:%Y-%m-%d %H:%M}")
	print(f"Updated     : {product.updated_at:%Y-%m-%d %H:%M}")

	if not product.metadata:
		print("\nNo metadata entries.")
		return

	print("\nMetadata:")
	for m in product.metadata:
		print(
			f"- {m.namespace}.{m.meta_key} = {m.meta_value} "
			f"(type={m.value_type})"
		)
  
	print()


def add_product_interactively(registry: Registry) -> None:
	print_header("Add new data product")
	name = input("Name: ").strip()
	description = input("Description: ").strip()
	data_format = input("Data format (e.g. parquet, delta): ").strip() or "parquet"
	access_uri = input("Access URI: ").strip()
	status = input("Status (active, deprecated, draft): ").strip() or "draft"
	classification = (
		input("Classification (internal/restricted/public): ").strip() or "internal"
	)

	print("\nAvailable teams:")
	for team in registry.list_teams():
		print(f"[{team.teams_id}] {team.name}")

	while True:
		raw = input("Owner team id: ").strip()
		try:
			owner_team_id = int(raw)
		except ValueError:
			print("Please enter a numeric team id.")
			continue
		if registry.get_team(owner_team_id) is None:
			print("Unknown team id, try again.")
			continue
		break

	product = registry.create_data_product(
		name=name,
		description=description,
		owner_team_id=owner_team_id,
		data_format=data_format,
		access_uri=access_uri,
		status=status,
		classification=classification,
	)

	print(f"\nCreated data product {product.name} with id {product.product_id}.")


def search_products(registry: Registry) -> None:
	term = input("Search term in product name: ").strip()
	results = registry.search_products_by_name(term)
	if not results:
		print("No products found.")
		return
	print_header(f"Search results for '{term}'")
	for p in results:
		team = registry.get_team(p.owner_team_id)
		owner = team.name if team else f"team:{p.owner_team_id}"
		print(f"[{p.product_id}] {p.name} (owner={owner})")


def build_parser() -> argparse.ArgumentParser:
	"""Create the top-level argument parser for the `dcat` executable."""

	parser = argparse.ArgumentParser(
		prog="mesh",
		description="Data registry CLI (mock MVP)",
	)
	subparsers = parser.add_subparsers(dest="command", required=True)

	# dcat teams
	subparsers.add_parser("teams", help="List teams")

	# dcat products
	subparsers.add_parser("products", help="List data products")

	# dcat product <id>
	product_parser = subparsers.add_parser(
		"product",
		help="Show a data product by id",
	)
	product_parser.add_argument("id", type=int, help="Data product id")
 
	# dcat search <term>
	search_parser = subparsers.add_parser(
		"search",
		help="Search data products by name",
	)
	search_parser.add_argument("term", help="Search term in product name")

	# dcat add-product  (still interactive for now)
	subparsers.add_parser(
		"add-product",
		help="Interactively add a new data product",
	)

	return parser





def run(argv: Optional[List[str]] = None) -> None:
    parser = build_parser()
    args = parser.parse_args(argv)

    registry = Registry()

    # ----------------------
    # LOAD EXISTING STATE
    # ----------------------
    if DATA_FILE.exists():
        with DATA_FILE.open("r") as f:
            data = json.load(f)

        for team in data.get("teams", []):
            registry.create_team(team["name"])

        for product in data.get("products", []):
            registry.create_data_product(
                name=product["name"],
                description=product["description"],
                owner_team_id=product["owner_team_id"],
                data_format=product["data_format"],
                access_uri=product["access_uri"],
                status=product["status"],
                classification=product["classification"],
            )
    else:
        seed_mock_data(registry)

    # ----------------------
    # EXECUTE COMMAND
    # ----------------------
    cmd = args.command

    if cmd == "teams":
        print_teams(registry)
    elif cmd == "products":
        print_products(registry)
    elif cmd == "product":
        print_product_details(registry, args.id)
    elif cmd == "search":
        results = registry.search_products_by_name(args.term)
        if not results:
            print(f"No products found for term '{args.term}'.")
        else:
            print_header(f"Search results for '{args.term}'")
            for p in results:
                team = registry.get_team(p.owner_team_id)
                owner = team.name if team else f"team:{p.owner_team_id}"
                print(f"[{p.product_id}] {p.name} (owner={owner})")
    elif cmd == "add-product":
        add_product_interactively(registry)
    else:
        parser.error(f"Unknown mesh subcommand: {cmd}")

    # ----------------------
    # SAVE STATE
    # ----------------------
    data = {
        "teams": [{"name": t.name} for t in registry.list_teams()],
        "products": [
            {
                "name": p.name,
                "description": p.description,
                "owner_team_id": p.owner_team_id,
                "data_format": p.data_format,
                "access_uri": p.access_uri,
                "status": p.status,
                "classification": p.classification,
            }
            for p in registry.list_products()
        ],
    }

    with DATA_FILE.open("w") as f:
        json.dump(data, f, indent=2)



def main() -> None:
	"""Compatibility wrapper for `python cli.py` / `python main.py`.

	This just forwards to `run()` which implements the Unix-style CLI.
	"""
	run()


if __name__ == "__main__":
	main()
