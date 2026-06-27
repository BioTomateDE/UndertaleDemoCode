if instance_exists (obj_mainchara)
if obj_mainchara.y > yy+130 then 
{
draw_set_color(c_white)
draw_rectangle(view_xview[view_current]+16,view_yview[view_current]+5,view_xview[view_current]+304,view_yview[view_current]+80,false)

draw_set_color(c_black)
draw_rectangle(view_xview[view_current]+19,view_yview[view_current]+8,view_xview[view_current]+301,view_yview[view_current]+77,false)
}
else
{
draw_set_color(c_white)
draw_rectangle(view_xview[view_current]+16,view_yview[view_current]+160,view_xview[view_current]+304,view_yview[view_current]+235,false)

draw_set_color(c_black)
draw_rectangle(view_xview[view_current]+19,view_yview[view_current]+163,view_xview[view_current]+301,view_yview[view_current]+232,false)
}
