---
title: "getPreviousAdjacentEdge"
excerpt: "Get the previous adjacent edge to the edge given."
layout: manual
---

Get the previous adjacent edge to the edge given.



```js
getPreviousAdjacentEdge(tag: String, extrude_group: ExtrudeGroup) -> Uuid
```

### Examples

```js
const part001 = startSketchOn('XY')
  |> startProfileAt([0, 0], %)
  |> line({ to: [0, 10], tag: "thing" }, %)
  |> line({ to: [10, 0], tag: "thing1" }, %)
  |> line({ to: [0, -10], tag: "thing2" }, %)
  |> close(%)
  |> extrude(10, %)
  |> fillet({
       radius: 2,
       tags: [getPreviousAdjacentEdge("thing2", %)]
     }, %)
```

### Arguments

* `tag`: `String` (REQUIRED)
* `extrude_group`: `ExtrudeGroup` - An extrude group is a collection of extrude surfaces. (REQUIRED)
```js
{
	// The id of the extrusion end cap
	endCapId: uuid,
	// The height of the extrude group.
	height: number,
	// The id of the extrude group.
	id: uuid,
	// The position of the extrude group.
	position: [number, number, number],
	// The rotation of the extrude group.
	rotation: [number, number, number, number],
	// The sketch group paths.
	sketchGroupValues: [{
	// The from point.
	from: [number, number],
	// The name of the path.
	name: string,
	// The to point.
	to: [number, number],
	type: "ToPoint",
} |
{
	// arc's direction
	ccw: string,
	// the arc's center
	center: [number, number],
	// The from point.
	from: [number, number],
	// The name of the path.
	name: string,
	// The to point.
	to: [number, number],
	type: "TangentialArcTo",
} |
{
	// The from point.
	from: [number, number],
	// The name of the path.
	name: string,
	// The to point.
	to: [number, number],
	type: "TangentialArc",
} |
{
	// The from point.
	from: [number, number],
	// The name of the path.
	name: string,
	// The to point.
	to: [number, number],
	type: "Horizontal",
	// The x coordinate.
	x: number,
} |
{
	// The from point.
	from: [number, number],
	// The name of the path.
	name: string,
	// The to point.
	to: [number, number],
	type: "AngledLineTo",
	// The x coordinate.
	x: number,
	// The y coordinate.
	y: number,
} |
{
	// The from point.
	from: [number, number],
	// The name of the path.
	name: string,
	// The to point.
	to: [number, number],
	type: "Base",
}],
	// The id of the extrusion start cap
	startCapId: uuid,
	// The extrude surfaces.
	value: [{
	// The face id for the extrude plane.
	faceId: uuid,
	// The id of the geometry.
	id: uuid,
	// The name.
	name: string,
	// The position.
	position: [number, number, number],
	// The rotation.
	rotation: [number, number, number, number],
	// The source range.
	sourceRange: [number, number],
	type: "extrudePlane",
} |
{
	// The face id for the extrude plane.
	faceId: uuid,
	// The id of the geometry.
	id: uuid,
	// The name.
	name: string,
	// The position.
	position: [number, number, number],
	// The rotation.
	rotation: [number, number, number, number],
	// The source range.
	sourceRange: [number, number],
	type: "extrudeArc",
}],
	// The x-axis of the extrude group base plane in the 3D space
	xAxis: {
	x: number,
	y: number,
	z: number,
},
	// The y-axis of the extrude group base plane in the 3D space
	yAxis: {
	x: number,
	y: number,
	z: number,
},
	// The z-axis of the extrude group base plane in the 3D space
	zAxis: {
	x: number,
	y: number,
	z: number,
},
}
```

### Returns

`Uuid`


