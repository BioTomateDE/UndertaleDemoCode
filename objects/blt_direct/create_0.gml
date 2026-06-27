sprite_index=global.bulletappearance
myspeed = global.bulletvariable[0]
mydirection = global.bulletvariable[6] //direction
mydirectionalspeed = global.bulletvariable[7] //directionalspeed 
myrandomspeed = global.bulletvariable[1]
mydirectionrandom = global.bulletvariable[2]
gravity = global.bulletvariable[3]
gravity_direction = global.bulletvariable[4]
friction = global.bulletvariable[5]

instance_create(x+(sprite_width/2-8),y+(sprite_width/2-8),blt_gen)
dmg = 0
