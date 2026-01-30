from __future__ import annotations

from datetime import datetime
from typing import Dict, List, Optional

from .models import DataProduct, MetadataEntry, Team


class Registry:
	"""In-memory data registry for the CLI MVP."""

	def __init__(self) -> None:
		self._teams: Dict[int, Team] = {}
		self._products: Dict[int, DataProduct] = {}
		self._metadata: Dict[int, MetadataEntry] = {}
		self._next_team_id = 1
		self._next_product_id = 1
		self._next_metadata_id = 1

	# -------------------- creation helpers --------------------

	def create_team(self, name: str) -> Team:
		team = Team(teams_id=self._next_team_id, name=name, created_at=datetime.utcnow())
		self._teams[team.teams_id] = team
		self._next_team_id += 1
		return team

	def create_data_product(
		self,
		name: str,
		description: str,
		owner_team_id: int,
		data_format: str,
		access_uri: str,
		status: str,
		classification: str,
	) -> DataProduct:
		now = datetime.utcnow()
		product = DataProduct(
			product_id=self._next_product_id,
			name=name,
			description=description,
			owner_team_id=owner_team_id,
			data_format=data_format,
			access_uri=access_uri,
			status=status,
			classification=classification,
			created_at=now,
			updated_at=now,
		)
		self._products[product.product_id] = product
		self._next_product_id += 1
		return product

	def add_metadata(
		self,
		data_product_id: int,
		namespace: str,
		meta_key: str,
		meta_value: str,
		value_type: str,
	) -> MetadataEntry:
		if data_product_id not in self._products:
			raise ValueError(f"Unknown data_product_id {data_product_id}")

		entry = MetadataEntry(
			metadata_id=self._next_metadata_id,
			data_product_id=data_product_id,
			namespace=namespace,
			meta_key=meta_key,
			meta_value=meta_value,
			value_type=value_type,
			created_at=datetime.utcnow(),
		)
		self._metadata[entry.metadata_id] = entry
		self._products[data_product_id].metadata.append(entry)
		self._next_metadata_id += 1
		return entry

	# -------------------- query helpers --------------------

	def list_teams(self) -> List[Team]:
		return list(self._teams.values())

	def list_products(self) -> List[DataProduct]:
		return list(self._products.values())

	def get_team(self, team_id: int) -> Optional[Team]:
		return self._teams.get(team_id)

	def get_product(self, product_id: int) -> Optional[DataProduct]:
		return self._products.get(product_id)

	def search_products_by_name(self, term: str) -> List[DataProduct]:
		term_lower = term.lower()
		return [p for p in self._products.values() if term_lower in p.name.lower()]
