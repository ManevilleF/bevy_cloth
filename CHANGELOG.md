# Changelog

# Unreleased

## Added

* [bevy_rapier](https://github.com/dimforge/bevy_rapier) collision support:
  * Added `rapier_collisions` feature
  * Added `rapier_collision` example
  * Added `ClothCollider` component
* Added `AccelerationSmoothing` enum, defining gravity/winds acceleration smoothing
  * Added related `acceleration_smoothing` field to `ClothConfig`

## API changes

* (**BREAKING**) Renamed `ClothBuilder::fixed_points` to `anchored_vertex_ids`
  * Added `ClothBuilder::with_pinned_vertex_ids` method
  * Added `ClothBuilder::with_anchored_vertex_ids` method
  * Added `ClothBuilder::with_anchored_vertex_id` method
  * Deprecated `ClothBuilder::with_fixed_points` in favor of `ClothBuilder::with_pinned_vertex_ids`
* Added `ClothBuilder::anchored_vertex_colors` field:
  * Added `ClothBuilder::with_pinned_vertex_colors` method
  * Added `ClothBuilder::with_anchored_vertex_colors` method
  * Added `ClothBuilder::with_anchored_vertex_color` method
* Added `ClothBuilder::with_flat_normals` method
  * Deprecated `ClothBuilder::with_flat_normal_computation` in favor of `ClothBuilder::with_flat_normals`
* Added `ClothBuilder::with_smooth_normals` method
  * Deprecated `ClothBuilder::with_smooth_normal_computation` in favor of `ClothBuilder::with_smooth_normals`

## Examples

* Added `rapier_collisions` example
* Added `anchors` example

# 0.1.0

First version