# vim mouse (WIP)
This is meant to be a linx (x11) compatible replacement for the karabiner ["Mouse keys Mode v4" rules for Karabiner](https://ke-complex-modifications.pqrs.org/?q=vim%20mouse)

The goal of the program is the following:

**operate mouse via keyboard; (motion, click and scroll)**

## how it should work
mouse keys mode is active by the following procedure:
- d key_down
- any mouse keys key_down (h, j, k, l,...)
- any mouse keys key_up (h, j, k, l,...)
- mouse keys are active until you release d key

example:
- press d
- press h
- press j
- release h
- release j
- release d

## key bindings:
- h: mouse left
- j: mouse down
- k: mouse up
- l: mouse right
- v: left click
- b: middle click
- n: right click
- f: fast mode (fast mouse move by f+hjkl)
- g: slow mode (slow mouse move by g+hjkl)
- s: scroll mode (scroll by s+hjkl)

## completed
- hjkl mouse move
- modifier key blocking

## wip
- remappable keys

## todos
- diagonal movement
- speed setting
- left click
- middle click
- right click
- fast mode
- slow mode
- scroll mode
