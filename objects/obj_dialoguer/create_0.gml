global.facechange=0
xx=view_xview[view_current]
yy=view_yview[view_current]
//xx=round(xx)
//yy=round(yy)
if instance_exists (obj_mainchara)
if obj_mainchara.y > yy+130 then 
    {
     if global.facechoice != 0 then
     {
     writer = instance_create(xx+68,yy-5,OBJ_WRITER)
     script_execute(scr_facechoice)
     }
    else
     writer = instance_create(xx+10,yy-5,OBJ_WRITER)
    }
  else
    {
    if global.facechoice != 0 then
     {
     writer = instance_create(xx+68,yy+150,OBJ_WRITER)
     script_execute(scr_facechoice)
     }
    else
    writer = instance_create(xx+10,yy+150,OBJ_WRITER)
    }
