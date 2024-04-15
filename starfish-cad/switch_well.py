from solid2 import cube, polygon
import math

padding = 0.01

################
# Keyboard Constants
################
num_rows = 3
num_cols = 6

################
# Switch Well of grid of seats
################
switch_w = 15
switch_h = 8

switch_margin = 5
switch_seat_thickness = 4
switch_seat_w = switch_w + switch_margin

well_radius_hor = 300
well_radius_vert = 100

switch_seat_points = [
    # outer
    [switch_w / 2, switch_w / 2],
    [switch_w / 2, -switch_w / 2],
    [-switch_w / 2, -switch_w / 2],
    [-switch_w / 2, switch_w / 2],
    # hole
    [(switch_seat_w) / 2, (switch_seat_w) / 2],
    [(switch_seat_w) / 2, -(switch_seat_w) / 2],
    [-(switch_seat_w) / 2, -(switch_seat_w) / 2],
    [-(switch_seat_w) / 2, (switch_seat_w) / 2],
]
switch_seat_paths = [
    [0, 1, 2, 3],
    [4, 5, 6, 7],
]
switch_seat = polygon(switch_seat_points, switch_seat_paths, 5)
# switch_seat = switch_seat.right(switch_seat_w / 2).forward(switch_seat_w / 2)

# Empty shape to start from
well_row = cube().translate(100, 100, 100) & cube()
switch_well = cube().translate(100, 100, 100) & cube()

well_circumference = 2 * math.pi * well_radius_hor
arc_angle_rad = math.asin(switch_seat_w / well_radius_hor)
arc_angle_deg = math.degrees(arc_angle_rad)
for i_col in range(num_cols):
    arc_rotation = arc_angle_deg * i_col
    iter_seat = switch_seat.down(well_radius_hor)
    iter_seat = iter_seat.rotateY(-arc_rotation)
    iter_seat = iter_seat.up(well_radius_hor)
    well_row += iter_seat
well_row = well_row.right(switch_seat_w / 2)
arc_span = arc_angle_rad * (num_cols - 1)
arc_tri_base = well_radius_hor * math.sin(arc_span / 2)
row_angle_offset = math.acos(arc_tri_base / well_radius_hor)
well_row = well_row.rotateY(90 - math.degrees(row_angle_offset))

arc_angle_rad = math.asin(switch_seat_w / well_radius_vert)
arc_angle_deg = arc_angle_rad * 180 / 3.14159
for i_row in range(num_rows):
    arc_rotation = arc_angle_deg * i_row
    iter_row = well_row.down(well_radius_vert)
    iter_row = iter_row.rotateX(arc_rotation)
    iter_row = iter_row.up(well_radius_vert)
    switch_well += iter_row

switch_seat.save_as_scad(filename="switch_seat.scad", outdir="scad")
switch_well.save_as_scad(filename="switch_well.scad", outdir="scad")
