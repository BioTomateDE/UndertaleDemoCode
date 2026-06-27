if global.mnfight = 2 then movement = 1
else movement = 0
global.invc -= 1
if global.invc > 0 or obj_battlecontroller.runaway=1 then
{
image_speed = 0.5
}
else
{
image_index = 0
image_speed = 0
}
