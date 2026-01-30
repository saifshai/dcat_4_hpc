from __future__ import annotations

from .services import Registry


def seed_mock_data(registry: Registry) -> None:
	"""Populate the registry with Agriculture Canada–style example data products."""

	# Teams (domains)
	climate_team = registry.create_team("Climate & Environment")
	crops_team = registry.create_team("Crop Analytics")

	# Data products
	climate_obs_dp = registry.create_data_product(
		name="canada_climate_observations_daily",
		description="Daily climate observations aggregated from Environment Canada weather stations.",
		owner_team_id=climate_team.teams_id,
		data_format="parquet",
		access_uri="s3://agriculture-canada/curated/climate_observations_daily/",
		status="active",
		classification="public",
	)

	crop_yield_dp = registry.create_data_product(
		name="crop_yield_forecast_grid",
		description="Grid-level crop yield forecasts derived from historical yield data and climate indicators.",
		owner_team_id=crops_team.teams_id,
		data_format="geoparquet",
		access_uri="s3://agriculture-canada/curated/crop_yield_forecast/",
		status="active",
		classification="internal",
	)

	# Metadata for climate observations
	registry.add_metadata(
		data_product_id=climate_obs_dp.product_id,
		namespace="business",
		meta_key="domain",
		meta_value="climate",
		value_type="string",
	)
	registry.add_metadata(
		data_product_id=climate_obs_dp.product_id,
		namespace="technical",
		meta_key="spatial_resolution",
		meta_value="station-level",
		value_type="string",
	)
	registry.add_metadata(
		data_product_id=climate_obs_dp.product_id,
		namespace="technical",
		meta_key="refresh_schedule",
		meta_value="0 6 * * *",
		value_type="cron",
	)
	registry.add_metadata(
		data_product_id=climate_obs_dp.product_id,
		namespace="governance",
		meta_key="source",
		meta_value="Environment and Climate Change Canada",
		value_type="string",
	)

	# Metadata for crop yield forecast
	registry.add_metadata(
		data_product_id=crop_yield_dp.product_id,
		namespace="business",
		meta_key="domain",
		meta_value="agriculture",
		value_type="string",
	)
	registry.add_metadata(
		data_product_id=crop_yield_dp.product_id,
		namespace="technical",
		meta_key="grid_system",
		meta_value="10km x 10km national grid",
		value_type="string",
	)
	registry.add_metadata(
		data_product_id=crop_yield_dp.product_id,
		namespace="technical",
		meta_key="model_type",
		meta_value="regression + climate indicators",
		value_type="string",
	)
	registry.add_metadata(
		data_product_id=crop_yield_dp.product_id,
		namespace="governance",
		meta_key="intended_use",
		meta_value="research and policy analysis",
		value_type="string",
	)
