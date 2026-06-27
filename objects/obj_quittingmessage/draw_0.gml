if obj_time.quit>0 then draw_sprite_ext(sprite_index,image_index,view_xview[view_current],view_yview[view_current],1,1,0,c_white,image_alpha) else instance_destroy()
if image_alpha<0.9 then image_alpha+=0.1
