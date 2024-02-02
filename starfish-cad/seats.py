# All units in mm
from solid2 import cube, cylinder, set_global_fn

padding = 0.01

# set number of faces per curved shape
set_global_fn(40)

################
# Raspberry Pi Pico seat
################
rp_w = 20.96
rp_l = 50.8
rp_h = 5 # not sure about this measurement
rp_hull = cube(rp_w, rp_l, rp_h + padding, center=True)

rp_seat_wall_thickness = 4

rp_seat = cube(rp_w + rp_seat_wall_thickness * 2,
               rp_l + rp_seat_wall_thickness * 2,
               rp_h + rp_seat_wall_thickness,
               center = True)
# rp_seat = rp_seat.translate(0, 0, rp_seat_wall_thickness)
rp_seat -= rp_hull.translate(0, 0, rp_seat_wall_thickness)
# rp_seat = rp_seat.translate(0, 0, rp_seat_wall_thickness)
# rp_seat.save_as_scad()

################
# Hall Effect Sensor Seat
################
hall_w = 4
hall_l = 2.7
hall_h = 1.5
hall_hull = cube(hall_w, hall_l, hall_h)

################
# USBC Female Breakout Seat
################
usbc_w = 10
usbc_h = 4
usbc_d = 10
usbc_r = usbc_h / 2
usbc_wall = cylinder(r=usbc_r, h=usbc_d, center=True)
usbc_wall = usbc_wall.rotateX(90)
usbc_hull = cube(usbc_w - usbc_h, usbc_d, usbc_h, center=True)
usbc_hull += usbc_wall.translateX(usbc_w/2 - usbc_r)
usbc_hull += usbc_wall.translateX(-usbc_w/2 + usbc_r)

# usbc_hull.save_as_scad()
