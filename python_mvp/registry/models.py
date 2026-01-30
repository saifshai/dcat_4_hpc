from __future__ import annotations

from dataclasses import dataclass, field
from datetime import datetime
from typing import List

# Main data models for the registry system (not including the registry itself).

@dataclass
class Team:
	teams_id: int
	name: str
	created_at: datetime


@dataclass
class MetadataEntry:
	metadata_id: int
	data_product_id: int
	namespace: str
	meta_key: str
	meta_value: str
	value_type: str
	created_at: datetime


@dataclass
class DataProduct:
	product_id: int
	name: str
	description: str
	owner_team_id: int
	data_format: str
	access_uri: str
	status: str
	classification: str
	created_at: datetime
	updated_at: datetime
	metadata: List[MetadataEntry] = field(default_factory=list)
